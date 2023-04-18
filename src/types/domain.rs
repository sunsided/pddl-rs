use crate::types::{Functions, PredicateDefinitions, Requirements, StructureDef, Types};

#[derive(Debug, Clone, PartialEq)]
pub struct Domain<'a> {
    name: &'a str,
    requirements: Requirements,
    /// Requires [Typing](Requirement::Typing).
    types: Types<'a>,
    predicates: PredicateDefinitions<'a>,
    /// Requires [Fluents](Requirement::Fluents).
    functions: Functions<'a>,
    // TODO: add constraints (:constraints requirement)
    structure: StructureDef<'a>,
}

impl<'a> Domain<'a> {
    pub fn new(
        name: &'a str,
        requirements: Requirements,
        types: Types<'a>,
        predicates: PredicateDefinitions<'a>,
        functions: Functions<'a>,
        structure: StructureDef<'a>,
    ) -> Self {
        Self {
            name,
            requirements,
            types,
            predicates,
            functions,
            structure,
        }
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn requirements(&self) -> &Requirements {
        &self.requirements
    }

    pub const fn types(&self) -> &Types<'a> {
        &self.types
    }

    pub const fn predicates(&self) -> &PredicateDefinitions<'a> {
        &self.predicates
    }

    pub const fn functions(&self) -> &Functions<'a> {
        &self.functions
    }

    pub const fn structure(&self) -> &StructureDef<'a> {
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

impl<'a> AsRef<StructureDef<'a>> for Domain<'a> {
    fn as_ref(&self) -> &StructureDef<'a> {
        &self.structure
    }
}
