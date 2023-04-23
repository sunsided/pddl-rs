use crate::types::{FunctionSymbol, Term};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FHead<'a> {
    Simple(FunctionSymbol<'a>),
    WithTerms(FunctionSymbol<'a>, Vec<Term<'a>>),
}

impl<'a> FHead<'a> {
    pub const fn new(symbol: FunctionSymbol<'a>) -> Self {
        Self::Simple(symbol)
    }

    pub fn new_with_terms<I: IntoIterator<Item = Term<'a>>>(
        symbol: FunctionSymbol<'a>,
        terms: I,
    ) -> Self {
        Self::WithTerms(symbol, terms.into_iter().collect())
    }
}
