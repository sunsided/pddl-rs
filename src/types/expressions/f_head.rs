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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FunctionHead {
    /// A function without arguments, `<function-symbol>` or `(<function-symbol>)`.
    Simple(FunctionSymbol), // TODO: Unify with `WithTerms`?
    /// A function applied to terms, `(<function-symbol> <term>*)`.
    WithTerms(FunctionSymbol, Vec<Term>),
}

impl FunctionHead {
    /// Creates a function head without arguments.
    #[must_use]
    pub const fn new(symbol: FunctionSymbol) -> Self {
        Self::Simple(symbol)
    }

    /// Creates a function head applied to the given terms.
    #[doc(alias = "new_with_terms")]
    #[must_use]
    pub fn with_terms<I: IntoIterator<Item = Term>>(symbol: FunctionSymbol, terms: I) -> Self {
        Self::WithTerms(symbol, terms.into_iter().collect())
    }

    /// Creates a function head applied to the given terms.
    #[must_use]
    pub fn new_with_terms<I: IntoIterator<Item = Term>>(symbol: FunctionSymbol, terms: I) -> Self {
        Self::with_terms(symbol, terms)
    }
}

/// Alias for [`FunctionHead`]; matches BNF `<f-head>`.
#[deprecated(since = "0.2.0", note = "Use `FunctionHead` instead")]
pub type FHead = FunctionHead;
