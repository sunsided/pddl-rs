//! Contains types.

use crate::types::Name;
use std::ops::Deref;

/// The `object` type.
pub const TYPE_OBJECT: PrimitiveType<'static> = PrimitiveType(Name::new("object"));

/// The `number` type.
#[allow(dead_code)]
pub const TYPE_NUMBER: PrimitiveType<'static> = PrimitiveType(Name::new("number"));

/// A primitive type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct PrimitiveType<'a>(Name<'a>);

/// A type selection from `<primitive-type> | (either <primitive-type>)`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type<'a> {
    /// The type is exactly this named type.
    Exactly(PrimitiveType<'a>),
    /// The type is either of these named types..
    EitherOf(Vec<PrimitiveType<'a>>),
}

impl<'a> Type<'a> {
    /// The predefined type `object`.
    pub const OBJECT: Type<'a> = Type::Exactly(TYPE_OBJECT);

    /// The predefined type `number`.
    pub const NUMBER: Type<'a> = Type::Exactly(TYPE_NUMBER);

    pub fn len(&self) -> usize {
        match self {
            Type::Exactly(_) => 1,
            Type::EitherOf(v) => v.len(),
        }
    }
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
        Self(Name::new(value))
    }
}

impl<'a> From<Name<'a>> for PrimitiveType<'a> {
    fn from(value: Name<'a>) -> Self {
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
