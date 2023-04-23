//! Contains the [`DurativeActionDefinition`] type.

use crate::types::domain::{
    DurationConstraint, DurativeActionEffect, DurativeActionGoalDefinition, DurativeActionSymbol,
    Variable,
};
use crate::types::utility::TypedList;

#[derive(Debug, Clone, PartialEq)]
pub struct DurativeActionDefinition<'a> {
    symbol: DurativeActionSymbol<'a>,
    parameters: TypedList<'a, Variable<'a>>,
    duration: Option<DurationConstraint<'a>>,
    condition: Option<DurativeActionGoalDefinition<'a>>,
    effect: Option<DurativeActionEffect<'a>>,
}

impl<'a> DurativeActionDefinition<'a> {
    pub const fn new(
        symbol: DurativeActionSymbol<'a>,
        parameters: TypedList<'a, Variable<'a>>,
        duration: Option<DurationConstraint<'a>>,
        condition: Option<DurativeActionGoalDefinition<'a>>,
        effect: Option<DurativeActionEffect<'a>>,
    ) -> Self {
        Self {
            symbol,
            parameters,
            duration,
            condition,
            effect,
        }
    }

    pub const fn symbol(&self) -> &DurativeActionSymbol<'a> {
        &self.symbol
    }

    pub const fn parameters(&self) -> &TypedList<'a, Variable<'a>> {
        &self.parameters
    }

    pub const fn duration(&self) -> &Option<DurationConstraint<'a>> {
        &self.duration
    }

    pub const fn condition(&self) -> &Option<DurativeActionGoalDefinition<'a>> {
        &self.condition
    }

    pub const fn effect(&self) -> &Option<DurativeActionEffect<'a>> {
        &self.effect
    }
}

impl<'a> AsRef<DurativeActionSymbol<'a>> for DurativeActionDefinition<'a> {
    fn as_ref(&self) -> &DurativeActionSymbol<'a> {
        self.symbol()
    }
}

impl<'a> AsRef<TypedList<'a, Variable<'a>>> for DurativeActionDefinition<'a> {
    fn as_ref(&self) -> &TypedList<'a, Variable<'a>> {
        self.parameters()
    }
}

impl<'a> AsRef<Option<DurationConstraint<'a>>> for DurativeActionDefinition<'a> {
    fn as_ref(&self) -> &Option<DurationConstraint<'a>> {
        self.duration()
    }
}

impl<'a> AsRef<Option<DurativeActionGoalDefinition<'a>>> for DurativeActionDefinition<'a> {
    fn as_ref(&self) -> &Option<DurativeActionGoalDefinition<'a>> {
        self.condition()
    }
}

impl<'a> AsRef<Option<DurativeActionEffect<'a>>> for DurativeActionDefinition<'a> {
    fn as_ref(&self) -> &Option<DurativeActionEffect<'a>> {
        self.effect()
    }
}
