//! Contains action definitions via the [`ActionDefinition`] type.

use crate::types::TypedVariables;
use crate::types::{ActionSymbol, Effects};
use crate::PreconditionGoalDescriptions;

/// An action definition.
///
/// ## Usage
/// Used by [`StructureDef`](crate::StructureDef).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionDefinition {
    /// The action's name, from `(:action <action-symbol> ...)`.
    symbol: ActionSymbol,
    /// The action's parameters, from `:parameters`.
    parameters: TypedVariables,
    /// The action's preconditions, from `:precondition`; empty if the action has none.
    precondition: PreconditionGoalDescriptions,
    /// The action's effect, from `:effect`; `None` if the action has none.
    effect: Option<Effects>,
}

impl ActionDefinition {
    /// Creates a new action definition from its name, parameters, preconditions and optional effect.
    #[must_use]
    pub const fn new(
        symbol: ActionSymbol,
        parameters: TypedVariables,
        precondition: PreconditionGoalDescriptions,
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
    #[must_use]
    pub const fn symbol(&self) -> &ActionSymbol {
        &self.symbol
    }

    /// Gets the action's parameters, i.e. the typed variables from `:parameters`.
    #[must_use]
    pub const fn parameters(&self) -> &TypedVariables {
        &self.parameters
    }

    /// Gets the action's preconditions, from `:precondition`; empty if the action has none.
    #[must_use]
    pub const fn precondition(&self) -> &PreconditionGoalDescriptions {
        &self.precondition
    }

    /// Gets the action's effect, from `:effect`; `None` if the action has none.
    #[must_use]
    pub const fn effect(&self) -> &Option<Effects> {
        &self.effect
    }
}

impl AsRef<ActionSymbol> for ActionDefinition {
    fn as_ref(&self) -> &ActionSymbol {
        &self.symbol
    }
}
