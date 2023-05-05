//! Contains the [`BasicFunctionTerm`] type.

use crate::types::{FunctionSymbol, Name};

/// ## Usage
/// Used by [`InitElement`](crate::InitElement).
#[derive(Debug, Clone, PartialEq)]
pub struct BasicFunctionTerm(FunctionSymbol, Vec<Name>);

impl BasicFunctionTerm {
    pub fn new<N: IntoIterator<Item = Name>>(symbol: FunctionSymbol, names: N) -> Self {
        Self(symbol, names.into_iter().collect())
    }

    /// Returns the function symbol.
    pub const fn symbol(&self) -> &FunctionSymbol {
        &self.0
    }

    /// Returns the associated names.
    pub fn names(&self) -> &[Name] {
        self.1.as_slice()
    }
}

impl AsRef<FunctionSymbol> for BasicFunctionTerm {
    fn as_ref(&self) -> &FunctionSymbol {
        self.symbol()
    }
}

impl AsRef<[Name]> for BasicFunctionTerm {
    fn as_ref(&self) -> &[Name] {
        self.names()
    }
}
