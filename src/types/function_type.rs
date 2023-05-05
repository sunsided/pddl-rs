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
pub struct FunctionType(Type);

impl FunctionType {
    pub const NUMBER: FunctionType = FunctionType::from(Type::NUMBER);

    pub fn new<T: Into<Type>>(r#type: T) -> Self {
        Self(r#type.into())
    }

    pub const fn from(r#type: Type) -> Self {
        Self(r#type)
    }
}

impl Default for FunctionType {
    fn default() -> Self {
        Self(Type::NUMBER)
    }
}

impl From<&str> for FunctionType {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<Vec<&str>> for FunctionType {
    fn from(value: Vec<&str>) -> Self {
        Self(Type::from_iter(
            value.iter().map(|&x| PrimitiveType::from(x)),
        ))
    }
}

impl From<Type> for FunctionType {
    fn from(value: Type) -> Self {
        Self(value)
    }
}

impl Deref for FunctionType {
    type Target = Type;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
