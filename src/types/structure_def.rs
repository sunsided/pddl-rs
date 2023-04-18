//! Contains definitions for domain structure definitions.

use crate::types::ActionDefinition;

/// A domain structure definition.
#[derive(Debug, Clone, PartialEq)]
pub enum StructureDef<'a> {
    Action(ActionDefinition<'a>), // TODO: Add <durative-action-def> in :durative-actions
                                  // TODO: Add <derived-def> in :derived-predicates
}

impl<'a> StructureDef<'a> {
    pub const fn new_action(action: ActionDefinition<'a>) -> Self {
        Self::Action(action)
    }
}

impl<'a> From<ActionDefinition<'a>> for StructureDef<'a> {
    fn from(value: ActionDefinition<'a>) -> Self {
        StructureDef::new_action(value)
    }
}
