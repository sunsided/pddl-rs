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
pub enum FHead {
    Simple(FunctionSymbol), // TODO: Unify with `WithTerms`?
    WithTerms(FunctionSymbol, Vec<Term>),
}

impl FHead {
    pub const fn new(symbol: FunctionSymbol) -> Self {
        Self::Simple(symbol)
    }

    pub fn new_with_terms<I: IntoIterator<Item = Term>>(symbol: FunctionSymbol, terms: I) -> Self {
        Self::WithTerms(symbol, terms.into_iter().collect())
    }
}
