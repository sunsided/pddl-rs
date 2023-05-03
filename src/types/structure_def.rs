//! Contains definitions for domain structure definitions.

use crate::types::{ActionDefinition, DerivedPredicate, DurativeActionDefinition};

/// A domain structure definition.
///
/// ## Usage
/// Used by [`StructureDefs`](crate::StructureDefs) in [`Domain`](crate::Domain).
#[derive(Debug, Clone, PartialEq)]
pub enum StructureDef<'a> {
    Action(ActionDefinition<'a>),
    /// ## Requirements
    /// Requires [Durative Actions](crate::Requirement::DurativeActions).
    DurativeAction(DurativeActionDefinition<'a>),
    /// ## Requirements
    /// Requires [Derived Predicates](crate::Requirement::DerivedPredicates).
    Derived(DerivedPredicate<'a>),
}

impl<'a> StructureDef<'a> {
    pub const fn new_action(action: ActionDefinition<'a>) -> Self {
        Self::Action(action)
    }
    pub const fn new_durative_action(action: DurativeActionDefinition<'a>) -> Self {
        Self::DurativeAction(action)
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

impl<'a> From<DurativeActionDefinition<'a>> for StructureDef<'a> {
    fn from(value: DurativeActionDefinition<'a>) -> Self {
        StructureDef::new_durative_action(value)
    }
}

impl<'a> From<DerivedPredicate<'a>> for StructureDef<'a> {
    fn from(value: DerivedPredicate<'a>) -> Self {
        StructureDef::new_derived(value)
    }
}
