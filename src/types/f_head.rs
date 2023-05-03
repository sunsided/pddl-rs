//! Contains function declarations via the [`FHead`] type.

use crate::types::{FunctionSymbol, Term};

/// A function declaration.
///
/// ## Requirements
/// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
///
/// ## Usage
/// Used by [`FExp`](crate::FExp), [`PEffect`](crate::PEffect), [`TimedEffect`](crate::TimedEffect)
/// and [`FAssignDa`](crate::FAssignDa).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FHead<'a> {
    Simple(FunctionSymbol<'a>), // TODO: Unify with `WithTerms`?
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
