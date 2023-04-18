use crate::types::term::Term;
use crate::types::FunctionSymbol;

/// A function term.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionTerm<'a>(FunctionSymbol<'a>, Vec<Term<'a>>);

impl<'a> FunctionTerm<'a> {
    pub fn new<I: IntoIterator<Item = Term<'a>>>(symbol: FunctionSymbol<'a>, terms: I) -> Self {
        Self(symbol, terms.into_iter().collect())
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
