//! Contains action definitions via the [`ActionDefinition`] type.

use crate::types::TypedVariables;
use crate::types::{ActionSymbol, Effects};
use crate::PreconditionGoalDefinitions;

/// An action definition.
///
/// ## Usage
/// Used by [`StructureDef`](crate::StructureDef).
#[derive(Debug, Clone, PartialEq)]
pub struct ActionDefinition<'a> {
    symbol: ActionSymbol<'a>,
    parameters: TypedVariables<'a>,
    precondition: PreconditionGoalDefinitions<'a>,
    effect: Option<Effects<'a>>,
}

impl<'a> ActionDefinition<'a> {
    pub const fn new(
        symbol: ActionSymbol<'a>,
        parameters: TypedVariables<'a>,
        precondition: PreconditionGoalDefinitions<'a>,
        effect: Option<Effects<'a>>,
    ) -> Self {
        Self {
            symbol,
            parameters,
            precondition,
            effect,
        }
    }

    pub const fn symbol(&self) -> &ActionSymbol<'a> {
        &self.symbol
    }

    pub const fn parameters(&self) -> &TypedVariables<'a> {
        &self.parameters
    }

    pub const fn precondition(&self) -> &PreconditionGoalDefinitions<'a> {
        &self.precondition
    }

    pub const fn effect(&self) -> &Option<Effects<'a>> {
        &self.effect
    }
}

impl<'a> AsRef<ActionSymbol<'a>> for ActionDefinition<'a> {
    fn as_ref(&self) -> &ActionSymbol<'a> {
        &self.symbol
    }
}
