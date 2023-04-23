//! Contains action definitions.

use crate::types::domain::{ActionSymbol, Effect, PreGD, TypedList, Variable};

/// An action definition.
#[derive(Debug, Clone, PartialEq)]
pub struct ActionDefinition<'a> {
    symbol: ActionSymbol<'a>,
    parameters: TypedList<'a, Variable<'a>>,
    precondition: Option<PreGD<'a>>,
    effect: Option<Effect<'a>>,
}

impl<'a> ActionDefinition<'a> {
    pub const fn new(
        symbol: ActionSymbol<'a>,
        parameters: TypedList<'a, Variable<'a>>,
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

    pub const fn parameters(&self) -> &TypedList<'a, Variable<'a>> {
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
