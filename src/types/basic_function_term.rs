//! Contains the [`BasicFunctionTerm`] type.

use crate::types::{FunctionSymbol, Name};

#[derive(Debug, Clone, PartialEq)]
pub struct BasicFunctionTerm<'a>(FunctionSymbol<'a>, Vec<Name<'a>>);

impl<'a> BasicFunctionTerm<'a> {
    pub fn new<N: IntoIterator<Item = Name<'a>>>(symbol: FunctionSymbol<'a>, names: N) -> Self {
        Self(symbol, names.into_iter().collect())
    }

    pub const fn symbol(&self) -> &FunctionSymbol<'a> {
        &self.0
    }

    pub fn names(&self) -> &[Name<'a>] {
        self.1.as_slice()
    }
}

impl<'a> AsRef<FunctionSymbol<'a>> for BasicFunctionTerm<'a> {
    fn as_ref(&self) -> &FunctionSymbol<'a> {
        self.symbol()
    }
}

impl<'a> AsRef<[Name<'a>]> for BasicFunctionTerm<'a> {
    fn as_ref(&self) -> &[Name<'a>] {
        self.names()
    }
}
