//! The domain as code generation sees it: types in numbering order,
//! predicates and actions over typed variables, all names resolved.

use std::collections::{HashMap, HashSet};

use pddl_core::{
    ActionDefinition, AtomicFormula, ConditionalEffect, Domain, EffectCondition, Effects,
    GoalDescription, Literal, PreconditionGoalDescription, PreconditionGoalDescriptions,
    PreferenceGoalDescription, PrimitiveEffect, Requirement, StructureDef, Term, Type,
    TypedVariables,
};

use crate::Error;

/// Lowercases a name.
pub fn lc(name: &str) -> String {
    name.to_lowercase()
}

/// A PDDL type.
pub struct TypeInfo {
    /// The name, lowercase.
    pub name: String,
    /// The supertype; `None` for `object`.
    pub parent: Option<usize>,
}

/// A predicate.
pub struct PredicateInfo {
    /// The name, lowercase.
    pub name: String,
    /// The type of each argument; `either` types are `object` (0).
    pub args: Vec<usize>,
    /// Whether no action changes it, so its facts are fixed by the problem.
    pub is_static: bool,
}

/// A term in a formula: a variable slot or a constant.
#[derive(Clone, Copy)]
pub enum Arg {
    /// The variable in this slot of the action's scope.
    Var(usize),
    /// The constant with this index.
    Const(usize),
}

/// A condition, compiled to a boolean expression.
pub enum Cond {
    /// A fact.
    Atom {
        /// The predicate index.
        pred: usize,
        /// The arguments.
        args: Vec<Arg>,
    },
    /// Two terms naming the same object.
    Eq(Arg, Arg),
    /// The negation.
    Not(Box<Cond>),
    /// The conjunction.
    And(Vec<Cond>),
    /// The disjunction.
    Or(Vec<Cond>),
    /// Some objects for the variables in these slots satisfy the body.
    Exists(Vec<usize>, Box<Cond>),
    /// All objects for the variables in these slots satisfy the body.
    Forall(Vec<usize>, Box<Cond>),
}

/// An effect.
pub enum Effect {
    /// Adds (`add`) or deletes a fact.
    Fact {
        /// The predicate index.
        pred: usize,
        /// The arguments.
        args: Vec<Arg>,
        /// Whether the fact is added rather than deleted.
        add: bool,
    },
    /// The effects for every object of the variables in these slots.
    Forall(Vec<usize>, Vec<Effect>),
    /// The effects, where the condition holds in the state before the action.
    When(Cond, Vec<Effect>),
}

impl Effect {
    /// Calls `f` with every fact effect: predicate and whether it adds.
    pub fn facts(&self, f: &mut impl FnMut(usize, bool)) {
        match self {
            Self::Fact { pred, add, .. } => f(*pred, *add),
            Self::Forall(_, body) | Self::When(_, body) => {
                for effect in body {
                    effect.facts(f);
                }
            }
        }
    }
}

/// A variable slot: an action parameter.
pub struct Slot {
    /// The PDDL name, lowercase, without `?`.
    pub name: String,
    /// The type, or `object` for an `either` type.
    pub ty: usize,
    /// For an `either` type, its members; the slot only takes their objects.
    pub either: Option<Vec<usize>>,
}

/// An action schema.
pub struct ActionInfo {
    /// The name, lowercase.
    pub name: String,
    /// The variable slots: the first `arity` are the parameters, the others
    /// are bound by quantifiers.
    pub slots: Vec<Slot>,
    /// The number of parameters.
    pub arity: usize,
    /// The precondition, as a conjunction.
    pub pre: Vec<Cond>,
    /// The effects.
    pub effects: Vec<Effect>,
}

/// A domain, resolved for code generation.
pub struct Model {
    /// The domain's name, lowercase.
    pub name: String,
    /// The types, each before its subtypes, each subtree contiguous; `object` first.
    pub types: Vec<TypeInfo>,
    /// The predicates, in declaration order.
    pub predicates: Vec<PredicateInfo>,
    /// The constants with their types.
    pub constants: Vec<(String, usize)>,
    /// The actions, in declaration order.
    pub actions: Vec<ActionInfo>,
}

/// The requirements generated code supports.
const SUPPORTED: &[Requirement] = &[
    Requirement::Strips,
    Requirement::Typing,
    Requirement::NegativePreconditions,
    Requirement::Equality,
    Requirement::DisjunctivePreconditions,
    Requirement::ExistentialPreconditions,
    Requirement::UniversalPreconditions,
    Requirement::QuantifiedPreconditions,
    Requirement::ConditionalEffects,
    Requirement::Adl,
];

/// Returns an error for an unsupported construct.
fn unsupported<T>(what: &str) -> Result<T, Error> {
    Err(Error::new(format!("unsupported: {what}")))
}

impl Model {
    /// Resolves a domain.
    pub fn new(domain: &Domain) -> Result<Self, Error> {
        check_requirements(domain)?;
        if !domain.extends().is_empty() {
            return unsupported("domain extension (:extends)");
        }
        if !domain.constraints().is_empty() {
            return unsupported("constraints");
        }
        if !domain.timeless().is_empty() {
            return unsupported("timeless literals");
        }
        if !domain.functions().values().is_empty() {
            return unsupported("numeric fluents");
        }

        let types = types(domain)?;
        let type_index = |ty: &Type| resolve_type(&types, ty);

        let mut predicates = Vec::new();
        for skeleton in domain.predicates().iter() {
            let name = lc(skeleton.name());
            if predicates.iter().any(|p: &PredicateInfo| p.name == name) {
                return Err(Error::new(format!("predicate `{name}` is declared twice")));
            }
            let args = skeleton
                .variables()
                .iter()
                .map(|v| type_index(v.type_()).map(|(t, _)| t))
                .collect::<Result<_, _>>()?;
            predicates.push(PredicateInfo {
                name,
                args,
                is_static: true,
            });
        }

        let mut constants = Vec::new();
        for typed in domain.constants().iter() {
            let (ty, either) = type_index(typed.type_())?;
            if either.is_some() {
                return unsupported("constants with an `either` type");
            }
            constants.push((lc(typed.value()), ty));
        }

        let mut model = Self {
            name: lc(domain.name()),
            types,
            predicates,
            constants,
            actions: Vec::new(),
        };
        for def in domain.structure().iter() {
            match def {
                StructureDef::Action(action) => {
                    let action = Scope::new(&model).action(action)?;
                    model.actions.push(action);
                }
                StructureDef::DurativeAction(_) => return unsupported("durative actions"),
                StructureDef::Derived(_) => return unsupported("derived predicates"),
            }
        }
        let mut changed = HashSet::new();
        for effect in model.actions.iter().flat_map(|a| &a.effects) {
            effect.facts(&mut |pred, _| {
                changed.insert(pred);
            });
        }
        for (i, pred) in model.predicates.iter_mut().enumerate() {
            pred.is_static = !changed.contains(&i);
        }
        Ok(model)
    }

    /// Whether type `sub` is `sup` or one of its subtypes.
    pub fn is_subtype(&self, mut sub: usize, sup: usize) -> bool {
        loop {
            if sub == sup {
                return true;
            }
            match self.types[sub].parent {
                Some(parent) => sub = parent,
                None => return false,
            }
        }
    }
}

/// Resolves a type: its index, and for an `either` of several types, `object`
/// with the members.
fn resolve_type(types: &[TypeInfo], ty: &Type) -> Result<(usize, Option<Vec<usize>>), Error> {
    let find = |name: &str| {
        let name = lc(name);
        types
            .iter()
            .position(|t| t.name == name)
            .ok_or_else(|| Error::new(format!("unknown type `{name}`")))
    };
    match ty {
        Type::Exactly(name) => Ok((find(name)?, None)),
        Type::EitherOf(names) => {
            let members = names
                .iter()
                .map(|n| find(n))
                .collect::<Result<Vec<_>, _>>()?;
            match members.as_slice() {
                [single] => Ok((*single, None)),
                _ => Ok((0, Some(members))),
            }
        }
    }
}

/// Rejects requirements that generated code does not support.
fn check_requirements(domain: &Domain) -> Result<(), Error> {
    if let Some(unknown) = domain.requirements().unknown().first() {
        return Err(Error::new(format!("unknown requirement: {unknown}")));
    }
    let mut found: Vec<Requirement> = domain
        .requirements()
        .to_effective()
        .into_iter()
        .filter(|r| !SUPPORTED.contains(r))
        .collect();
    found.sort_by_key(ToString::to_string);
    match found.first() {
        Some(requirement) => Err(Error::new(format!(
            "unsupported requirement: {requirement}"
        ))),
        None => Ok(()),
    }
}

/// The types in depth-first order from `object`, children in declaration order.
fn types(domain: &Domain) -> Result<Vec<TypeInfo>, Error> {
    let mut children: Vec<(String, String)> = Vec::new();
    let mut declared: Vec<String> = vec!["object".to_string()];
    for typed in domain.types().iter() {
        let name = lc(typed.value());
        let parent = match typed.type_() {
            Type::Exactly(parent) => lc(parent),
            Type::EitherOf(parents) if parents.len() == 1 => lc(&parents[0]),
            Type::EitherOf(_) => return unsupported("types with more than one supertype"),
        };
        if name == "object" {
            continue;
        }
        if declared.contains(&name) {
            return Err(Error::new(format!("type `{name}` is declared twice")));
        }
        declared.push(name.clone());
        children.push((parent, name));
    }
    for (parent, _) in &children {
        if !declared.contains(parent) {
            return Err(Error::new(format!("unknown type `{parent}`")));
        }
    }

    let mut out = Vec::new();
    let mut stack = vec![("object".to_string(), None)];
    while let Some((name, parent)) = stack.pop() {
        let index = out.len();
        out.push(TypeInfo { name, parent });
        let name = &out[index].name;
        // Reverse, so the first child is visited first.
        for (_, child) in children.iter().rev().filter(|(p, _)| p == name) {
            stack.push((child.clone(), Some(index)));
        }
    }
    if out.len() != declared.len() {
        return Err(Error::new("the type hierarchy has a cycle"));
    }
    Ok(out)
}

/// Variables that a quantifier shadows, with the slots they had.
type Shadowed = Vec<(String, Option<usize>)>;

/// Resolves the formulas of one action.
struct Scope<'a> {
    /// The domain so far.
    model: &'a Model,
    /// The variable slots.
    slots: Vec<Slot>,
    /// The slot of each variable in scope.
    vars: HashMap<String, usize>,
}

impl<'a> Scope<'a> {
    /// An empty scope.
    fn new(model: &'a Model) -> Self {
        Self {
            model,
            slots: Vec::new(),
            vars: HashMap::new(),
        }
    }

    /// Adds slots for `vars` and brings them into scope. Returns the slots
    /// and the variables they shadow, for [`unbind`](Self::unbind).
    fn bind(&mut self, vars: &TypedVariables) -> Result<(Vec<usize>, Shadowed), Error> {
        let mut slots = Vec::new();
        let mut shadowed = Vec::new();
        for var in vars.iter() {
            let (ty, either) = resolve_type(&self.model.types, var.type_())?;
            let name = lc(var.value());
            let slot = self.slots.len();
            shadowed.push((name.clone(), self.vars.insert(name.clone(), slot)));
            self.slots.push(Slot { name, ty, either });
            slots.push(slot);
        }
        Ok((slots, shadowed))
    }

    /// Takes variables out of scope again, restoring the ones they shadowed.
    fn unbind(&mut self, shadowed: Shadowed) {
        for (name, old) in shadowed.into_iter().rev() {
            match old {
                Some(slot) => self.vars.insert(name, slot),
                None => self.vars.remove(&name),
            };
        }
    }

    /// Resolves an action.
    fn action(mut self, action: &ActionDefinition) -> Result<ActionInfo, Error> {
        let name = lc(action.symbol());
        self.bind(action.parameters())
            .map_err(|e| e.context(&name))?;
        let arity = self.slots.len();
        let mut pre = Vec::new();
        self.preconditions(action.precondition(), &mut pre)
            .map_err(|e| e.context(&name))?;
        let mut effects = Vec::new();
        if let Some(effect) = action.effect() {
            self.effects(effect, &mut effects)
                .map_err(|e| e.context(&name))?;
        }
        Ok(ActionInfo {
            name,
            slots: self.slots,
            arity,
            pre,
            effects,
        })
    }

    /// Resolves a term.
    fn arg(&self, term: &Term) -> Result<Arg, Error> {
        match term {
            Term::Variable(var) => {
                let name = lc(var);
                self.vars
                    .get(&name)
                    .map(|&slot| Arg::Var(slot))
                    .ok_or_else(|| Error::new(format!("unknown variable `?{name}`")))
            }
            Term::Name(name) => {
                let name = lc(name);
                self.model
                    .constants
                    .iter()
                    .position(|(c, _)| *c == name)
                    .map(Arg::Const)
                    .ok_or_else(|| Error::new(format!("unknown constant `{name}`")))
            }
            Term::Function(_) => unsupported("object fluents"),
        }
    }

    /// Resolves an atom's predicate and arguments.
    fn atom(&self, formula: &AtomicFormula<Term>) -> Result<(usize, Vec<Arg>), Error> {
        let AtomicFormula::Predicate(atom) = formula else {
            return unsupported("equality outside a condition");
        };
        let name = lc(atom.predicate());
        let pred = self
            .model
            .predicates
            .iter()
            .position(|p| p.name == name)
            .ok_or_else(|| Error::new(format!("unknown predicate `{name}`")))?;
        if self.model.predicates[pred].args.len() != atom.values().len() {
            return Err(Error::new(format!(
                "`{name}` takes {} arguments, not {}",
                self.model.predicates[pred].args.len(),
                atom.values().len()
            )));
        }
        let args = atom
            .values()
            .iter()
            .map(|t| self.arg(t))
            .collect::<Result<_, _>>()?;
        Ok((pred, args))
    }

    /// Resolves an atom or equality as a condition.
    fn literal(&self, formula: &AtomicFormula<Term>) -> Result<Cond, Error> {
        match formula {
            AtomicFormula::Equality(eq) => {
                Ok(Cond::Eq(self.arg(eq.first())?, self.arg(eq.second())?))
            }
            AtomicFormula::Predicate(_) => {
                let (pred, args) = self.atom(formula)?;
                Ok(Cond::Atom { pred, args })
            }
        }
    }

    /// Flattens a precondition into conjuncts.
    fn preconditions(
        &mut self,
        pres: &PreconditionGoalDescriptions,
        out: &mut Vec<Cond>,
    ) -> Result<(), Error> {
        for pre in pres {
            match pre {
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(gd)) => {
                    self.conjuncts(gd, out)?;
                }
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Preference(
                    _,
                )) => return unsupported("preferences"),
                PreconditionGoalDescription::Forall(vars, inner) => {
                    let (slots, shadowed) = self.bind(vars)?;
                    let mut body = Vec::new();
                    self.preconditions(inner, &mut body)?;
                    self.unbind(shadowed);
                    out.push(Cond::Forall(slots, Box::new(Cond::And(body))));
                }
            }
        }
        Ok(())
    }

    /// Flattens a goal description into conjuncts.
    fn conjuncts(&mut self, gd: &GoalDescription, out: &mut Vec<Cond>) -> Result<(), Error> {
        if let GoalDescription::And(parts) = gd {
            for part in parts {
                self.conjuncts(part, out)?;
            }
        } else {
            out.push(self.cond(gd)?);
        }
        Ok(())
    }

    /// Resolves a goal description.
    fn cond(&mut self, gd: &GoalDescription) -> Result<Cond, Error> {
        match gd {
            GoalDescription::AtomicFormula(formula)
            | GoalDescription::Literal(Literal::AtomicFormula(formula)) => self.literal(formula),
            GoalDescription::Literal(Literal::NotAtomicFormula(formula)) => {
                Ok(Cond::Not(Box::new(self.literal(formula)?)))
            }
            GoalDescription::Not(inner) => Ok(Cond::Not(Box::new(self.cond(inner)?))),
            GoalDescription::And(parts) => Ok(Cond::And(
                parts
                    .iter()
                    .map(|p| self.cond(p))
                    .collect::<Result<_, _>>()?,
            )),
            GoalDescription::Or(parts) => Ok(Cond::Or(
                parts
                    .iter()
                    .map(|p| self.cond(p))
                    .collect::<Result<_, _>>()?,
            )),
            GoalDescription::Imply(a, b) => Ok(Cond::Or(vec![
                Cond::Not(Box::new(self.cond(a)?)),
                self.cond(b)?,
            ])),
            GoalDescription::Exists(vars, inner) | GoalDescription::ForAll(vars, inner) => {
                let (slots, shadowed) = self.bind(vars)?;
                let body = self.cond(inner);
                self.unbind(shadowed);
                let body = Box::new(body?);
                Ok(if matches!(gd, GoalDescription::Exists(..)) {
                    Cond::Exists(slots, body)
                } else {
                    Cond::Forall(slots, body)
                })
            }
            GoalDescription::FluentComparison(_) => unsupported("numeric fluents"),
        }
    }

    /// Resolves effects.
    fn effects(&mut self, effects: &Effects, out: &mut Vec<Effect>) -> Result<(), Error> {
        for effect in effects {
            match effect {
                ConditionalEffect::Effect(primitive) => self.primitive(primitive, out)?,
                ConditionalEffect::Forall(forall) => {
                    let (slots, shadowed) = self.bind(&forall.variables)?;
                    let mut body = Vec::new();
                    let result = self.effects(&forall.effects, &mut body);
                    self.unbind(shadowed);
                    result?;
                    out.push(Effect::Forall(slots, body));
                }
                ConditionalEffect::When(when) => {
                    let cond = self.cond(&when.condition)?;
                    let mut body = Vec::new();
                    let primitives = match &when.effect {
                        EffectCondition::Single(effect) => std::slice::from_ref(effect),
                        EffectCondition::All(effects) => effects.as_slice(),
                    };
                    for primitive in primitives {
                        self.primitive(primitive, &mut body)?;
                    }
                    out.push(Effect::When(cond, body));
                }
            }
        }
        Ok(())
    }

    /// Resolves an add or delete effect.
    fn primitive(&mut self, effect: &PrimitiveEffect, out: &mut Vec<Effect>) -> Result<(), Error> {
        match effect {
            PrimitiveEffect::AtomicFormula(formula)
            | PrimitiveEffect::NotAtomicFormula(formula) => {
                let (pred, args) = self.atom(formula)?;
                out.push(Effect::Fact {
                    pred,
                    args,
                    add: matches!(effect, PrimitiveEffect::AtomicFormula(_)),
                });
                Ok(())
            }
            PrimitiveEffect::AssignNumericFluent(..) => unsupported("numeric fluents"),
            PrimitiveEffect::AssignObjectFluent(..) => unsupported("object fluents"),
        }
    }
}
