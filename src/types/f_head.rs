//! Contains function declarations via the [`FunctionHead`] type.

use crate::types::{FunctionSymbol, Term};

/// A function declaration.
///
/// ## Requirements
/// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
///
/// ## Usage
/// Used by [`FluentExpression`](crate::FluentExpression), [`PrimitiveEffect`](crate::PrimitiveEffect), [`TimedEffect`](crate::TimedEffect)
/// and [`DurativeActionFunctionAssignment`](crate::DurativeActionFunctionAssignment).
#[doc(alias("f-head"))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FunctionHead {
    Simple(FunctionSymbol), // TODO: Unify with `WithTerms`?
    WithTerms(FunctionSymbol, Vec<Term>),
}

impl FunctionHead {
    pub const fn new(symbol: FunctionSymbol) -> Self {
        Self::Simple(symbol)
    }

    pub fn new_with_terms<I: IntoIterator<Item = Term>>(symbol: FunctionSymbol, terms: I) -> Self {
        Self::WithTerms(symbol, terms.into_iter().collect())
    }
}

/// Alias for [`FunctionHead`]; matches BNF `<f-head>`.
#[deprecated(since = "0.2.0", note = "Use `FunctionHead` instead")]
pub type FHead = FunctionHead;
