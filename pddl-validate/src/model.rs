//! The checked form of a domain and problem that plans run against.
//!
//! [`Model::new`] indexes the domain's symbols and objects, [`check`] then checks every
//! formula against them, and [`crate::derived::stratify`] orders the derived-predicate
//! rules into strata.

pub mod check;
pub mod universe;

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::vec::Vec;

use pddl::{
    ActionDefinition, Domain, DurativeActionDefinition, GoalDescription, InitElement, Problem,
    ProcessDefinition, StructureDef, TypedVariables,
};

use crate::atom::lower;
use crate::error::{ModelError, ModelErrorKind};
use crate::report::{Construct, Unsupported};
use universe::Universe;

/// Events closer in time than this belong to one happening.
pub const SAME_TIME: f64 = 1e-9;

/// The indexed domain and problem. All names are lowercased.
#[derive(Debug)]
pub struct Model<'a> {
    /// The objects and their types.
    pub universe: Universe,
    /// Every predicate, derived ones included, with its number of parameters.
    pub predicates: BTreeMap<String, usize>,
    /// Every function with its number of parameters.
    pub functions: BTreeMap<String, usize>,
    /// The actions by name.
    pub actions: BTreeMap<String, &'a ActionDefinition>,
    /// The durative actions by name. Actions, durative actions, processes and events
    /// have distinct names.
    pub durative: BTreeMap<String, &'a DurativeActionDefinition>,
    /// The PDDL+ processes by name.
    pub processes: BTreeMap<String, &'a ProcessDefinition>,
    /// The PDDL+ events by name.
    pub events: BTreeMap<String, &'a ActionDefinition>,
    /// The derived-predicate rules, sorted by stratum once [`crate::derived::stratify`] ran.
    pub rules: Vec<Rule<'a>>,
    /// The names of the derived predicates.
    pub derived: BTreeSet<String>,
}

/// A derived-predicate rule, `(:derived (p ?x) body)`.
#[derive(Debug)]
pub struct Rule<'a> {
    /// The derived predicate, lowercased.
    pub predicate: String,
    /// The parameters of the head.
    pub parameters: &'a TypedVariables,
    /// The condition under which the head holds.
    pub body: &'a GoalDescription,
    /// The stratum: a rule reads only atoms of lower strata under negation. 0 until
    /// [`crate::derived::stratify`] runs.
    pub stratum: usize,
}

impl<'a> Model<'a> {
    /// Indexes the symbols, actions and objects of `domain` and `problem`.
    ///
    /// # Errors
    ///
    /// [`ModelErrorKind::DuplicateAction`] for two actions of one name, and the errors of
    /// [`Universe::new`].
    pub fn new(domain: &'a Domain, problem: &'a Problem) -> Result<Self, ModelError> {
        let universe = Universe::new(domain, problem)?;
        let mut predicates: BTreeMap<String, usize> = domain
            .predicates()
            .iter()
            .map(|p| (lower(p.name()), p.variables().len()))
            .collect();
        let functions = domain
            .functions()
            .iter()
            .map(|f| (lower(f.symbol()), f.variables().len()))
            .collect();
        let mut actions = BTreeMap::new();
        let mut durative = BTreeMap::new();
        let mut processes = BTreeMap::new();
        let mut events = BTreeMap::new();
        let mut names = BTreeSet::new();
        let mut rules = Vec::new();
        for def in domain.structure().iter() {
            // Rules of one derived predicate add up, so only they share names.
            let symbol: Option<&str> = match def {
                StructureDef::Action(action) | StructureDef::Event(action) => Some(action.symbol()),
                StructureDef::DurativeAction(action) => Some(action.symbol()),
                StructureDef::Process(process) => Some(process.symbol()),
                StructureDef::Derived(_) => None,
            };
            let name = symbol.map(lower).unwrap_or_default();
            if symbol.is_some() && !names.insert(name.clone()) {
                return Err(ModelError::new(
                    "domain",
                    ModelErrorKind::DuplicateAction(name),
                ));
            }

            match def {
                StructureDef::Action(action) => {
                    actions.insert(name, action);
                }
                StructureDef::Event(event) => {
                    events.insert(name, event);
                }
                StructureDef::DurativeAction(action) => {
                    durative.insert(name, &**action);
                }
                StructureDef::Process(process) => {
                    processes.insert(name, &**process);
                }
                StructureDef::Derived(rule) => {
                    let head: &pddl::AtomicFormulaSkeleton = rule.as_ref();
                    let predicate = lower(head.name());
                    predicates.insert(predicate.clone(), head.variables().len());
                    rules.push(Rule {
                        predicate,
                        parameters: head.variables(),
                        body: rule.as_ref(),
                        stratum: 0,
                    });
                }
            }
        }

        let derived = rules.iter().map(|rule| rule.predicate.clone()).collect();

        Ok(Self {
            universe,
            predicates,
            functions,
            actions,
            durative,
            processes,
            events,
            rules,
            derived,
        })
    }
}

impl Model<'_> {
    /// Returns `true` if plans for this task run on the temporal timeline whatever their
    /// form: the domain has durative actions, processes or events, or the problem timed
    /// initial literals.
    pub fn is_temporal(&self, problem: &Problem) -> bool {
        !self.durative.is_empty()
            || !self.processes.is_empty()
            || !self.events.is_empty()
            || problem
                .init()
                .iter()
                .any(|element| matches!(element, InitElement::At(..)))
    }

    /// Returns `true` if a metric's `(symbol)` with `arity` arguments means `total-time`.
    ///
    /// The parser reads `(total-time)` as a function application; it means the plan's
    /// duration unless the domain declares a function of that name.
    pub fn is_total_time(&self, symbol: &str, arity: usize) -> bool {
        arity == 0
            && symbol.eq_ignore_ascii_case("total-time")
            && !self.functions.contains_key("total-time")
    }
}

/// Checks that `problem` is for `domain`.
///
/// # Errors
///
/// [`ModelErrorKind::DomainMismatch`] if the problem names another domain.
pub fn check_domain_name(domain: &Domain, problem: &Problem) -> Result<(), ModelError> {
    if domain.name() == problem.domain() {
        return Ok(());
    }

    let kind = ModelErrorKind::DomainMismatch {
        domain: lower(domain.name()),
        problem_domain: lower(problem.domain()),
    };

    Err(ModelError::new("problem", kind))
}

/// Returns the first section of the domain or problem that uses a construct the
/// validator does not interpret, judging by the sections present rather than by the
/// declared requirements: `:fluents`, for one, also declares object fluents.
pub fn unsupported_sections(domain: &Domain, problem: &Problem) -> Option<Unsupported> {
    let requirements = domain.requirements().unknown().iter();
    let mut unknown = requirements.chain(problem.requirements().unknown());
    // PDDL+'s `:time`, which the parser does not know, is interpreted.
    if let Some(name) = unknown.find(|name| !name.eq_ignore_ascii_case(":time")) {
        let construct = Construct::UnknownRequirement(lower(name));
        return Some(Unsupported::new(construct, "requirements"));
    }

    let found = [
        (!domain.extends().is_empty(), Construct::DomainExtension),
        (!domain.timeless().is_empty(), Construct::Timeless),
        (!domain.constraints().is_empty(), Construct::Constraints),
        (!problem.constraints().is_empty(), Construct::Constraints),
    ];
    found
        .into_iter()
        .find(|(present, _)| *present)
        .map(|(_, construct)| Unsupported::new(construct, "domain or problem"))
}
