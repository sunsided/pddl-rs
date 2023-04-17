//! Contains function types.

use crate::types::{PrimitiveType, Type};
use std::ops::Deref;

/// A function type.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionType<'a>(Type<'a>);

impl<'a> FunctionType<'a> {
    pub const fn new(r#type: Type<'a>) -> Self {
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
