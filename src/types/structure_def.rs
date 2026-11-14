//! Contains definitions for domain structure definitions.

use crate::types::{ActionDefinition, DerivedPredicate, DurativeActionDefinition};

/// A domain structure definition.
///
/// ## Usage
/// Used by [`StructureDefs`](crate::StructureDefs) in [`Domain`](crate::Domain).
#[derive(Debug, Clone, PartialEq)]
pub enum StructureDef {
    /// An action definition, `(:action ...)`.
    Action(ActionDefinition),
    /// ## Requirements
    /// Requires [Durative Actions](crate::Requirement::DurativeActions).
    DurativeAction(Box<DurativeActionDefinition>),
    /// ## Requirements
    /// Requires [Derived Predicates](crate::Requirement::DerivedPredicates).
    Derived(DerivedPredicate),
}

impl StructureDef {
    /// Creates a new action structure definition.
    #[doc(alias = "new_action")]
    pub const fn action(action: ActionDefinition) -> Self {
        Self::Action(action)
    }

    /// Creates a new action structure definition; same as [`action`](Self::action).
    pub fn new_action(action: ActionDefinition) -> Self {
        Self::action(action)
    }

    /// Creates a new durative action structure definition, boxing the action.
    #[doc(alias = "new_durative_action")]
    pub fn durative_action(action: DurativeActionDefinition) -> Self {
        Self::DurativeAction(Box::new(action))
    }

    /// Creates a new durative action structure definition; same as [`durative_action`](Self::durative_action).
    pub fn new_durative_action(action: DurativeActionDefinition) -> Self {
        Self::durative_action(action)
    }

    /// Creates a new derived predicate structure definition.
    #[doc(alias = "new_derived")]
    pub const fn derived(predicate: DerivedPredicate) -> Self {
        Self::Derived(predicate)
    }

    /// Creates a new derived predicate structure definition; same as [`derived`](Self::derived).
    pub fn new_derived(predicate: DerivedPredicate) -> Self {
        Self::derived(predicate)
    }
}

impl From<ActionDefinition> for StructureDef {
    fn from(value: ActionDefinition) -> Self {
        StructureDef::action(value)
    }
}

impl From<DurativeActionDefinition> for StructureDef {
    fn from(value: DurativeActionDefinition) -> Self {
        StructureDef::durative_action(value)
    }
}

impl From<DerivedPredicate> for StructureDef {
    fn from(value: DerivedPredicate) -> Self {
        StructureDef::derived(value)
    }
}
