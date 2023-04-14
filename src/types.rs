//! Contains types.

use std::ops::Deref;

/// The `object` type.
const TYPE_OBJECT: PrimitiveType<'static> = PrimitiveType("object");

/// The `number` type.
#[allow(dead_code)]
const TYPE_NUMBER: PrimitiveType<'static> = PrimitiveType("number");

/// A primitive type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct PrimitiveType<'a>(&'a str);

/// A type selection from `<primitive-type> | (either <primitive-type>)`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type<'a> {
    /// The type is exactly this named type.
    Exactly(PrimitiveType<'a>),
    /// The type is either of these named types..
    EitherOf(Vec<PrimitiveType<'a>>),
}

impl<'a> Default for Type<'a> {
    fn default() -> Self {
        Self::Exactly(TYPE_OBJECT)
    }
}

impl<'a> From<&'a str> for Type<'a> {
    fn from(value: &'a str) -> Self {
        Self::Exactly(value.into())
    }
}

impl<'a> From<Vec<&'a str>> for Type<'a> {
    fn from(value: Vec<&'a str>) -> Self {
        Self::EitherOf(value.iter().map(|&x| PrimitiveType::from(x)).collect())
    }
}

impl<'a> From<PrimitiveType<'a>> for Type<'a> {
    fn from(value: PrimitiveType<'a>) -> Self {
        Self::Exactly(value)
    }
}

impl<'a> From<Vec<PrimitiveType<'a>>> for Type<'a> {
    fn from(value: Vec<PrimitiveType<'a>>) -> Self {
        Self::EitherOf(value)
    }
}

impl<'a> FromIterator<PrimitiveType<'a>> for Type<'a> {
    fn from_iter<T: IntoIterator<Item = PrimitiveType<'a>>>(iter: T) -> Self {
        Self::EitherOf(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<&'a str> for Type<'a> {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        Self::EitherOf(iter.into_iter().map(PrimitiveType::from).collect())
    }
}

impl<'a> From<&'a str> for PrimitiveType<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl<'a> AsRef<str> for PrimitiveType<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> Deref for PrimitiveType<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
