//! The checked form of a domain and problem that plans run against.
//!
//! [`Model::new`] indexes the domain's symbols and objects, [`check`] then checks every
//! formula against them, and [`crate::derived::stratify`] orders the derived-predicate
//! rules into strata.

pub mod check;
pub mod universe;

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use pddl::{ActionDefinition, Domain, GoalDescription, Problem, StructureDef, TypedVariables};

use crate::atom::lower;
use crate::error::{ModelError, ModelErrorKind};
use crate::report::{Construct, Unsupported};
use universe::Universe;

/// The indexed domain and problem. All names are lowercased.
#[derive(Debug)]
pub struct Model<'a> {
    /// The objects and their types.
    pub universe: Universe,
    /// Every predicate, derived ones included, with its number of parameters.
    pub predicates: BTreeMap<String, usize>,
    /// The actions by name.
    pub actions: BTreeMap<String, &'a ActionDefinition>,
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
        let mut actions = BTreeMap::new();
        let mut rules = Vec::new();
        for def in domain.structure().iter() {
            match def {
                StructureDef::Action(action) => {
                    let name = lower(action.symbol());
                    if actions.insert(name.clone(), action).is_some() {
                        let kind = ModelErrorKind::DuplicateAction(name);
                        return Err(ModelError::new("domain", kind));
                    }
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
                // `unsupported_sections` reports these before a model is built.
                StructureDef::DurativeAction(_) => {}
            }
        }

        let derived = rules.iter().map(|rule| rule.predicate.clone()).collect();

        Ok(Self {
            universe,
            predicates,
            actions,
            rules,
            derived,
        })
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
    let unknown = requirements.chain(problem.requirements().unknown());
    if let Some(name) = unknown.into_iter().next() {
        let construct = Construct::UnknownRequirement(lower(name));
        return Some(Unsupported::new(construct, "requirements"));
    }

    let found = [
        (!domain.extends().is_empty(), Construct::DomainExtension),
        (!domain.timeless().is_empty(), Construct::Timeless),
        (!domain.constraints().is_empty(), Construct::Constraints),
        (!problem.constraints().is_empty(), Construct::Constraints),
    ];
    if let Some((_, construct)) = found.into_iter().find(|(present, _)| *present) {
        return Some(Unsupported::new(construct, "domain or problem"));
    }

    domain.structure().iter().find_map(|def| match def {
        StructureDef::DurativeAction(action) => Some(Unsupported::new(
            Construct::DurativeActions,
            format!("durative action `{}`", lower(action.symbol())),
        )),
        StructureDef::Action(_) | StructureDef::Derived(_) => None,
    })
}
