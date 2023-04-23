//! Contains the [`Domain`] type.

use crate::types::{
    ConGD, Constants, Functions, PredicateDefinitions, Requirement, Requirements, StructureDefs,
};
use crate::types::{Name, Types};

/// The `Domain` type specifies a problem domain in which to plan.
#[derive(Debug, Clone, PartialEq)]
pub struct Domain<'a> {
    name: Name<'a>,
    /// PDDL 1.2
    extends: Vec<Name<'a>>,
    requirements: Requirements,
    /// Requires [Typing](Requirement::Typing).
    types: Types<'a>,
    constants: Constants<'a>,
    predicates: PredicateDefinitions<'a>,
    /// Requires [Fluents](Requirement::Fluents).
    functions: Functions<'a>,
    /// Requires [Constraints](Requirement::Constraints).
    constraints: ConGD<'a>,
    structure: StructureDefs<'a>,
}

impl<'a> Domain<'a> {
    pub fn new(
        name: Name<'a>,
        extends: Vec<Name<'a>>,
        requirements: Requirements,
        types: Types<'a>,
        constants: Constants<'a>,
        predicates: PredicateDefinitions<'a>,
        functions: Functions<'a>,
        constraints: ConGD<'a>,
        structure: StructureDefs<'a>,
    ) -> Self {
        Self {
            name,
            extends,
            requirements,
            types,
            constants,
            predicates,
            functions,
            constraints,
            structure,
        }
    }

    /// Creates a builder to easily construct problems.
    pub fn builder(name: Name<'a>, structure: StructureDefs<'a>) -> Self {
        Self {
            name,
            extends: Vec::default(),
            requirements: Requirements::default(),
            types: Types::default(),
            constants: Constants::default(),
            predicates: PredicateDefinitions::default(),
            functions: Functions::default(),
            constraints: ConGD::default(),
            structure,
        }
    }

    /// Adds a list of optional domain names this domain definition extends upon.
    /// This is a PDDL 1.2 construct.
    pub fn with_extends<N: IntoIterator<Item = Name<'a>>>(mut self, names: N) -> Self {
        self.extends = names.into_iter().collect();
        self
    }

    /// Adds a list of optional domain requirements.
    pub fn with_requirements<R: IntoIterator<Item = Requirement>>(
        mut self,
        requirements: R,
    ) -> Self {
        self.requirements = requirements.into_iter().collect();
        self
    }

    /// Adds a list of optional type declarations.
    pub fn with_types<T: Into<Types<'a>>>(mut self, types: T) -> Self {
        self.types = types.into();
        self
    }

    /// Adds a list of optional constant declarations.
    pub fn with_constants<C: Into<Constants<'a>>>(mut self, constants: C) -> Self {
        self.constants = constants.into();
        self
    }

    /// Adds a list of optional predicate definitions.
    pub fn with_predicates<P: Into<PredicateDefinitions<'a>>>(mut self, predicates: P) -> Self {
        self.predicates = predicates.into();
        self
    }

    /// Adds a list of optional function definitions.
    pub fn with_functions<F: Into<Functions<'a>>>(mut self, functions: F) -> Self {
        self.functions = functions.into();
        self
    }

    /// Adds a list of optional constraints.
    pub fn with_constraints(mut self, constraints: ConGD<'a>) -> Self {
        self.constraints = constraints;
        self
    }

    /// Gets the domain name.
    pub const fn name(&self) -> &Name<'a> {
        &self.name
    }

    /// Gets the names of the domains this definition extends.
    /// This is a PDDL 1.2 construct.
    pub fn extends(&self) -> &[Name<'a>] {
        &self.extends.as_slice()
    }

    /// Returns the optional domain requirements.
    /// If no requirements were specified by the domain, [Strips](Requirement::Strips) is implied.
    pub const fn requirements(&self) -> &Requirements {
        &self.requirements
    }

    /// Returns the optional type declarations.
    /// Requires [Typing](Requirement::Typing).
    pub const fn types(&self) -> &Types<'a> {
        &self.types
    }

    /// Returns the optional constant definitions.
    pub const fn constants(&self) -> &Constants<'a> {
        &self.constants
    }

    /// Returns the optional predicate definitions.
    pub const fn predicates(&self) -> &PredicateDefinitions<'a> {
        &self.predicates
    }

    /// Returns the optional function definitions.
    /// Requires [Fluents](Requirement::Fluents).
    pub const fn functions(&self) -> &Functions<'a> {
        &self.functions
    }

    /// Returns the optional constraint declaration.
    pub const fn constraints(&self) -> &ConGD<'a> {
        &self.constraints
    }

    /// Returns the domain structure definitions.
    pub const fn structure(&self) -> &StructureDefs<'a> {
        &self.structure
    }
}

impl<'a> AsRef<Requirements> for Domain<'a> {
    fn as_ref(&self) -> &Requirements {
        &self.requirements
    }
}

impl<'a> AsRef<Types<'a>> for Domain<'a> {
    fn as_ref(&self) -> &Types<'a> {
        &self.types
    }
}

impl<'a> AsRef<PredicateDefinitions<'a>> for Domain<'a> {
    fn as_ref(&self) -> &PredicateDefinitions<'a> {
        &self.predicates
    }
}

impl<'a> AsRef<Functions<'a>> for Domain<'a> {
    fn as_ref(&self) -> &Functions<'a> {
        &self.functions
    }
}

impl<'a> AsRef<StructureDefs<'a>> for Domain<'a> {
    fn as_ref(&self) -> &StructureDefs<'a> {
        &self.structure
    }
}
