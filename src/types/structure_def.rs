//! Contains definitions for domain structure definitions.

use crate::types::{ActionDefinition, DerivedPredicate, DurativeActionDefinition};

/// A domain structure definition.
///
/// ## Usage
/// Used by [`StructureDefs`](crate::StructureDefs) in [`Domain`](crate::Domain).
#[derive(Debug, Clone, PartialEq)]
pub enum StructureDef {
    Action(ActionDefinition),
    /// ## Requirements
    /// Requires [Durative Actions](crate::Requirement::DurativeActions).
    DurativeAction(DurativeActionDefinition),
    /// ## Requirements
    /// Requires [Derived Predicates](crate::Requirement::DerivedPredicates).
    Derived(DerivedPredicate),
}

impl StructureDef {
    pub const fn new_action(action: ActionDefinition) -> Self {
        Self::Action(action)
    }
    pub const fn new_durative_action(action: DurativeActionDefinition) -> Self {
        Self::DurativeAction(action)
    }
    pub const fn new_derived(predicate: DerivedPredicate) -> Self {
        Self::Derived(predicate)
    }
}

impl From<ActionDefinition> for StructureDef {
    fn from(value: ActionDefinition) -> Self {
        StructureDef::new_action(value)
    }
}

impl From<DurativeActionDefinition> for StructureDef {
    fn from(value: DurativeActionDefinition) -> Self {
        StructureDef::new_durative_action(value)
    }
}

impl From<DerivedPredicate> for StructureDef {
    fn from(value: DerivedPredicate) -> Self {
        StructureDef::new_derived(value)
    }
}
