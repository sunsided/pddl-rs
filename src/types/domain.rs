//! Contains the [`Domain`] type.

use crate::types::{
    ConGD, Constants, Functions, PredicateDefinitions, Requirements, StructureDefs,
};
use crate::types::{Name, Types};

/// The `Domain` type specifies a problem domain in which to plan.
#[derive(Debug, Clone, PartialEq)]
pub struct Domain<'a> {
    name: Name<'a>,
    requirements: Requirements,
    /// Requires [Typing](crate::types::Requirement::Typing).
    types: Types<'a>,
    constants: Constants<'a>,
    predicates: PredicateDefinitions<'a>,
    /// Requires [Fluents](crate::types::Requirement::Fluents).
    functions: Functions<'a>,
    /// Requires [Constraints](crate::types::Requirement::Constraints).
    constraints: ConGD<'a>,
    structure: StructureDefs<'a>,
}

impl<'a> Domain<'a> {
    pub fn new(
        name: Name<'a>,
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
            requirements,
            types,
            constants,
            predicates,
            functions,
            constraints,
            structure,
        }
    }

    /// Gets the domain name.
    pub const fn name(&self) -> &Name<'a> {
        &self.name
    }

    /// Returns the optional domain requirements.
    /// If no requirements were specified by the domain, [Strips](crate::types::Requirement::Strips) is implied.
    pub const fn requirements(&self) -> &Requirements {
        &self.requirements
    }

    /// Returns the optional type declarations.
    /// Requires [Typing](crate::types::Requirement::Typing).
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
    /// Requires [Fluents](crate::types::Requirement::Fluents).
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
