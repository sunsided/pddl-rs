//! Contains atomic function skeletons.

use crate::types::domain::{FunctionSymbol, Typed, TypedList, Variable};

/// An atomic function skeleton.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AtomicFunctionSkeleton<'a> {
    symbol: FunctionSymbol<'a>,
    variables: TypedList<'a, Variable<'a>>,
}

impl<'a> AtomicFunctionSkeleton<'a> {
    pub const fn new(symbol: FunctionSymbol<'a>, variables: TypedList<'a, Variable<'a>>) -> Self {
        Self { symbol, variables }
    }

    /// Gets a reference to the predicate.
    pub const fn symbol_ref(&self) -> &FunctionSymbol<'a> {
        &self.symbol
    }

    /// Gets a reference to the variables.
    pub fn variables_ref(&self) -> &TypedList<'a, Variable<'a>> {
        &self.variables
    }
}

impl<'a> From<(FunctionSymbol<'a>, TypedList<'a, Variable<'a>>)> for AtomicFunctionSkeleton<'a> {
    fn from(value: (FunctionSymbol<'a>, TypedList<'a, Variable<'a>>)) -> Self {
        AtomicFunctionSkeleton::new(value.0, value.1)
    }
}

impl<'a> From<(FunctionSymbol<'a>, Vec<Typed<'a, Variable<'a>>>)> for AtomicFunctionSkeleton<'a> {
    fn from(value: (FunctionSymbol<'a>, Vec<Typed<'a, Variable<'a>>>)) -> Self {
        AtomicFunctionSkeleton::new(value.0, value.1.into())
    }
}
