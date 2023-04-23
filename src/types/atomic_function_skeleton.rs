//! Contains atomic function skeletons.

use crate::types::Typed;
use crate::types::{FunctionSymbol, TypedVariables, Variable};

/// An atomic function skeleton.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AtomicFunctionSkeleton<'a> {
    symbol: FunctionSymbol<'a>,
    variables: TypedVariables<'a>,
}

impl<'a> AtomicFunctionSkeleton<'a> {
    pub const fn new(symbol: FunctionSymbol<'a>, variables: TypedVariables<'a>) -> Self {
        Self { symbol, variables }
    }

    /// Gets a reference to the function symbol.
    pub const fn symbol(&self) -> &FunctionSymbol<'a> {
        &self.symbol
    }

    /// Gets a reference to the variables.
    pub fn variables(&self) -> &TypedVariables<'a> {
        &self.variables
    }
}

impl<'a> From<(FunctionSymbol<'a>, TypedVariables<'a>)> for AtomicFunctionSkeleton<'a> {
    fn from(value: (FunctionSymbol<'a>, TypedVariables<'a>)) -> Self {
        AtomicFunctionSkeleton::new(value.0, value.1)
    }
}

impl<'a> From<(FunctionSymbol<'a>, Vec<Typed<'a, Variable<'a>>>)> for AtomicFunctionSkeleton<'a> {
    fn from(value: (FunctionSymbol<'a>, Vec<Typed<'a, Variable<'a>>>)) -> Self {
        AtomicFunctionSkeleton::new(value.0, value.1.into())
    }
}
