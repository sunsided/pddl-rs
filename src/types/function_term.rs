use crate::types::FunctionSymbol;
use crate::types::term::Term;

/// A function term.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionTerm<'a>(FunctionSymbol<'a>, Vec<Term<'a>>);

impl<'a> FunctionTerm<'a> {
    pub const fn new(symbol: FunctionSymbol<'a>, terms: Vec<Term<'a>>) -> Self {
        Self(symbol, terms)
    }
}

impl<'a> AsRef<FunctionSymbol<'a>> for FunctionTerm<'a> {
    fn as_ref(&self) -> &FunctionSymbol<'a> {
        &self.0
    }
}

impl<'a> AsRef<Vec<Term<'a>>> for FunctionTerm<'a> {
    fn as_ref(&self) -> &Vec<Term<'a>> {
        &self.1
    }
}
