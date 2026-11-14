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
    /// The action's name, from `(:action <action-symbol> ...)`.
    symbol: ActionSymbol,
    /// The action's parameters, from `:parameters`.
    parameters: TypedVariables,
    /// The action's preconditions, from `:precondition`; empty if the action has none.
    precondition: PreconditionGoalDefinitions,
    /// The action's effect, from `:effect`; `None` if the action has none.
    effect: Option<Effects>,
}

impl ActionDefinition {
    /// Creates a new action definition from its name, parameters, preconditions and optional effect.
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

    /// Gets the action's name.
    pub const fn symbol(&self) -> &ActionSymbol {
        &self.symbol
    }

    /// Gets the action's parameters, i.e. the typed variables from `:parameters`.
    pub const fn parameters(&self) -> &TypedVariables {
        &self.parameters
    }

    /// Gets the action's preconditions, from `:precondition`; empty if the action has none.
    pub const fn precondition(&self) -> &PreconditionGoalDefinitions {
        &self.precondition
    }

    /// Gets the action's effect, from `:effect`; `None` if the action has none.
    pub const fn effect(&self) -> &Option<Effects> {
        &self.effect
    }
}

impl AsRef<ActionSymbol> for ActionDefinition {
    fn as_ref(&self) -> &ActionSymbol {
        &self.symbol
    }
}
