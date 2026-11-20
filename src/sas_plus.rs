//! Grounds a PDDL domain and problem into a Fast Downward SAS+ task.
//!
//! [`ground`] instantiates the actions over the problem's objects that are
//! reachable in the delete relaxation, computing reachability while it binds
//! parameters so unreachable groundings are never built, and returns a
//! [`SasFile`] that prints as Fast Downward's `output.sas` (file version 3).
//! Call [`SasFile::to_task`] for a plain [`Task`].
//!
//! ## Encoding
//! Every reachable ground atom of a fluent predicate (one that some action
//! changes) becomes a binary variable with the values `Atom p(a, b)` (value 0)
//! and `NegatedAtom p(a, b)` (value 1), named as Fast Downward names them.
//! Static predicates are evaluated during grounding and do not become variables.
//! There are no mutex groups and no axioms.
//!
//! PDDL applies deletes before adds, so a delete effect only fires where no add
//! of the same atom does. Where conditional adds overlap a delete, the delete is
//! split into conditional effects that exclude them; a split that would need more
//! than [`MAX_DELETE_PIECES`] effects fails with [`GroundError::ConflictingEffects`].
//!
//! ## Supported subset
//! - typing, including `either` and supertypes; constants and objects,
//! - preconditions and goals made of `and`, atoms, negated atoms and `=`,
//!   `forall`, and `exists` at the top level of a precondition,
//! - add and delete effects, `forall` and `when` (conditional effects),
//! - action costs: `(increase (total-cost) <n>)` with a number or a static
//!   function from the initial state, used when the problem has
//!   `(:metric minimize (total-cost))`; otherwise every action costs 1.
//!   [`ground`] needs integer costs; [`ground_scaled`] also takes decimal
//!   costs and scales them to integers by a power of ten it returns.
//!
//! Everything else (`or`, `imply`, numeric fluents, derived predicates,
//! durative actions, preferences, constraints, …) is rejected with
//! [`GroundError::Unsupported`] instead of being dropped.
//!
//! ## Example
//! ```
//! # #[cfg(feature = "parser")]
//! # fn main() {
//! use pddl::{Domain, Parser, Problem};
//!
//! let domain = Domain::from_str(
//!     "(define (domain d) (:predicates (on ?x))
//!        (:action switch :parameters (?x) :precondition (not (on ?x)) :effect (on ?x)))",
//! )
//! .unwrap();
//! let problem = Problem::from_str(
//!     "(define (problem p) (:domain d) (:objects a b) (:init) (:goal (on a)))",
//! )
//! .unwrap();
//!
//! let file = pddl::sas_plus::ground(&domain, &problem).unwrap();
//! assert_eq!(file.variables.len(), 2);
//! assert_eq!(file.operators[0].name, "switch a");
//! print!("{file}"); // output.sas
//! # }
//! # #[cfg(not(feature = "parser"))]
//! # fn main() {}
//! ```

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

// The types `ground` returns and their fields, so callers can name them
// without depending on `sas-plus` themselves.
pub use sas_plus::fdr::{self, SasFile};
use sas_plus::fdr::{SasEffect, SasOperator, SasVariable};
pub use sas_plus::{Fact, Task, Value, VarId};

use crate::{
    ActionDefinition, AssignOp, AtomicFormula, ConditionalEffect, Domain, EffectCondition, Effects,
    FluentExpression, FunctionHead, GoalDescription, InitElement, Literal, MetricFluentExpression,
    Optimization, PreconditionGoalDescription, PreconditionGoalDescriptions,
    PreferenceGoalDescription, PrimitiveEffect, Problem, StructureDef, Term, Type, TypedVariables,
};

/// Why a domain and problem could not be grounded.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum GroundError {
    /// The input uses a PDDL construct the grounder does not handle.
    #[error("unsupported PDDL construct: {0}")]
    Unsupported(&'static str),
    /// A type is used but never declared.
    #[error("unknown type `{0}`")]
    UnknownType(String),
    /// A name is used but is neither an object nor a constant.
    #[error("unknown object `{0}`")]
    UnknownObject(String),
    /// A variable is used outside the scope that declares it.
    #[error("unknown variable `{0}`")]
    UnknownVariable(String),
    /// An action's cost is not a non-negative integer, or refers to a function
    /// without a value in the initial state.
    #[error("action `{action}` has a cost that is not a non-negative integer")]
    InvalidCost {
        /// The ground action, e.g. `drive depot north`.
        action: String,
    },
    /// An action's cost doesn't fit in a `u32`: as it is, or, with
    /// [`ground_scaled`], scaled to the most decimal places of any cost.
    #[error("action `{action}` costs more than a u32 holds at {places} decimal places")]
    CostOutOfRange {
        /// The ground action, e.g. `drive depot north`.
        action: String,
        /// The decimal places that every cost is scaled to.
        places: u32,
    },
    /// The goal is false in every reachable state.
    #[error("the goal is unreachable")]
    Unsolvable,
    /// A delete effect overlaps so many conditional adds of the same atom
    /// that encoding "the add wins" would take more than
    /// [`MAX_DELETE_PIECES`] conditional effects.
    #[error(
        "action `{action}` deletes an atom that too many conditional adds of it overlap \
         to encode in at most {MAX_DELETE_PIECES} conditional effects"
    )]
    ConflictingEffects {
        /// The ground action, e.g. `drive depot north`.
        action: String,
    },
}

/// The most conditional effects one delete effect of a ground action may be
/// split into; see [`GroundError::ConflictingEffects`].
///
/// PDDL applies deletes before adds, so a delete only takes effect where no add
/// of the same atom fires. Each conditional add the delete's conditions neither
/// imply nor contradict splits the delete into one piece per literal of the
/// add's condition, so the number of pieces is a product over those adds.
pub const MAX_DELETE_PIECES: usize = 1024;

/// Grounds `domain` and `problem` into a SAS+ task with one binary variable
/// per reachable fluent atom. See the [module documentation](self) for the
/// supported subset of PDDL.
///
/// # Errors
/// Returns a [`GroundError`] if the input uses an unsupported construct, refers
/// to undeclared types, objects or variables, has an action cost that is not a
/// non-negative integer ([`InvalidCost`](GroundError::InvalidCost)) or doesn't
/// fit in a `u32` ([`CostOutOfRange`](GroundError::CostOutOfRange)), or has a
/// goal that no reachable state satisfies.
pub fn ground(domain: &Domain, problem: &Problem) -> Result<SasFile, GroundError> {
    ground_with_costs(domain, problem, false).map(|task| task.file)
}

/// A SAS+ task whose operator costs are the action costs times
/// [`cost_scale`](Self::cost_scale).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaledTask {
    /// The task. Divide its operator and plan costs by `cost_scale` for the
    /// costs the PDDL states.
    pub file: SasFile,
    /// `10^k` for the most decimal places `k` of any reachable action's cost:
    /// 1 when every cost is an integer, 10 for costs such as `0.5` and `2`.
    pub cost_scale: u32,
}

/// Like [`ground`], but accepts action costs with decimal places: it scales
/// every cost by `10^k`, for the most decimal places `k` of any reachable
/// action's cost, so that the costs stay exact integers, and returns the
/// scale. `0.1` and `0.3` become 1 and 3 with a scale of 10. A problem with
/// integer costs grounds to the same task as with [`ground`], with a scale of 1.
///
/// Costs are read from the numbers in the parsed PDDL, which are `f64`: each
/// is taken as the shortest decimal that reads back as the same `f64`, which is
/// the literal as written for up to 15 significant digits.
///
/// # Errors
/// As [`ground`], except that decimal costs are accepted, and
/// [`GroundError::CostOutOfRange`] if a scaled cost doesn't fit in a `u32`.
pub fn ground_scaled(domain: &Domain, problem: &Problem) -> Result<ScaledTask, GroundError> {
    ground_with_costs(domain, problem, true)
}

/// Grounds with every cost scaled to the most decimal places of any cost in
/// the task; without `decimal_costs`, a cost with decimal places is an
/// [`InvalidCost`](GroundError::InvalidCost).
fn ground_with_costs(
    domain: &Domain,
    problem: &Problem,
    decimal_costs: bool,
) -> Result<ScaledTask, GroundError> {
    reject_unsupported(domain, problem)?;
    let world = World::new(domain, problem)?;
    let use_metric = uses_total_cost(problem)?;
    let (init, functions) = initial_state(problem, &world)?;

    let schemas = domain
        .structure()
        .iter()
        .map(|def| match def {
            StructureDef::Action(action) => Schema::new(&world, action),
            StructureDef::DurativeAction(_) => Err(GroundError::Unsupported("durative actions")),
            StructureDef::Derived(_) => Err(GroundError::Unsupported("derived predicates")),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let fluent: HashSet<&str> = schemas
        .iter()
        .flat_map(|schema| &schema.effects)
        .map(|effect| effect.atom.pred.as_str())
        .collect();

    let grounder = Grounder {
        init: &init,
        fluent: &fluent,
        functions: &functions,
        use_metric,
    };
    let (ops, reachable) = grounder.reachable_ops(&schemas)?;
    let index: HashMap<&Atom, VarId> = reachable
        .iter()
        .enumerate()
        .map(|(var, atom)| (atom, var))
        .collect();

    let mut goal_lits = Vec::new();
    Builder {
        world: &world,
        params: RefCell::default(),
    }
    .preconditions(problem.goals(), &HashMap::new(), false, &mut goal_lits)?;
    let goal = ground_goal(&goal_lits, &grounder, &index)?;

    // Only the operators that make it into the task set the scale.
    let mut operators = Vec::new();
    for op in &ops {
        if let Some(operator) = op.to_sas(&index, 0)? {
            operators.push((operator, op));
        }
    }
    if let Some((_, op)) = operators.iter().find(|(_, op)| op.cost.places > 0) {
        if !decimal_costs {
            return Err(GroundError::InvalidCost {
                action: op.name.clone(),
            });
        }
    }
    let places = operators
        .iter()
        .map(|(_, op)| op.cost.places)
        .max()
        .unwrap_or(0);
    let operators = operators
        .into_iter()
        .map(|(mut operator, op)| {
            operator.cost = op
                .cost
                .scaled(places)
                .ok_or_else(|| GroundError::CostOutOfRange {
                    action: op.name.clone(),
                    places,
                })?;
            Ok(operator)
        })
        .collect::<Result<_, GroundError>>()?;
    let file = SasFile {
        use_metric,
        variables: reachable
            .iter()
            .enumerate()
            .map(|(var, atom)| SasVariable {
                name: format!("var{var}"),
                axiom_layer: None,
                values: vec![
                    format!("Atom {}", atom_name(atom)),
                    format!("NegatedAtom {}", atom_name(atom)),
                ],
            })
            .collect(),
        mutex_groups: Vec::new(),
        initial: reachable
            .iter()
            .map(|atom| if init.contains(atom) { TRUE } else { FALSE })
            .collect(),
        goal,
        operators,
        axioms: Vec::new(),
    };
    let cost_scale = 10u32.pow(places); // `scaled` checked that it fits
    Ok(ScaledTask { file, cost_scale })
}

/// A non-negative decimal number, exact: `units / 10^places`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Decimal {
    /// The value times `10^places`.
    units: u128,
    /// The number of decimal places.
    places: u32,
}

impl Decimal {
    /// A whole number.
    const fn whole(units: u128) -> Self {
        Self { units, places: 0 }
    }

    /// The shortest decimal that reads back as `value`, or `None` if it is
    /// negative, not finite, or has more digits than fit.
    fn from_f64(value: f64) -> Option<Self> {
        if !value.is_finite() || value < 0.0 {
            return None;
        }
        // `Display` for `f64` prints the shortest round-trip digits, never an exponent.
        let text = value.to_string();
        let (whole, fraction) = text.split_once('.').unwrap_or((&text, ""));
        let places = u32::try_from(fraction.len()).ok()?;
        let units = format!("{whole}{fraction}").parse().ok()?;
        Some(Self { units, places })
    }

    /// The exact sum, or `None` if it doesn't fit.
    fn checked_add(self, other: Self) -> Option<Self> {
        let places = self.places.max(other.places);
        let units = self
            .units
            .checked_mul(10u128.checked_pow(places - self.places)?)?
            .checked_add(
                other
                    .units
                    .checked_mul(10u128.checked_pow(places - other.places)?)?,
            )?;
        Some(Self { units, places }.normalized())
    }

    /// The same number without trailing zero places: `1.0` is `1`, so a sum
    /// such as `0.5 + 0.5` counts as an integer.
    fn normalized(mut self) -> Self {
        while self.places > 0 && self.units % 10 == 0 {
            self.units /= 10;
            self.places -= 1;
        }
        self
    }

    /// The value times `10^places`, if that is a `u32` and `places` is at
    /// least the number's own decimal places.
    fn scaled(self, places: u32) -> Option<u32> {
        10u32.checked_pow(places)?;
        let units = self
            .units
            .checked_mul(10u128.checked_pow(places.checked_sub(self.places)?)?)?;
        u32::try_from(units).ok()
    }
}

/// The value of a binary variable whose atom holds.
const TRUE: Value = 0;
/// The value of a binary variable whose atom does not hold.
const FALSE: Value = 1;

/// A ground atom: predicate and object names, all lowercase.
type Atom = (String, Vec<String>);

/// Formats an atom the way Fast Downward does, e.g. `at(ball1, rooma)`.
fn atom_name((pred, args): &Atom) -> String {
    format!("{pred}({})", args.join(", "))
}

/// Lowercases a PDDL name; PDDL is case-insensitive.
fn lc(name: &str) -> String {
    name.to_lowercase()
}

/// Rejects domain and problem sections the grounder cannot encode.
fn reject_unsupported(domain: &Domain, problem: &Problem) -> Result<(), GroundError> {
    if !domain.extends().is_empty() {
        return Err(GroundError::Unsupported("domain extension"));
    }
    if !domain.constraints().is_empty() || !problem.constraints().is_empty() {
        return Err(GroundError::Unsupported("constraints"));
    }
    if !domain.timeless().values().is_empty() {
        return Err(GroundError::Unsupported("timeless literals"));
    }
    Ok(())
}

/// Whether the problem asks to minimize `total-cost`; any other metric is
/// unsupported.
fn uses_total_cost(problem: &Problem) -> Result<bool, GroundError> {
    match problem.metric_spec() {
        None => Ok(false),
        Some(metric) => match (metric.optimization(), metric.expression()) {
            (Optimization::Minimize, MetricFluentExpression::Function(symbol, args))
                if args.is_empty() && lc(symbol) == "total-cost" =>
            {
                Ok(true)
            }
            _ => Err(GroundError::Unsupported(
                "metrics other than minimizing total-cost",
            )),
        },
    }
}

/// The objects of the problem, grouped by type.
struct World {
    /// Every object and constant.
    objects: HashSet<String>,
    /// The objects of each declared type, including those of its subtypes,
    /// in declaration order.
    by_type: HashMap<String, Vec<String>>,
}

impl World {
    /// Collects the constants and objects and resolves the type hierarchy.
    fn new(domain: &Domain, problem: &Problem) -> Result<Self, GroundError> {
        let mut parents: HashMap<String, Vec<String>> = HashMap::new();
        parents.entry("object".to_string()).or_default();
        for typed in domain.types().iter() {
            let ty = lc(typed.value());
            for parent in typed.type_().clone() {
                let parent = lc(&parent);
                parents.entry(parent.clone()).or_default();
                if parent != ty {
                    parents.entry(ty.clone()).or_default().push(parent);
                }
            }
        }

        let mut world = Self {
            objects: HashSet::new(),
            by_type: parents.keys().map(|ty| (ty.clone(), Vec::new())).collect(),
        };
        for typed in domain.constants().iter().chain(problem.objects().iter()) {
            // A redeclared object, e.g. a constant listed again as an object,
            // gets the types of every declaration.
            let object = lc(typed.value());
            let redeclared = !world.objects.insert(object.clone());
            let mut pending: Vec<String> =
                typed.type_().clone().into_iter().map(|t| lc(&t)).collect();
            pending.push("object".to_string());
            let mut seen = HashSet::new();
            while let Some(ty) = pending.pop() {
                let Some(ty_parents) = parents.get(&ty) else {
                    return Err(GroundError::UnknownType(ty));
                };
                if seen.insert(ty.clone()) {
                    pending.extend(ty_parents.iter().cloned());
                    let members = world.by_type.get_mut(&ty).expect("declared");
                    if !redeclared || !members.contains(&object) {
                        members.push(object.clone());
                    }
                }
            }
        }
        Ok(world)
    }

    /// The objects of `ty`, a single type or an `either` of types.
    fn objects_of(&self, ty: &Type) -> Result<Vec<String>, GroundError> {
        let mut objects = Vec::new();
        let mut seen = HashSet::new();
        for primitive in ty.clone() {
            let name = lc(&primitive);
            let members = self
                .by_type
                .get(&name)
                .ok_or(GroundError::UnknownType(name))?;
            for object in members {
                if seen.insert(object) {
                    objects.push(object.clone());
                }
            }
        }
        Ok(objects)
    }

    /// Calls `f` with every way of binding `vars` to objects, each extending
    /// `subst`, one at a time.
    fn each_substitution(
        &self,
        vars: &TypedVariables,
        subst: &Scope,
        f: &mut dyn FnMut(&Scope) -> Result<(), GroundError>,
    ) -> Result<(), GroundError> {
        let vars = vars
            .iter()
            .map(|var| Ok((lc(var.value()), self.objects_of(var.type_())?)))
            .collect::<Result<Vec<_>, GroundError>>()?;
        let mut scope = subst.clone();
        Self::bind_each(&vars, &mut scope, f)
    }

    /// Binds the first of `vars` to each of its objects, then the rest.
    fn bind_each(
        vars: &[(String, Vec<String>)],
        scope: &mut Scope,
        f: &mut dyn FnMut(&Scope) -> Result<(), GroundError>,
    ) -> Result<(), GroundError> {
        let Some(((name, objects), rest)) = vars.split_first() else {
            return f(scope);
        };
        let shadowed = scope.get(name).cloned();
        for object in objects {
            scope.insert(name.clone(), Arg::Obj(object.clone()));
            Self::bind_each(rest, scope, f)?;
        }
        match shadowed {
            Some(arg) => scope.insert(name.clone(), arg),
            None => scope.remove(name),
        };
        Ok(())
    }
}

/// The ground atoms that hold initially and the values of numeric functions.
type Initial = (HashSet<Atom>, HashMap<Atom, f64>);

/// Reads the initial state of the problem.
fn initial_state(problem: &Problem, world: &World) -> Result<Initial, GroundError> {
    let object = |name: &crate::Name| {
        let name = lc(name);
        if world.objects.contains(&name) {
            Ok(name)
        } else {
            Err(GroundError::UnknownObject(name))
        }
    };
    let mut atoms = HashSet::new();
    let mut functions = HashMap::new();
    for element in problem.init().iter() {
        match element {
            InitElement::Literal(Literal::AtomicFormula(AtomicFormula::Predicate(atom))) => {
                let args = atom.values().iter().map(object).collect::<Result<_, _>>()?;
                atoms.insert((lc(atom.predicate()), args));
            }
            // Closed world: negative initial literals hold anyway.
            InitElement::Literal(Literal::NotAtomicFormula(AtomicFormula::Predicate(_))) => {}
            InitElement::Literal(_) => {
                return Err(GroundError::Unsupported("equality in the initial state"))
            }
            InitElement::At(..) => return Err(GroundError::Unsupported("timed initial literals")),
            InitElement::IsValue(head, value) => {
                let args = head.names().iter().map(object).collect::<Result<_, _>>()?;
                functions.insert((lc(head.symbol()), args), **value);
            }
            InitElement::IsObject(..) => return Err(GroundError::Unsupported("object fluents")),
        }
    }
    Ok((atoms, functions))
}

/// The variables in scope, each bound to a parameter or, inside `forall`, to
/// an object.
type Scope = HashMap<String, Arg>;

/// An argument of a schematic atom: a parameter (by index) or an object.
#[derive(Clone)]
enum Arg {
    /// The parameter at this index.
    Var(usize),
    /// A fixed object.
    Obj(String),
}

/// An atom whose arguments may be parameters.
#[derive(Clone)]
struct SAtom {
    /// The predicate, lowercase.
    pred: String,
    /// The arguments.
    args: Vec<Arg>,
}

/// A schematic literal of a precondition, goal or effect condition.
#[derive(Clone)]
enum SLit {
    /// An atom that must hold (`true`) or not hold (`false`).
    Atom(SAtom, bool),
    /// Two arguments that must be equal (`true`) or different (`false`).
    Eq(Arg, Arg, bool),
}

impl SLit {
    /// The highest parameter index the literal mentions.
    fn last_param(&self) -> Option<usize> {
        let args: Vec<&Arg> = match self {
            Self::Atom(atom, _) => atom.args.iter().collect(),
            Self::Eq(a, b, _) => vec![a, b],
        };
        args.into_iter()
            .filter_map(|arg| match arg {
                Arg::Var(i) => Some(*i),
                Arg::Obj(_) => None,
            })
            .max()
    }
}

/// A schematic add (`add`) or delete effect with its conditions.
struct SEffect {
    /// The `when` conditions; empty for an unconditional effect.
    conds: Vec<SLit>,
    /// The atom that is added or deleted.
    atom: SAtom,
    /// Whether the atom is added rather than deleted.
    add: bool,
}

/// One summand of an action's cost.
enum CostTerm {
    /// A constant cost.
    Number(f64),
    /// The value of a static numeric function.
    Function(String, Vec<Arg>),
}

/// An action with its quantifiers expanded, ready to be instantiated.
struct Schema {
    /// The action name, lowercase.
    name: String,
    /// The number of declared parameters; the remaining parameters come from
    /// top-level `exists` in the precondition and are not part of the name.
    arity: usize,
    /// The objects each parameter can be bound to.
    params: Vec<Vec<String>>,
    /// The precondition as a conjunction of literals.
    pre: Vec<SLit>,
    /// The add and delete effects.
    effects: Vec<SEffect>,
    /// The summands of the action's cost.
    cost: Vec<CostTerm>,
}

impl Schema {
    /// Flattens an action definition.
    fn new(world: &World, action: &ActionDefinition) -> Result<Self, GroundError> {
        let mut subst = Scope::new();
        let mut params = Vec::new();
        for var in action.parameters().iter() {
            subst.insert(lc(var.value()), Arg::Var(params.len()));
            params.push(world.objects_of(var.type_())?);
        }
        let arity = params.len();

        // Effects see only the declared parameters, not the `exists` binders.
        let builder = Builder {
            world,
            params: RefCell::new(params),
        };
        let mut pre = Vec::new();
        builder.preconditions(action.precondition(), &subst, true, &mut pre)?;
        let mut effects = Vec::new();
        let mut cost = Vec::new();
        if let Some(effect) = action.effect() {
            builder.effects(effect, &subst, &[], &mut effects, &mut cost)?;
        }
        Ok(Self {
            name: lc(action.symbol()),
            arity,
            params: builder.params.into_inner(),
            pre,
            effects,
            cost,
        })
    }
}

/// Turns goal descriptions and effects into schematic literals and effects.
struct Builder<'a> {
    /// The objects, for `forall` and constant names.
    world: &'a World,
    /// The objects each parameter can be bound to; a top-level `exists` in a
    /// precondition adds a parameter per binder.
    params: RefCell<Vec<Vec<String>>>,
}

impl Builder<'_> {
    /// Resolves a term to a parameter or an object.
    fn arg(&self, term: &Term, subst: &Scope) -> Result<Arg, GroundError> {
        match term {
            Term::Name(name) => {
                let name = lc(name);
                if self.world.objects.contains(&name) {
                    Ok(Arg::Obj(name))
                } else {
                    Err(GroundError::UnknownObject(name))
                }
            }
            Term::Variable(var) => {
                let name = lc(var);
                subst
                    .get(&name)
                    .cloned()
                    .ok_or(GroundError::UnknownVariable(name))
            }
            Term::Function(_) => Err(GroundError::Unsupported("object fluents")),
        }
    }

    /// Converts an atomic formula into a literal of the given sign.
    fn literal(
        &self,
        formula: &AtomicFormula<Term>,
        positive: bool,
        subst: &Scope,
    ) -> Result<SLit, GroundError> {
        Ok(match formula {
            AtomicFormula::Equality(eq) => SLit::Eq(
                self.arg(eq.first(), subst)?,
                self.arg(eq.second(), subst)?,
                positive,
            ),
            AtomicFormula::Predicate(_) => SLit::Atom(self.atom(formula, subst)?, positive),
        })
    }

    /// Converts a predicate atom.
    fn atom(&self, formula: &AtomicFormula<Term>, subst: &Scope) -> Result<SAtom, GroundError> {
        match formula {
            AtomicFormula::Predicate(atom) => Ok(SAtom {
                pred: lc(atom.predicate()),
                args: atom
                    .values()
                    .iter()
                    .map(|term| self.arg(term, subst))
                    .collect::<Result<_, _>>()?,
            }),
            AtomicFormula::Equality(_) => Err(GroundError::Unsupported("equality in effects")),
        }
    }

    /// Flattens a precondition or goal list into literals.
    fn preconditions(
        &self,
        pres: &PreconditionGoalDescriptions,
        subst: &Scope,
        allow_exists: bool,
        out: &mut Vec<SLit>,
    ) -> Result<(), GroundError> {
        for pre in pres {
            match pre {
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(gd)) => {
                    self.goal(gd, subst, allow_exists, out)?;
                }
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Preference(
                    _,
                )) => {
                    return Err(GroundError::Unsupported("preferences"));
                }
                PreconditionGoalDescription::Forall(vars, inner) => {
                    self.world.each_substitution(vars, subst, &mut |s| {
                        self.preconditions(inner, s, false, out)
                    })?;
                }
            }
        }
        Ok(())
    }

    /// Flattens a goal description into a conjunction of literals.
    fn goal(
        &self,
        gd: &GoalDescription,
        subst: &Scope,
        allow_exists: bool,
        out: &mut Vec<SLit>,
    ) -> Result<(), GroundError> {
        match gd {
            GoalDescription::AtomicFormula(formula)
            | GoalDescription::Literal(Literal::AtomicFormula(formula)) => {
                out.push(self.literal(formula, true, subst)?);
            }
            GoalDescription::Literal(Literal::NotAtomicFormula(formula)) => {
                out.push(self.literal(formula, false, subst)?);
            }
            GoalDescription::Not(inner) => match inner.as_ref() {
                GoalDescription::AtomicFormula(formula)
                | GoalDescription::Literal(Literal::AtomicFormula(formula)) => {
                    out.push(self.literal(formula, false, subst)?);
                }
                GoalDescription::Literal(Literal::NotAtomicFormula(formula)) => {
                    out.push(self.literal(formula, true, subst)?);
                }
                _ => return Err(GroundError::Unsupported("not over a non-atomic formula")),
            },
            GoalDescription::And(gds) => {
                for gd in gds {
                    self.goal(gd, subst, allow_exists, out)?;
                }
            }
            GoalDescription::Exists(vars, inner) if allow_exists => {
                let mut scope = subst.clone();
                // Each binder becomes an extra parameter of the action.
                for var in vars.iter() {
                    let objects = self.world.objects_of(var.type_())?;
                    let mut params = self.params.borrow_mut();
                    scope.insert(lc(var.value()), Arg::Var(params.len()));
                    params.push(objects);
                }
                self.goal(inner, &scope, true, out)?;
            }
            GoalDescription::Exists(..) => return Err(GroundError::Unsupported("nested exists")),
            GoalDescription::ForAll(vars, inner) => {
                self.world
                    .each_substitution(vars, subst, &mut |s| self.goal(inner, s, false, out))?;
            }
            GoalDescription::Or(_) => return Err(GroundError::Unsupported("or")),
            GoalDescription::Imply(..) => return Err(GroundError::Unsupported("imply")),
            GoalDescription::FluentComparison(_) => {
                return Err(GroundError::Unsupported("numeric fluents"))
            }
        }
        Ok(())
    }

    /// Flattens effects under the conditions `conds` collected so far.
    fn effects(
        &self,
        effects: &Effects,
        subst: &Scope,
        conds: &[SLit],
        out: &mut Vec<SEffect>,
        cost: &mut Vec<CostTerm>,
    ) -> Result<(), GroundError> {
        for effect in effects {
            match effect {
                ConditionalEffect::Effect(effect) => {
                    self.primitive(effect, subst, conds, out, cost)?;
                }
                ConditionalEffect::Forall(forall) => {
                    self.world
                        .each_substitution(&forall.variables, subst, &mut |s| {
                            self.effects(&forall.effects, s, conds, out, cost)
                        })?;
                }
                ConditionalEffect::When(when) => {
                    let mut conds = conds.to_vec();
                    self.goal(&when.condition, subst, false, &mut conds)?;
                    let primitives = match &when.effect {
                        EffectCondition::Single(effect) => std::slice::from_ref(effect),
                        EffectCondition::All(effects) => effects.as_slice(),
                    };
                    for effect in primitives {
                        self.primitive(effect, subst, &conds, out, cost)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Converts an add, delete or cost effect.
    fn primitive(
        &self,
        effect: &PrimitiveEffect,
        subst: &Scope,
        conds: &[SLit],
        out: &mut Vec<SEffect>,
        cost: &mut Vec<CostTerm>,
    ) -> Result<(), GroundError> {
        match effect {
            PrimitiveEffect::AtomicFormula(formula)
            | PrimitiveEffect::NotAtomicFormula(formula) => {
                out.push(SEffect {
                    conds: conds.to_vec(),
                    atom: self.atom(formula, subst)?,
                    add: matches!(effect, PrimitiveEffect::AtomicFormula(_)),
                });
            }
            PrimitiveEffect::AssignNumericFluent(AssignOp::Increase, head, value)
                if is_total_cost(head) =>
            {
                if !conds.is_empty() {
                    return Err(GroundError::Unsupported("conditional action costs"));
                }
                cost.push(match value {
                    FluentExpression::Number(n) => CostTerm::Number(**n),
                    FluentExpression::Function(FunctionHead::Simple(symbol)) => {
                        CostTerm::Function(lc(symbol), Vec::new())
                    }
                    FluentExpression::Function(FunctionHead::WithTerms(symbol, terms)) => {
                        CostTerm::Function(
                            lc(symbol),
                            terms
                                .iter()
                                .map(|t| self.arg(t, subst))
                                .collect::<Result<_, _>>()?,
                        )
                    }
                    _ => return Err(GroundError::Unsupported("numeric fluents")),
                });
            }
            PrimitiveEffect::AssignNumericFluent(..) => {
                return Err(GroundError::Unsupported("numeric fluents"))
            }
            PrimitiveEffect::AssignObjectFluent(..) => {
                return Err(GroundError::Unsupported("object fluents"))
            }
        }
        Ok(())
    }
}

/// Whether a function head is `(total-cost)`.
fn is_total_cost(head: &FunctionHead) -> bool {
    match head {
        FunctionHead::Simple(symbol) => lc(symbol) == "total-cost",
        FunctionHead::WithTerms(symbol, terms) => terms.is_empty() && lc(symbol) == "total-cost",
    }
}

/// A ground literal on a fluent atom.
type GLit = (Atom, bool);

/// A ground effect on a fluent atom.
struct GEffect {
    /// The fluent conditions; static ones were evaluated away.
    conds: Vec<GLit>,
    /// The atom that is added or deleted.
    atom: Atom,
    /// Whether the atom is added rather than deleted.
    add: bool,
}

/// A ground action.
struct GroundOp {
    /// The name, e.g. `pick ball1 rooma left`.
    name: String,
    /// The fluent preconditions; static ones were evaluated away.
    pre: Vec<GLit>,
    /// The add and delete effects.
    effects: Vec<GEffect>,
    /// The cost, exact.
    cost: Decimal,
}

/// The state of binding the parameters of one schema.
struct Search<'s, 'o> {
    /// The schema being instantiated.
    schema: &'s Schema,
    /// The literals to check once each parameter is bound.
    checks: &'s [Vec<&'s SLit>],
    /// The fluent atoms reachable so far.
    reachable: &'s HashSet<Atom>,
    /// The bindings already grounded, as object indices.
    emitted: &'o mut HashSet<Vec<usize>>,
    /// The objects bound so far.
    binding: Vec<String>,
    /// The indices of those objects in the parameters' object lists.
    path: Vec<usize>,
    /// The ground actions found, with their object indices.
    out: &'o mut Vec<(Vec<usize>, GroundOp)>,
}

/// Instantiates schemas against the initial state.
struct Grounder<'a> {
    /// The atoms that hold initially.
    init: &'a HashSet<Atom>,
    /// The predicates that some action changes.
    fluent: &'a HashSet<&'a str>,
    /// The initial values of numeric functions.
    functions: &'a HashMap<Atom, f64>,
    /// Whether action costs count.
    use_metric: bool,
}

impl Grounder<'_> {
    /// Grounds an atom under a binding.
    fn atom(atom: &SAtom, binding: &[String]) -> Atom {
        (
            atom.pred.clone(),
            atom.args
                .iter()
                .map(|arg| ground_arg(arg, binding))
                .collect(),
        )
    }

    /// Evaluates a literal that does not depend on the state: an equality or a
    /// static atom. Returns `None` for a literal on a fluent atom.
    fn eval_static(&self, lit: &SLit, binding: &[String]) -> Option<bool> {
        match lit {
            SLit::Eq(a, b, positive) => {
                Some((ground_arg(a, binding) == ground_arg(b, binding)) == *positive)
            }
            SLit::Atom(atom, positive) if !self.fluent.contains(atom.pred.as_str()) => {
                Some(self.init.contains(&Self::atom(atom, binding)) == *positive)
            }
            SLit::Atom(..) => None,
        }
    }

    /// Grounds the fluent literals of a conjunction, or returns `None` if a
    /// static literal is false.
    fn fluent_lits(&self, lits: &[SLit], binding: &[String]) -> Option<Vec<GLit>> {
        let mut out = Vec::new();
        for lit in lits {
            match (self.eval_static(lit, binding), lit) {
                (Some(false), _) => return None,
                (None, SLit::Atom(atom, positive)) => {
                    out.push((Self::atom(atom, binding), *positive));
                }
                _ => {}
            }
        }
        Some(out)
    }

    /// Grounds the actions that are applicable in the delete relaxation and
    /// returns them, in schema and parameter order, with the reachable fluent
    /// atoms, sorted.
    ///
    /// Grounding and reachability alternate, as in Fast Downward: a parameter
    /// is only bound to objects for which the static and positive fluent
    /// preconditions mentioning it already hold or are reachable, so groundings
    /// that can never apply are never built.
    fn reachable_ops(
        &self,
        schemas: &[Schema],
    ) -> Result<(Vec<GroundOp>, BTreeSet<Atom>), GroundError> {
        let mut reachable: HashSet<Atom> = self
            .init
            .iter()
            .filter(|(pred, _)| self.fluent.contains(pred.as_str()))
            .cloned()
            .collect();
        // Each ground action with its schema and the indices of its objects,
        // which order them as one pass over every binding would.
        let mut ops: Vec<(usize, Vec<usize>, GroundOp)> = Vec::new();
        let mut emitted: Vec<HashSet<Vec<usize>>> = vec![HashSet::new(); schemas.len()];
        // The add effects of `ops` whose conditions were not reachable yet.
        let mut pending: Vec<(Vec<GLit>, Atom)> = Vec::new();
        loop {
            let before = ops.len();
            for (i, schema) in schemas.iter().enumerate() {
                let mut found = Vec::new();
                self.instantiate(schema, &reachable, &mut emitted[i], &mut found)?;
                ops.extend(found.into_iter().map(|(path, op)| (i, path, op)));
            }
            if ops.len() == before {
                break;
            }
            for (_, _, op) in &ops[before..] {
                pending.extend(
                    op.effects
                        .iter()
                        .filter(|effect| effect.add)
                        .map(|effect| (effect.conds.clone(), effect.atom.clone())),
                );
            }
            // Fire the conditional adds of the actions found so far.
            loop {
                let count = reachable.len();
                pending.retain(|(conds, atom)| {
                    if reachable.contains(atom) {
                        false
                    } else if holds_relaxed(conds, &reachable) {
                        reachable.insert(atom.clone());
                        false
                    } else {
                        true
                    }
                });
                if reachable.len() == count {
                    break;
                }
            }
        }
        ops.sort_by(|(a, pa, _), (b, pb, _)| (a, pa).cmp(&(b, pb)));
        Ok((
            ops.into_iter().map(|(_, _, op)| op).collect(),
            reachable.into_iter().collect(),
        ))
    }

    /// Adds every grounding of `schema` that is not in `emitted` and whose
    /// static preconditions hold and positive fluent preconditions are
    /// `reachable`, with the indices of its objects.
    fn instantiate(
        &self,
        schema: &Schema,
        reachable: &HashSet<Atom>,
        emitted: &mut HashSet<Vec<usize>>,
        out: &mut Vec<(Vec<usize>, GroundOp)>,
    ) -> Result<(), GroundError> {
        // Check each literal as soon as its last parameter is bound.
        let mut checks: Vec<Vec<&SLit>> = vec![Vec::new(); schema.params.len()];
        for lit in &schema.pre {
            let checked = match lit {
                SLit::Eq(..) => true,
                SLit::Atom(atom, positive) => {
                    *positive || !self.fluent.contains(atom.pred.as_str())
                }
            };
            if checked {
                match lit.last_param() {
                    Some(i) => checks[i].push(lit),
                    None if !self.relaxed(lit, &[], reachable) => return Ok(()),
                    None => {}
                }
            }
        }
        let mut search = Search {
            schema,
            checks: &checks,
            reachable,
            emitted,
            binding: Vec::with_capacity(schema.params.len()),
            path: Vec::with_capacity(schema.params.len()),
            out,
        };
        self.bind(&mut search)
    }

    /// Whether `lit` can hold in the delete relaxation: a static literal that
    /// holds, or a positive fluent atom that is `reachable`. Negative fluent
    /// literals are ignored, as the relaxation assumes they can hold.
    fn relaxed(&self, lit: &SLit, binding: &[String], reachable: &HashSet<Atom>) -> bool {
        match (self.eval_static(lit, binding), lit) {
            (Some(holds), _) => holds,
            (None, SLit::Atom(atom, true)) => reachable.contains(&Self::atom(atom, binding)),
            (None, _) => true,
        }
    }

    /// Binds the next parameter, recursively.
    fn bind(&self, search: &mut Search<'_, '_>) -> Result<(), GroundError> {
        let depth = search.binding.len();
        if depth == search.schema.params.len() {
            if search.emitted.insert(search.path.clone()) {
                let mut found = Vec::new();
                self.emit(search.schema, &search.binding, &mut found)?;
                search
                    .out
                    .extend(found.into_iter().map(|op| (search.path.clone(), op)));
            }
            return Ok(());
        }
        for (i, object) in search.schema.params[depth].iter().enumerate() {
            search.binding.push(object.clone());
            search.path.push(i);
            if search.checks[depth]
                .iter()
                .all(|lit| self.relaxed(lit, &search.binding, search.reachable))
            {
                self.bind(search)?;
            }
            search.binding.pop();
            search.path.pop();
        }
        Ok(())
    }

    /// Adds the ground action for a complete binding.
    fn emit(
        &self,
        schema: &Schema,
        binding: &[String],
        out: &mut Vec<GroundOp>,
    ) -> Result<(), GroundError> {
        let Some(pre) = self.fluent_lits(&schema.pre, binding) else {
            return Ok(());
        };
        let name = std::iter::once(schema.name.as_str())
            .chain(binding[..schema.arity].iter().map(String::as_str))
            .collect::<Vec<_>>()
            .join(" ");
        let effects = schema
            .effects
            .iter()
            .filter_map(|effect| {
                Some(GEffect {
                    conds: self.fluent_lits(&effect.conds, binding)?,
                    atom: Self::atom(&effect.atom, binding),
                    add: effect.add,
                })
            })
            .collect();
        let cost = if self.use_metric {
            self.cost(schema, binding)
                .ok_or_else(|| GroundError::InvalidCost {
                    action: name.clone(),
                })?
        } else {
            Decimal::whole(1)
        };
        out.push(GroundOp {
            name,
            pre,
            effects,
            cost,
        });
        Ok(())
    }

    /// Sums the cost of a ground action exactly, or `None` if a summand is
    /// negative or refers to a function without a value.
    fn cost(&self, schema: &Schema, binding: &[String]) -> Option<Decimal> {
        let mut total = Decimal::whole(0);
        for term in &schema.cost {
            let value = match term {
                CostTerm::Number(n) => *n,
                CostTerm::Function(symbol, args) => *self.functions.get(&(
                    symbol.clone(),
                    args.iter().map(|a| ground_arg(a, binding)).collect(),
                ))?,
            };
            total = total.checked_add(Decimal::from_f64(value)?)?;
        }
        Some(total)
    }
}

/// The object an argument stands for under a binding.
fn ground_arg(arg: &Arg, binding: &[String]) -> String {
    match arg {
        Arg::Var(i) => binding[*i].clone(),
        Arg::Obj(object) => object.clone(),
    }
}

/// Whether a conjunction of fluent literals can hold in the delete relaxation:
/// each positive atom is `reachable`.
fn holds_relaxed(lits: &[GLit], reachable: &HashSet<Atom>) -> bool {
    lits.iter()
        .all(|(atom, pos)| !pos || reachable.contains(atom))
}

/// Resolves a ground literal to a fact on a binary variable: `Ok(None)` if it
/// trivially holds (a negated unreachable atom), `Err(())` if it can never
/// hold (an unreachable atom).
fn resolve(index: &HashMap<&Atom, VarId>, (atom, positive): &GLit) -> Result<Option<Fact>, ()> {
    match (index.get(atom), positive) {
        (Some(&var), true) => Ok(Some(Fact::new(var, TRUE))),
        (Some(&var), false) => Ok(Some(Fact::new(var, FALSE))),
        (None, true) => Err(()),
        (None, false) => Ok(None),
    }
}

/// Resolves a conjunction to sorted facts, or `None` if it can never hold.
fn resolve_all(index: &HashMap<&Atom, VarId>, lits: &[GLit]) -> Option<BTreeMap<VarId, Value>> {
    let mut facts = BTreeMap::new();
    for lit in lits {
        if let Some(fact) = resolve(index, lit).ok()? {
            if *facts.entry(fact.var).or_insert(fact.value) != fact.value {
                return None;
            }
        }
    }
    Some(facts)
}

/// Converts a map of variable values to facts.
fn facts(map: &BTreeMap<VarId, Value>) -> Vec<Fact> {
    map.iter()
        .map(|(&var, &value)| Fact::new(var, value))
        .collect()
}

/// Splits the conjunction `conds` into conjunctions that together hold
/// exactly where `conds` holds and `other` does not. The facts of each piece
/// stay sorted.
fn unless(conds: Vec<Fact>, other: &[Fact]) -> Vec<Vec<Fact>> {
    let value = |var| conds.iter().find(|f| f.var == var).map(|f| f.value);
    if other
        .iter()
        .any(|f| value(f.var).is_some_and(|v| v != f.value))
    {
        return vec![conds]; // the two never hold together
    }
    other
        .iter()
        .filter(|f| value(f.var).is_none())
        .map(|f| {
            let negated = Fact::new(f.var, if f.value == TRUE { FALSE } else { TRUE });
            let mut piece = conds.clone();
            piece.push(negated);
            piece.sort();
            piece
        })
        .collect()
}

/// Drops every conjunction that implies another one in `pieces`, keeping their
/// disjunction the same. Each conjunction's facts must be sorted.
fn drop_subsumed(mut pieces: Vec<Vec<Fact>>) -> Vec<Vec<Fact>> {
    pieces.sort_by_key(Vec::len);
    pieces.dedup();
    let mut kept: Vec<Vec<Fact>> = Vec::with_capacity(pieces.len());
    for piece in pieces {
        // A shorter or equal kept piece whose facts all occur in `piece` covers it.
        if !kept
            .iter()
            .any(|k| k.iter().all(|f| piece.binary_search(f).is_ok()))
        {
            kept.push(piece);
        }
    }
    kept
}

impl GroundOp {
    /// Converts the action to a SAS+ operator, or `None` if its precondition
    /// can never hold.
    ///
    /// # Errors
    /// [`GroundError::ConflictingEffects`] if restricting a delete to where no
    /// add of the same atom fires needs more than [`MAX_DELETE_PIECES`] effects.
    fn to_sas(
        &self,
        index: &HashMap<&Atom, VarId>,
        cost: u32,
    ) -> Result<Option<SasOperator>, GroundError> {
        let Some(pre) = resolve_all(index, &self.pre) else {
            return Ok(None);
        };
        let effects: Vec<(Vec<Fact>, VarId, Value)> = self
            .effects
            .iter()
            .filter_map(|effect| {
                let var = *index.get(&effect.atom)?; // deleting an unreachable atom does nothing
                let conds = resolve_all(index, &effect.conds)?;
                Some((facts(&conds), var, if effect.add { TRUE } else { FALSE }))
            })
            .collect();
        // PDDL applies deletes before adds, so an add wins over a delete of the
        // same atom: restrict each delete to the states where no add fires.
        let adds: Vec<(Vec<Fact>, VarId)> = effects
            .iter()
            .filter(|(_, _, post)| *post == TRUE)
            .map(|(conds, var, _)| (conds.clone(), *var))
            .collect();
        let mut split = Vec::with_capacity(effects.len());
        for (conds, var, post) in effects {
            let mut pieces = vec![conds];
            if post == FALSE {
                for (add, _) in adds.iter().filter(|(_, v)| *v == var) {
                    pieces =
                        drop_subsumed(pieces.into_iter().flat_map(|c| unless(c, add)).collect());
                    if pieces.len() > MAX_DELETE_PIECES {
                        return Err(GroundError::ConflictingEffects {
                            action: self.name.clone(),
                        });
                    }
                }
            }
            split.extend(pieces.into_iter().map(|c| (c, var, post)));
        }
        let mut effects = split;
        effects.sort();
        effects.dedup();
        // Like Fast Downward, drop effects that cannot change the state, and
        // operators left without effects.
        effects.retain(|(conds, var, post)| !(conds.is_empty() && pre.get(var) == Some(post)));
        if effects.is_empty() {
            return Ok(None);
        }

        let affected: HashSet<VarId> = effects.iter().map(|(_, var, _)| *var).collect();
        Ok(Some(SasOperator {
            name: self.name.clone(),
            prevail: pre
                .iter()
                .filter(|(var, _)| !affected.contains(var))
                .map(|(&var, &value)| Fact::new(var, value))
                .collect(),
            effects: effects
                .into_iter()
                .map(|(conditions, var, post)| SasEffect {
                    conditions,
                    var,
                    pre: pre.get(&var).copied(),
                    post,
                })
                .collect(),
            cost,
        }))
    }
}

/// Grounds the goal, or fails if no reachable state satisfies it.
fn ground_goal(
    lits: &[SLit],
    grounder: &Grounder<'_>,
    index: &HashMap<&Atom, VarId>,
) -> Result<Vec<Fact>, GroundError> {
    let fluent = grounder
        .fluent_lits(lits, &[])
        .ok_or(GroundError::Unsolvable)?;
    resolve_all(index, &fluent)
        .map(|goal| facts(&goal))
        .ok_or(GroundError::Unsolvable)
}

#[cfg(all(test, feature = "parser"))]
mod tests {
    use super::*;
    use crate::Parser;

    fn ground_str(domain: &str, problem: &str) -> Result<SasFile, GroundError> {
        ground(
            &Domain::from_str(domain).unwrap(),
            &Problem::from_str(problem).unwrap(),
        )
    }

    #[test]
    fn either_typing_and_supertypes() {
        let file = ground_str(
            "(define (domain d) (:requirements :typing)
               (:types car bike - vehicle boat)
               (:predicates (ready ?v))
               (:action prep :parameters (?v - (either vehicle boat)) :effect (ready ?v)))",
            "(define (problem p) (:domain d) (:objects c - car b - bike s - boat x)
               (:init) (:goal (ready c)))",
        )
        .unwrap();
        let names: Vec<_> = file.operators.iter().map(|op| op.name.as_str()).collect();
        assert_eq!(names, ["prep c", "prep b", "prep s"]);
    }

    #[test]
    fn constants_and_negative_preconditions() {
        let file = ground_str(
            "(define (domain d) (:requirements :negative-preconditions)
               (:constants home) (:predicates (at ?x) (locked))
               (:action go :parameters (?x) :precondition (and (not (locked)) (not (= ?x home)))
                  :effect (and (at home) (not (at ?x)))))",
            "(define (problem p) (:domain d) (:objects away) (:init (at away)) (:goal (at home)))",
        )
        .unwrap();
        assert_eq!(file.operators.len(), 1);
        let op = &file.operators[0];
        assert_eq!(op.name, "go away");
        // `locked` is static and false, so `(not (locked))` is evaluated away.
        assert_eq!(op.prevail, [] as [sas_plus::Fact; 0]);
        assert_eq!(file.variables.len(), 2);
        assert_eq!(file.to_string().parse::<SasFile>().unwrap(), file);
    }

    #[test]
    fn rejects_or() {
        let err = ground_str(
            "(define (domain d) (:predicates (p) (q))
               (:action a :parameters () :precondition (or (p) (q)) :effect (p)))",
            "(define (problem p) (:domain d) (:init) (:goal (p)))",
        );
        assert_eq!(err, Err(GroundError::Unsupported("or")));
    }

    #[test]
    fn static_false_goal_is_unsolvable() {
        let err = ground_str(
            "(define (domain d) (:predicates (p) (q))
               (:action a :parameters () :effect (p)))",
            "(define (problem p) (:domain d) (:init) (:goal (and (p) (q))))",
        );
        assert_eq!(err, Err(GroundError::Unsolvable));
    }
}
