//! Contains the [`Domain`] type.

use crate::types::{
    ConGD, Constants, DomainConstraintsDef, Functions, PredicateDefinitions, Requirements,
    StructureDefs, Timeless,
};
use crate::types::{Name, Types};

/// The `Domain` type specifies a problem domain in which to plan.
///
/// ## Usage
/// This is the top-level type of a domain description. See also [`Problem`](crate::Problem).
///
/// ## Example
/// ```
/// # use pddl::{Domain, Name, Parser};
/// let input = r#"(define (domain briefcase-world)
///       (:requirements :strips :equality :typing :conditional-effects)
///       (:types location physob)
///       (:constants
///             B ; the briefcase
///             P ; the paycheck
///             D
///             - physob)
///       (:predicates (at ?x - physob ?y - location) ; an item is at a location
///                    (in ?x ?y - physob))           ; an item is in another item
///       (:constraints (and))
///
///       ; Move briefcase from one location to another.
///       (:action mov-B
///            :parameters (?m ?l - location)
///            :precondition (and (at B ?m) (not (= ?m ?l)))
///            :effect (and (at B ?l) (not (at B ?m))
///                         (forall (?z)
///                             (when (and (in ?z) (not (= ?z B)))
///                                   (and (at ?z ?l) (not (at ?z ?m)))))) )
///
///       ; Put the item in the briefcase.
///       (:action put-in
///            :parameters (?x - physob ?l - location)
///            :precondition (not (= ?x B))  ; the item must not be the briefcase itself
///            :effect (when (and (at ?x ?l) (at B ?l))
///                  (in ?x)) )
///
///       ; Take the item out of the briefcase.
///       (:action take-out
///            :parameters (?x - physob)
///            :precondition (not (= ?x B)) ; the item must be the briefcase itself
///            :effect (not (in ?x)) )
///     )"#;
///
/// let domain = Domain::from_str(input).unwrap();
///
/// assert_eq!(domain.name(), "briefcase-world");
/// assert_eq!(domain.requirements().len(), 4);
/// assert_eq!(domain.types().len(), 2);
/// assert_eq!(domain.constants().len(), 3);
/// assert_eq!(domain.predicates().len(), 2);
/// assert!(domain.constraints().is_empty());
/// assert_eq!(domain.structure().len(), 3);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Domain {
    /// The domain name.
    name: Name,
    /// The domain extension list.
    // TODO: PDDL 1.2 - deprecated?
    extends: Vec<Name>,
    /// The specified requirements.
    requirements: Requirements,
    /// The optional type declarations.
    ///
    /// ## Requirements
    /// Requires [Typing](crate::Requirement::Typing).
    types: Types,
    /// The optional constant declarations.
    constants: Constants,
    /// The predicate definitions.
    predicates: PredicateDefinitions,
    /// The optional function definitions.
    ///
    /// ## Requirements
    /// Requires [Fluents](crate::Requirement::Fluents).
    functions: Functions,
    /// The optional constraint definitions.
    ///
    /// ## Requirements
    /// Requires [Constraints](crate::Requirement::Constraints).
    constraints: DomainConstraintsDef,
    /// The optional timeless predicate definitions.
    // TODO: PDDL 1.2 - deprecated?
    timeless: Timeless,
    /// The structure definition, i.e. [action](crate::ActionDefinition),
    /// [durative action](crate::DurativeActionDefinition), and/or
    /// [derived predicate](crate::DerivedPredicate) definitions.
    structure: StructureDefs,
}

impl Domain {
    /// Creates a builder to easily construct [`Domain`] instances.
    pub fn builder(name: Name, structure: StructureDefs) -> Self {
        Self {
            name,
            extends: Vec::default(),
            requirements: Requirements::default(),
            types: Types::default(),
            constants: Constants::default(),
            predicates: PredicateDefinitions::default(),
            functions: Functions::default(),
            constraints: DomainConstraintsDef::default(),
            timeless: Timeless::default(),
            structure,
        }
    }

    /// Adds a list of optional domain names this domain definition extends upon.
    /// This is a PDDL 1.2 construct.
    pub fn with_extends<N: IntoIterator<Item = Name>>(mut self, names: N) -> Self {
        self.extends = names.into_iter().collect();
        self
    }

    /// Adds a list of optional domain requirements.
    pub fn with_requirements(mut self, requirements: Requirements) -> Self {
        self.requirements = requirements;
        self
    }

    /// Adds a list of optional type declarations.
    pub fn with_types<T: Into<Types>>(mut self, types: T) -> Self {
        self.types = types.into();
        self
    }

    /// Adds a list of optional constant declarations.
    pub fn with_constants<C: Into<Constants>>(mut self, constants: C) -> Self {
        self.constants = constants.into();
        self
    }

    /// Adds a list of optional predicate definitions.
    pub fn with_predicates<P: Into<PredicateDefinitions>>(mut self, predicates: P) -> Self {
        self.predicates = predicates.into();
        self
    }

    /// Adds a list of optional function definitions.
    pub fn with_functions<F: Into<Functions>>(mut self, functions: F) -> Self {
        self.functions = functions.into();
        self
    }

    /// Adds a list of optional constraints.
    pub fn with_constraints(mut self, constraints: DomainConstraintsDef) -> Self {
        self.constraints = constraints;
        self
    }

    /// Adds a list of timeless predicates.
    pub fn with_timeless(mut self, timeless: Timeless) -> Self {
        self.timeless = timeless;
        self
    }

    /// Gets the domain name.
    pub const fn name(&self) -> &Name {
        &self.name
    }

    /// Gets the names of the domains this definition extends.
    /// This is a PDDL 1.2 construct.
    pub fn extends(&self) -> &[Name] {
        self.extends.as_slice()
    }

    /// Returns the optional domain requirements.
    /// If no requirements were specified by the domain, [STRIPS](crate::Requirement::Strips) is implied.
    pub const fn requirements(&self) -> &Requirements {
        &self.requirements
    }

    /// Returns the optional type declarations.
    /// ## Requirements
    /// Requires [Typing](crate::Requirement::Typing).
    pub const fn types(&self) -> &Types {
        &self.types
    }

    /// Returns the optional constant definitions.
    pub const fn constants(&self) -> &Constants {
        &self.constants
    }

    /// Returns the optional predicate definitions.
    pub const fn predicates(&self) -> &PredicateDefinitions {
        &self.predicates
    }

    /// Returns the optional function definitions.
    /// ## Requirements
    /// Requires [Fluents](Requirement::Fluents).
    pub const fn functions(&self) -> &Functions {
        &self.functions
    }

    /// Returns the optional constraint declaration.
    pub const fn constraints(&self) -> &ConGD {
        self.constraints.value()
    }

    /// Returns the domain structure definitions.
    pub const fn structure(&self) -> &StructureDefs {
        &self.structure
    }
}

impl AsRef<Requirements> for Domain {
    fn as_ref(&self) -> &Requirements {
        &self.requirements
    }
}

impl AsRef<Types> for Domain {
    fn as_ref(&self) -> &Types {
        &self.types
    }
}

impl AsRef<PredicateDefinitions> for Domain {
    fn as_ref(&self) -> &PredicateDefinitions {
        &self.predicates
    }
}

impl AsRef<Functions> for Domain {
    fn as_ref(&self) -> &Functions {
        &self.functions
    }
}

impl AsRef<StructureDefs> for Domain {
    fn as_ref(&self) -> &StructureDefs {
        &self.structure
    }
}
