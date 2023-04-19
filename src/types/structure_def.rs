//! Contains definitions for domain structure definitions.

use crate::types::{ActionDefinition, DerivedPredicate};

/// A domain structure definition.
#[derive(Debug, Clone, PartialEq)]
pub enum StructureDef<'a> {
    Action(ActionDefinition<'a>), // TODO: Add <durative-action-def> in :durative-actions
    /// Requires [DerivedPredicates](crate::types::Requirement::DerivedPredicates).
    Derived(DerivedPredicate<'a>),
}

impl<'a> StructureDef<'a> {
    pub const fn new_action(action: ActionDefinition<'a>) -> Self {
        Self::Action(action)
    }
    pub const fn new_derived(predicate: DerivedPredicate<'a>) -> Self {
        Self::Derived(predicate)
    }
}

impl<'a> From<ActionDefinition<'a>> for StructureDef<'a> {
    fn from(value: ActionDefinition<'a>) -> Self {
        StructureDef::new_action(value)
    }
}

impl<'a> From<DerivedPredicate<'a>> for StructureDef<'a> {
    fn from(value: DerivedPredicate<'a>) -> Self {
        StructureDef::new_derived(value)
    }
}
