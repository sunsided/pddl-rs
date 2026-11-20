//! The domain as code generation sees it: types in numbering order,
//! predicates and actions over typed variables, all names resolved.

use std::collections::{HashMap, HashSet};

use pddl_core::{
    ActionDefinition, AssignOp, AtomicFormula, BinaryComparison, BinaryOp, ConditionalEffect,
    DerivedPredicate, Domain, EffectCondition, Effects, FluentExpression, FunctionHead,
    FunctionTerm, GoalDescription, Literal, MultiOp, PreconditionGoalDescription,
    PreconditionGoalDescriptions, PreferenceGoalDescription, PrimitiveEffect, Requirement,
    StructureDef, Term, Type, TypedVariables,
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
    /// Whether it is derived: computed from the other facts by rules.
    pub derived: bool,
}

/// A numeric function, other than `total-cost`.
pub struct FunctionInfo {
    /// The name, lowercase.
    pub name: String,
    /// The type of each argument; `either` types are `object` (0).
    pub args: Vec<usize>,
    /// Whether no action changes it.
    pub is_static: bool,
}

/// A numeric comparison.
#[derive(Clone, Copy)]
pub enum Comparison {
    /// `<`
    Less,
    /// `<=`
    LessOrEqual,
    /// `=`
    Equal,
    /// `>=`
    GreaterOrEqual,
    /// `>`
    Greater,
}

/// A numeric expression.
pub enum NumExpr {
    /// A constant.
    Number(f64),
    /// The value of a function.
    Fluent {
        /// The function index.
        func: usize,
        /// The arguments.
        args: Vec<Arg>,
    },
    /// `a + b`
    Add(Box<NumExpr>, Box<NumExpr>),
    /// `a - b`
    Sub(Box<NumExpr>, Box<NumExpr>),
    /// `a * b`
    Mul(Box<NumExpr>, Box<NumExpr>),
    /// `a / b`
    Div(Box<NumExpr>, Box<NumExpr>),
    /// `-a`
    Neg(Box<NumExpr>),
}

impl NumExpr {
    /// Calls `f` with every function the expression reads.
    pub fn functions(&self, f: &mut impl FnMut(usize)) {
        match self {
            Self::Number(_) => {}
            Self::Fluent { func, .. } => f(*func),
            Self::Add(a, b) | Self::Sub(a, b) | Self::Mul(a, b) | Self::Div(a, b) => {
                a.functions(f);
                b.functions(f);
            }
            Self::Neg(a) => a.functions(f),
        }
    }
}

/// How a numeric effect changes its function.
#[derive(Clone, Copy)]
pub enum NumericOp {
    /// `assign`: set to the value.
    Assign,
    /// `increase`: add the value.
    Increase,
    /// `decrease`: subtract the value.
    Decrease,
    /// `scale-up`: multiply by the value.
    ScaleUp,
    /// `scale-down`: divide by the value.
    ScaleDown,
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
    /// A numeric comparison.
    Compare(Comparison, NumExpr, NumExpr),
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
    /// Changes the value of a function.
    Assign {
        /// How.
        op: NumericOp,
        /// The function index.
        func: usize,
        /// The arguments.
        args: Vec<Arg>,
        /// The value, read in the state before the action.
        value: NumExpr,
    },
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
            Self::Assign { .. } => {}
        }
    }

    /// Calls `f` with every function a numeric effect changes.
    pub fn assigned(&self, f: &mut impl FnMut(usize)) {
        match self {
            Self::Fact { .. } => {}
            Self::Forall(_, body) | Self::When(_, body) => {
                for effect in body {
                    effect.assigned(f);
                }
            }
            Self::Assign { func, .. } => f(*func),
        }
    }

    /// Whether the effects change any function.
    pub fn has_assign(effects: &[Effect]) -> bool {
        let mut found = false;
        for effect in effects {
            effect.assigned(&mut |_| found = true);
        }
        found
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
    /// The summands of the action's cost, from `(increase (total-cost) ...)`.
    pub cost: Vec<NumExpr>,
}

/// A domain, resolved for code generation.
pub struct Model {
    /// The domain's name, lowercase.
    pub name: String,
    /// The types, each before its subtypes, each subtree contiguous; `object` first.
    pub types: Vec<TypeInfo>,
    /// The predicates, in declaration order.
    pub predicates: Vec<PredicateInfo>,
    /// The numeric functions, in declaration order, without `total-cost`.
    pub functions: Vec<FunctionInfo>,
    /// The constants with their types.
    pub constants: Vec<(String, usize)>,
    /// The actions, in declaration order.
    pub actions: Vec<ActionInfo>,
    /// The rules of the derived predicates, grouped by stratum, lowest first.
    /// A rule is an [`ActionInfo`] named after its predicate, whose parameters
    /// are the head's arguments and whose precondition is the body.
    pub strata: Vec<Vec<ActionInfo>>,
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
    Requirement::NumericFluents,
    Requirement::ActionCosts,
    Requirement::Fluents,
    // Allowed so that `:fluents` works; using an object fluent is an error.
    Requirement::ObjectFluents,
    Requirement::DerivedPredicates,
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
                derived: false,
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

        let functions = functions(domain, &types)?;

        let mut model = Self {
            name: lc(domain.name()),
            types,
            predicates,
            functions,
            constants,
            actions: Vec::new(),
            strata: Vec::new(),
        };
        let mut rules = Vec::new();
        for def in domain.structure().iter() {
            if let StructureDef::Derived(derived) = def {
                let rule = Scope::new(&model).rule(derived)?;
                let pred = model
                    .predicates
                    .iter()
                    .position(|p| p.name == rule.name)
                    .expect("the rule resolved its predicate");
                model.predicates[pred].derived = true;
                rules.push((pred, rule));
            }
        }
        for def in domain.structure().iter() {
            match def {
                StructureDef::Action(action) => {
                    let action = Scope::new(&model).action(action)?;
                    model.actions.push(action);
                }
                StructureDef::DurativeAction(_) => return unsupported("durative actions"),
                StructureDef::Process(_) | StructureDef::Event(_) => {
                    return unsupported("processes and events")
                }
                StructureDef::Derived(_) => {}
            }
        }
        model.strata = stratify(&model.predicates, rules)?;
        model.mark_static()?;
        for action in &model.actions {
            for term in &action.cost {
                model
                    .check_cost(term)
                    .map_err(|e| e.context(&action.name))?;
            }
        }
        Ok(model)
    }

    /// Marks the predicates and functions that no action changes as static,
    /// and rejects actions that change a derived predicate.
    fn mark_static(&mut self) -> Result<(), Error> {
        let mut changed = HashSet::new();
        for effect in self.actions.iter().flat_map(|a| &a.effects) {
            effect.facts(&mut |pred, _| {
                changed.insert(pred);
            });
        }
        for (i, pred) in self.predicates.iter_mut().enumerate() {
            if pred.derived && changed.contains(&i) {
                return Err(Error::new(format!(
                    "an action changes the derived predicate `{}`",
                    pred.name
                )));
            }
            pred.is_static = !pred.derived && !changed.contains(&i);
        }
        let mut assigned = HashSet::new();
        for effect in self.actions.iter().flat_map(|a| &a.effects) {
            effect.assigned(&mut |func| {
                assigned.insert(func);
            });
        }
        for (i, func) in self.functions.iter_mut().enumerate() {
            func.is_static = !assigned.contains(&i);
        }
        Ok(())
    }

    /// Checks that a summand of an action cost is a non-negative integer for
    /// every binding: a sum or product of such numbers and static functions,
    /// whose values the loader checks.
    fn check_cost(&self, term: &NumExpr) -> Result<(), Error> {
        match term {
            NumExpr::Number(n) if *n >= 0.0 && n.fract() == 0.0 => Ok(()),
            NumExpr::Fluent { func, .. } if self.functions[*func].is_static => Ok(()),
            NumExpr::Add(a, b) | NumExpr::Mul(a, b) => {
                self.check_cost(a)?;
                self.check_cost(b)
            }
            _ => unsupported(
                "action costs other than sums and products of non-negative integers \
                 and static functions",
            ),
        }
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

/// Calls `f` with every predicate a condition reads and whether it reads it
/// negated (under an odd number of `not`s).
fn reads(cond: &Cond, negated: bool, f: &mut impl FnMut(usize, bool)) {
    match cond {
        Cond::Atom { pred, .. } => f(*pred, negated),
        Cond::Not(inner) => reads(inner, !negated, f),
        Cond::Exists(_, inner) | Cond::Forall(_, inner) => reads(inner, negated, f),
        Cond::And(parts) | Cond::Or(parts) => {
            for part in parts {
                reads(part, negated, f);
            }
        }
        Cond::Eq(..) | Cond::Compare(..) => {}
    }
}

/// Groups the derived predicates' rules into strata: a rule's stratum is at
/// least that of every derived predicate its body reads, and above that of
/// every one it reads negated, so each stratum only negates finished ones.
fn stratify(
    predicates: &[PredicateInfo],
    rules: Vec<(usize, ActionInfo)>,
) -> Result<Vec<Vec<ActionInfo>>, Error> {
    let mut stratum = vec![0usize; predicates.len()];
    let derived = predicates.iter().filter(|p| p.derived).count();
    loop {
        let mut changed = false;
        for (head, rule) in &rules {
            let mut need = stratum[*head];
            for cond in &rule.pre {
                reads(cond, false, &mut |pred, negated| {
                    if predicates[pred].derived {
                        need = need.max(stratum[pred] + usize::from(negated));
                    }
                });
            }
            if need > stratum[*head] {
                if need >= derived.max(1) {
                    return Err(Error::new(format!(
                        "the derived predicates can't be stratified: `{}` depends on its own negation",
                        predicates[*head].name
                    )));
                }
                stratum[*head] = need;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let mut strata: Vec<Vec<ActionInfo>> = Vec::new();
    for (head, rule) in rules {
        let s = stratum[head];
        if strata.len() <= s {
            strata.resize_with(s + 1, Vec::new);
        }
        strata[s].push(rule);
    }
    Ok(strata)
}

/// The numeric functions, without `total-cost`.
fn functions(domain: &Domain, types: &[TypeInfo]) -> Result<Vec<FunctionInfo>, Error> {
    let mut functions = Vec::new();
    for typed in domain.functions().values().iter() {
        let skeleton = typed.value_ref();
        let name = lc(skeleton.symbol());
        if **typed.type_ref() != Type::NUMBER {
            return unsupported("object fluents");
        }
        if name == "total-cost" && skeleton.variables().is_empty() {
            continue;
        }
        if functions.iter().any(|f: &FunctionInfo| f.name == name) {
            return Err(Error::new(format!("function `{name}` is declared twice")));
        }
        let args = skeleton
            .variables()
            .iter()
            .map(|v| resolve_type(types, v.type_()).map(|(t, _)| t))
            .collect::<Result<_, _>>()?;
        functions.push(FunctionInfo {
            name,
            args,
            is_static: true,
        });
    }
    Ok(functions)
}

/// The symbol and arguments of a function head.
fn head_parts(head: &FunctionHead) -> (&str, &[Term]) {
    match head {
        FunctionHead::Simple(symbol) => (symbol, &[]),
        FunctionHead::WithTerms(symbol, terms) => (symbol, terms),
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
    /// Whether effects are inside a `forall` or `when`.
    nested: bool,
    /// The summands of the action's cost.
    cost: Vec<NumExpr>,
}

impl<'a> Scope<'a> {
    /// An empty scope.
    fn new(model: &'a Model) -> Self {
        Self {
            model,
            slots: Vec::new(),
            vars: HashMap::new(),
            nested: false,
            cost: Vec::new(),
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

    /// Resolves the rule of a derived predicate.
    fn rule(mut self, derived: &DerivedPredicate) -> Result<ActionInfo, Error> {
        let name = lc(derived.predicate().name());
        let context = |e: Error| Error::new(format!("derived predicate `{name}`: {}", e.message()));
        let pred = self
            .model
            .predicates
            .iter()
            .position(|p| p.name == name)
            .ok_or_else(|| context(Error::new("it is not declared in :predicates")))?;
        let variables = derived.predicate().variables();
        if variables.len() != self.model.predicates[pred].args.len() {
            return Err(context(Error::new("it has the wrong number of arguments")));
        }
        self.bind(variables).map_err(context)?;
        let arity = self.slots.len();
        let body = self.cond(derived.expression()).map_err(context)?;
        Ok(ActionInfo {
            name,
            slots: self.slots,
            arity,
            pre: vec![body],
            effects: Vec::new(),
            cost: Vec::new(),
        })
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
            cost: self.cost,
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

    /// Resolves a function and its arguments.
    fn function(&self, symbol: &str, terms: &[Term]) -> Result<(usize, Vec<Arg>), Error> {
        let name = lc(symbol);
        if name == "total-cost" && terms.is_empty() {
            return unsupported("total-cost outside an action cost");
        }
        let func = self
            .model
            .functions
            .iter()
            .position(|f| f.name == name)
            .ok_or_else(|| Error::new(format!("unknown function `{name}`")))?;
        if self.model.functions[func].args.len() != terms.len() {
            return Err(Error::new(format!(
                "`{name}` takes {} arguments, not {}",
                self.model.functions[func].args.len(),
                terms.len()
            )));
        }
        let args = terms
            .iter()
            .map(|t| self.arg(t))
            .collect::<Result<_, _>>()?;
        Ok((func, args))
    }

    /// Resolves a function term as a numeric expression.
    fn function_term(&self, term: &FunctionTerm) -> Result<NumExpr, Error> {
        let (func, args) = self.function(term.symbol(), term.terms())?;
        Ok(NumExpr::Fluent { func, args })
    }

    /// Resolves a numeric expression.
    fn num(&self, expr: &FluentExpression) -> Result<NumExpr, Error> {
        let boxed = |e: &FluentExpression| self.num(e).map(Box::new);
        Ok(match expr {
            FluentExpression::Number(n) => NumExpr::Number(**n),
            FluentExpression::Function(head) => {
                let (symbol, terms) = head_parts(head);
                let (func, args) = self.function(symbol, terms)?;
                NumExpr::Fluent { func, args }
            }
            FluentExpression::Negative(a) => NumExpr::Neg(boxed(a)?),
            FluentExpression::BinaryOp(op, a, b) => {
                let (a, b) = (boxed(a)?, boxed(b)?);
                match op {
                    BinaryOp::Addition => NumExpr::Add(a, b),
                    BinaryOp::Subtraction => NumExpr::Sub(a, b),
                    BinaryOp::Multiplication => NumExpr::Mul(a, b),
                    BinaryOp::Division => NumExpr::Div(a, b),
                }
            }
            FluentExpression::MultiOp(op, first, rest) => {
                let mut acc = self.num(first)?;
                for e in rest {
                    let e = boxed(e)?;
                    acc = match op {
                        MultiOp::Addition => NumExpr::Add(Box::new(acc), e),
                        MultiOp::Multiplication => NumExpr::Mul(Box::new(acc), e),
                    };
                }
                acc
            }
        })
    }

    /// Resolves an atom or equality as a condition.
    fn literal(&self, formula: &AtomicFormula<Term>) -> Result<Cond, Error> {
        match formula {
            // The parser reads `(= (f) (g))` as an equality of terms; it is numeric.
            AtomicFormula::Equality(eq) => match (eq.first(), eq.second()) {
                (Term::Function(a), Term::Function(b)) => Ok(Cond::Compare(
                    Comparison::Equal,
                    self.function_term(a)?,
                    self.function_term(b)?,
                )),
                (first, second) => Ok(Cond::Eq(self.arg(first)?, self.arg(second)?)),
            },
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
            GoalDescription::FluentComparison(comparison) => Ok(Cond::Compare(
                match comparison.comparison() {
                    BinaryComparison::LessThan => Comparison::Less,
                    BinaryComparison::LessThanOrEqual => Comparison::LessOrEqual,
                    BinaryComparison::Equal => Comparison::Equal,
                    BinaryComparison::GreaterOrEqual => Comparison::GreaterOrEqual,
                    BinaryComparison::GreaterThan => Comparison::Greater,
                },
                self.num(comparison.first())?,
                self.num(comparison.second())?,
            )),
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
                    let nested = std::mem::replace(&mut self.nested, true);
                    let result = self.effects(&forall.effects, &mut body);
                    self.nested = nested;
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
                    let nested = std::mem::replace(&mut self.nested, true);
                    let result = primitives
                        .iter()
                        .try_for_each(|primitive| self.primitive(primitive, &mut body));
                    self.nested = nested;
                    result?;
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
            PrimitiveEffect::AssignNumericFluent(op, head, value) => {
                let (symbol, terms) = head_parts(head);
                if lc(symbol) == "total-cost" && terms.is_empty() {
                    if !matches!(op, AssignOp::Increase) {
                        return unsupported("changing total-cost other than with increase");
                    }
                    if self.nested {
                        return unsupported("action costs inside forall or when");
                    }
                    self.cost.push(self.num(value)?);
                    return Ok(());
                }
                let (func, args) = self.function(symbol, terms)?;
                out.push(Effect::Assign {
                    op: match op {
                        AssignOp::Assign => NumericOp::Assign,
                        AssignOp::Increase => NumericOp::Increase,
                        AssignOp::Decrease => NumericOp::Decrease,
                        AssignOp::ScaleUp => NumericOp::ScaleUp,
                        AssignOp::ScaleDown => NumericOp::ScaleDown,
                    },
                    func,
                    args,
                    value: self.num(value)?,
                });
                Ok(())
            }
            // The parser reads `(assign (f) (g))` as an object assignment; for a
            // numeric `f` it is a numeric one.
            PrimitiveEffect::AssignObjectFluent(target, Some(Term::Function(value)))
                if self
                    .model
                    .functions
                    .iter()
                    .any(|f| f.name == lc(target.symbol())) =>
            {
                let (func, args) = self.function(target.symbol(), target.terms())?;
                out.push(Effect::Assign {
                    op: NumericOp::Assign,
                    func,
                    args,
                    value: self.function_term(value)?,
                });
                Ok(())
            }
            PrimitiveEffect::AssignObjectFluent(..) => unsupported("object fluents"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pddl_core::Parser;

    /// The strata of a domain's derived predicates, by predicate name.
    fn strata(domain: &str) -> Result<Vec<Vec<String>>, Error> {
        let model = Model::new(&Domain::from_str(domain).unwrap())?;
        Ok(model
            .strata
            .iter()
            .map(|rules| rules.iter().map(|r| r.name.clone()).collect())
            .collect())
    }

    #[test]
    fn negated_derived_predicates_come_first() {
        let domain = "(define (domain d) (:requirements :derived-predicates)
          (:predicates (p ?x) (q ?x) (r ?x) (base ?x))
          (:derived (r ?x) (not (q ?x)))
          (:derived (q ?x) (or (base ?x) (q ?x)))
          (:derived (p ?x) (and (r ?x) (q ?x))))";
        assert_eq!(strata(domain).unwrap(), [vec!["q"], vec!["r", "p"]]);
    }

    /// Every domain the generator rejects names what it rejects.
    #[test]
    fn rejections_name_the_problem() {
        let act = |pre: &str, eff: &str| {
            format!(
                "(define (domain d) (:requirements :adl :numeric-fluents :action-costs)
                   (:types t) (:constants c - t) (:predicates (p ?x - t) (q))
                   (:functions (f ?x - t) (g) (total-cost))
                   (:action a :parameters (?x - t) :precondition {pre} :effect {eff}))"
            )
        };
        let cases: Vec<(String, &str)> = vec![
            ("(define (domain d) (:extends base))".into(), "unsupported: domain extension"),
            ("(define (domain d) (:predicates (p)) (:constraints (always (p))))".into(), "unsupported: constraints"),
            ("(define (domain d) (:requirements :foo))".into(), "unknown requirement: :foo"),
            ("(define (domain d) (:requirements :durative-actions))".into(), "unsupported requirement: :durative-actions"),
            ("(define (domain d) (:predicates (p) (p)))".into(), "predicate `p` is declared twice"),
            ("(define (domain d) (:predicates (p ?x - nothing)))".into(), "unknown type `nothing`"),
            ("(define (domain d) (:types a b) (:constants c - (either a b)))".into(), "constants with an `either` type"),
            ("(define (domain d) (:requirements :fluents :typing) (:types t) (:functions (f) - t))".into(), "unsupported: object fluents"),
            ("(define (domain d) (:requirements :numeric-fluents) (:functions (f) (f)))".into(), "function `f` is declared twice"),
            ("(define (domain d) (:types a a))".into(), "type `a` is declared twice"),
            ("(define (domain d) (:types a - nothing))".into(), "unknown type `nothing`"),
            ("(define (domain d) (:types a - b b - a))".into(), "the type hierarchy has a cycle"),
            ("(define (domain d) (:requirements :derived-predicates) (:predicates (p)) (:derived (q) (p)))".into(), "it is not declared in :predicates"),
            ("(define (domain d) (:requirements :derived-predicates) (:predicates (p ?x)) (:derived (p) (p ?y)))".into(), "wrong number of arguments"),
            (act("(p ?y)", "(q)"), "unknown variable `?y`"),
            (act("(p nobody)", "(q)"), "unknown constant `nobody`"),
            (act("(r ?x)", "(q)"), "unknown predicate `r`"),
            (act("(p ?x c)", "(q)"), "`p` takes 1 arguments, not 2"),
            (act("(> (h) 1)", "(q)"), "unknown function `h`"),
            (act("(> (f ?x ?x) 1)", "(q)"), "`f` takes 1 arguments, not 2"),
            (act("(> (total-cost) 1)", "(q)"), "total-cost outside an action cost"),
            (act("(q)", "(= ?x c)"), "equality outside a condition"),
            (act("(q)", "(decrease (total-cost) 1)"), "changing total-cost other than with increase"),
            (act("(q)", "(when (q) (increase (total-cost) 1))"), "action costs inside forall or when"),
            (act("(q)", "(increase (total-cost) (- 2 1))"), "action costs other than sums and products"),
            (act("(q)", "(and (increase (g) 1) (increase (total-cost) (g)))"), "action costs other than sums and products"),
            (
                "(define (domain d) (:requirements :durative-actions) (:predicates (p))
                   (:durative-action a :parameters () :duration (= ?duration 1)
                     :condition (at start (p)) :effect (at end (p))))"
                    .into(),
                "unsupported requirement",
            ),
        ];
        for (domain, expected) in cases {
            let error =
                Model::new(&Domain::from_str(&domain).unwrap_or_else(|e| panic!("{domain}: {e}")))
                    .err()
                    .unwrap_or_else(|| panic!("accepted: {domain}"));
            assert!(
                error.message().contains(expected),
                "{domain}\n{}",
                error.message()
            );
        }
    }

    /// Numeric expressions of every shape resolve, and sums and products of
    /// static functions are valid costs.
    #[test]
    fn numeric_expressions_resolve() {
        let domain = "(define (domain d) (:requirements :adl :numeric-fluents :action-costs)
          (:types t) (:predicates (p ?x - t))
          (:functions (f ?x - t) (g) (total-cost))
          (:action a :parameters (?x - t)
            :precondition (and (>= (- (* 2 (f ?x)) (/ (g) 2)) (- (g))) (= (f ?x) (g)) (p ?x))
            :effect (and (scale-up (g) 2) (scale-down (g) 2) (assign (g) (+ 1 2 3))
                         (increase (total-cost) (* 2 (f ?x))))))";
        let model = Model::new(&Domain::from_str(domain).unwrap()).unwrap();
        assert!(model.functions[0].is_static && !model.functions[1].is_static);
        assert_eq!(model.actions[0].cost.len(), 1);
    }

    #[test]
    fn derived_predicates_must_not_be_changed_by_actions() {
        let domain = "(define (domain d) (:requirements :derived-predicates)
          (:predicates (p) (q))
          (:derived (p) (q))
          (:action a :parameters () :precondition () :effect (p)))";
        assert!(strata(domain)
            .unwrap_err()
            .message()
            .contains("changes the derived predicate `p`"));
    }
}
