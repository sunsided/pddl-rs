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
pub struct FunctionTerm(FunctionSymbol, Vec<Term>);

impl FunctionTerm {
    pub fn new<I: IntoIterator<Item = Term>>(symbol: FunctionSymbol, terms: I) -> Self {
        Self(symbol, terms.into_iter().collect())
    }

    /// Gets the function symbol.
    pub const fn symbol(&self) -> &FunctionSymbol {
        &self.0
    }

    /// Gets the function terms.
    pub fn terms(&self) -> &[Term] {
        self.1.as_ref()
    }
}

impl AsRef<FunctionSymbol> for FunctionTerm {
    fn as_ref(&self) -> &FunctionSymbol {
        &self.0
    }
}

impl AsRef<Vec<Term>> for FunctionTerm {
    fn as_ref(&self) -> &Vec<Term> {
        &self.1
    }
}
