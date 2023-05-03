//! Contains function terms via the [`FunctionTerm`] type.

use crate::types::term::Term;
use crate::types::FunctionSymbol;

/// A function term.
///
/// ## Requirements
/// Requires [Object Fluents](crate::Requirement::ObjectFluents).
///
/// ## Usage
/// Used by [`Term`], and [`PEffect`](crate::PEffect).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionTerm<'a>(FunctionSymbol<'a>, Vec<Term<'a>>);

impl<'a> FunctionTerm<'a> {
    pub fn new<I: IntoIterator<Item = Term<'a>>>(symbol: FunctionSymbol<'a>, terms: I) -> Self {
        Self(symbol, terms.into_iter().collect())
    }

    /// Gets the function symbol.
    pub const fn symbol(&self) -> &FunctionSymbol<'a> {
        &self.0
    }

    /// Gets the function terms.
    pub fn terms(&self) -> &[Term<'a>] {
        self.1.as_ref()
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
