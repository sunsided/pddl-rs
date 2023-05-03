//! Contains action definitions via the [`ActionDefinition`] type.

use crate::types::TypedVariables;
use crate::types::{ActionSymbol, Effect, PreGD};

/// An action definition.
///
/// ## Usage
/// Used by [`StructureDef`](crate::StructureDef).
#[derive(Debug, Clone, PartialEq)]
pub struct ActionDefinition<'a> {
    symbol: ActionSymbol<'a>,
    parameters: TypedVariables<'a>,
    precondition: Option<PreGD<'a>>,
    effect: Option<Effect<'a>>,
}

impl<'a> ActionDefinition<'a> {
    pub const fn new(
        symbol: ActionSymbol<'a>,
        parameters: TypedVariables<'a>,
        precondition: Option<PreGD<'a>>,
        effect: Option<Effect<'a>>,
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

    pub const fn precondition(&self) -> &Option<PreGD<'a>> {
        &self.precondition
    }

    pub const fn effect(&self) -> &Option<Effect<'a>> {
        &self.effect
    }
}

impl<'a> AsRef<ActionSymbol<'a>> for ActionDefinition<'a> {
    fn as_ref(&self) -> &ActionSymbol<'a> {
        &self.symbol
    }
}
