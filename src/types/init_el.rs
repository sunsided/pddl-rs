//! Contains the [`InitElement`] type.

use crate::types::{BasicFunctionTerm, Name, NameLiteral, Number};

/// ## Usage
/// Used by [`InitElements`](crate::InitElements) in [`Problem`](crate::Problem).
#[derive(Debug, Clone, PartialEq)]
pub enum InitElement {
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
    pub const fn new_literal(name: NameLiteral) -> Self {
        Self::Literal(name)
    }

    pub const fn new_at(time: Number, name: NameLiteral) -> Self {
        Self::At(time, name)
    }

    pub const fn new_is_value(term: BasicFunctionTerm, value: Number) -> Self {
        Self::IsValue(term, value)
    }

    pub const fn new_is_object(term: BasicFunctionTerm, value: Name) -> Self {
        Self::IsObject(term, value)
    }
}
