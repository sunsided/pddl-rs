//! Contains function types via the [`FunctionType`] ... type.

use crate::types::{PrimitiveType, Type};
use std::ops::Deref;

/// A function type.
///
/// ## Requirements
/// Requires [Fluents](crate::Requirement::Fluents), as well as either
/// [Numeric Fluents](crate::Requirement::NumericFluents) for the default `number` type, or
/// [Object Fluents](crate::Requirement::ObjectFluents) and [Typing](crate::Requirement::Typing)
/// for an arbitrary type.
///
/// ## Usage
/// Used by [`FunctionTyped`](crate::FunctionTyped) in [`FunctionTypedList`](crate::FunctionTypedList).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionType<'a>(Type<'a>);

impl<'a> FunctionType<'a> {
    pub const NUMBER: FunctionType<'static> = FunctionType::from(Type::NUMBER);

    pub fn new<T: Into<Type<'a>>>(r#type: T) -> Self {
        Self(r#type.into())
    }

    pub const fn from(r#type: Type<'a>) -> Self {
        Self(r#type)
    }
}

impl<'a> Default for FunctionType<'a> {
    fn default() -> Self {
        Self(Type::NUMBER)
    }
}

impl<'a> From<&'a str> for FunctionType<'a> {
    fn from(value: &'a str) -> Self {
        Self(value.into())
    }
}

impl<'a> From<Vec<&'a str>> for FunctionType<'a> {
    fn from(value: Vec<&'a str>) -> Self {
        Self(Type::from_iter(
            value.iter().map(|&x| PrimitiveType::from(x)),
        ))
    }
}

impl<'a> From<Type<'a>> for FunctionType<'a> {
    fn from(value: Type<'a>) -> Self {
        Self(value)
    }
}

impl<'a> Deref for FunctionType<'a> {
    type Target = Type<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
