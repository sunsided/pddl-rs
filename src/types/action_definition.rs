//! Contains action definitions via the [`ActionDefinition`] type.

use crate::types::TypedVariables;
use crate::types::{ActionSymbol, Effects};
use crate::PreconditionGoalDefinitions;

/// An action definition.
///
/// ## Usage
/// Used by [`StructureDef`](crate::StructureDef).
#[derive(Debug, Clone, PartialEq)]
pub struct ActionDefinition {
    symbol: ActionSymbol,
    parameters: TypedVariables,
    precondition: PreconditionGoalDefinitions,
    effect: Option<Effects>,
}

impl ActionDefinition {
    pub const fn new(
        symbol: ActionSymbol,
        parameters: TypedVariables,
        precondition: PreconditionGoalDefinitions,
        effect: Option<Effects>,
    ) -> Self {
        Self {
            symbol,
            parameters,
            precondition,
            effect,
        }
    }

    pub const fn symbol(&self) -> &ActionSymbol {
        &self.symbol
    }

    pub const fn parameters(&self) -> &TypedVariables {
        &self.parameters
    }

    pub const fn precondition(&self) -> &PreconditionGoalDefinitions {
        &self.precondition
    }

    pub const fn effect(&self) -> &Option<Effects> {
        &self.effect
    }
}

impl AsRef<ActionSymbol> for ActionDefinition {
    fn as_ref(&self) -> &ActionSymbol {
        &self.symbol
    }
}
