//! Contains the [`InitElement`] type.

use crate::types::{BasicFunctionTerm, Name, NameLiteral, Number};

/// ## Usage
/// Used by [`InitElements`](crate::InitElements) in [`Problem`](crate::Problem).
#[derive(Debug, Clone, PartialEq)]
pub enum InitElement<'a> {
    Literal(NameLiteral<'a>),
    /// ## Requirements
    /// Requires [TimedInitialLiterals](crate::types::Requirement::TimedInitialLiterals).
    At(Number, NameLiteral<'a>),
    /// ## Requirements
    /// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
    IsValue(BasicFunctionTerm<'a>, Number),
    /// ## Requirements
    /// Requires [ObjectFluents](crate::types::Requirement::ObjectFluents).
    IsObject(BasicFunctionTerm<'a>, Name<'a>),
}

impl<'a> InitElement<'a> {
    pub const fn new_literal(name: NameLiteral<'a>) -> Self {
        Self::Literal(name)
    }

    pub const fn new_at(time: Number, name: NameLiteral<'a>) -> Self {
        Self::At(time, name)
    }

    pub const fn new_is_value(term: BasicFunctionTerm<'a>, value: Number) -> Self {
        Self::IsValue(term, value)
    }

    pub const fn new_is_object(term: BasicFunctionTerm<'a>, value: Name<'a>) -> Self {
        Self::IsObject(term, value)
    }
}
