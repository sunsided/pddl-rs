//! Contains the [`InitElement`] type.

use crate::types::{BasicFunctionTerm, Name, NameLiteral, Number};

/// ## Usage
/// Used by [`InitElements`](crate::InitElements) in [`Problem`](crate::Problem).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InitElement {
    /// A literal that holds in the initial state, e.g. `(at truck depot)` or `(not (busy truck))`.
    Literal(NameLiteral),
    /// ## Requirements
    /// Requires [Timed Initial Literals](crate::Requirement::TimedInitialLiterals).
    At(Number, NameLiteral),
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    IsValue(BasicFunctionTerm, Number),
    /// ## Requirements
    /// Requires [Object Fluents](crate::Requirement::ObjectFluents).
    IsObject(BasicFunctionTerm, Name),
}

impl InitElement {
    /// Creates a new initial literal.
    #[must_use]
    pub const fn new_literal(name: NameLiteral) -> Self {
        Self::Literal(name)
    }

    /// Creates a new timed initial literal, i.e. `(at <number> <literal (name)>)`,
    /// which becomes true at the given time.
    #[must_use]
    pub const fn new_at(time: Number, name: NameLiteral) -> Self {
        Self::At(time, name)
    }

    /// Creates a new numeric fluent assignment, i.e. `(= <basic-function-term> <number>)`.
    #[must_use]
    pub const fn new_is_value(term: BasicFunctionTerm, value: Number) -> Self {
        Self::IsValue(term, value)
    }

    /// Creates a new object fluent assignment, i.e. `(= <basic-function-term> <name>)`.
    #[must_use]
    pub const fn new_is_object(term: BasicFunctionTerm, value: Name) -> Self {
        Self::IsObject(term, value)
    }
}
