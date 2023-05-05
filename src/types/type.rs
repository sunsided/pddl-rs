//! Contains types.

use crate::types::iterators::FlatteningIntoIterator;
use crate::types::Name;
use std::ops::Deref;

/// The `object` type.
pub const TYPE_OBJECT: PrimitiveType = PrimitiveType(Name::new_static("object"));

/// The `number` type.
#[allow(dead_code)]
pub const TYPE_NUMBER: PrimitiveType = PrimitiveType(Name::new_static("number"));

/// A primitive type.
///
/// ## Requirements
/// Requires [Typing](crate::Requirement::Typing).
///
/// ## Usage
/// Used by [`Type`].
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct PrimitiveType(Name);

/// A type selection from `<primitive-type> | (either <primitive-type>)`.
///
/// ## Requirements
/// Requires [Typing](crate::Requirement::Typing).
///
/// ## Usage
/// Used by [`Typed`](crate::Typed) in [`TypedList`](crate::TypedList),
/// [`FunctionType`](crate::FunctionType).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    /// The type is exactly this named type.
    Exactly(PrimitiveType),
    /// The type is either of these named types..
    EitherOf(Vec<PrimitiveType>),
}

impl Type {
    /// The predefined type `object`.
    pub const OBJECT: Type = Type::Exactly(TYPE_OBJECT);

    /// The predefined type `number`.
    pub const NUMBER: Type = Type::Exactly(TYPE_NUMBER);

    pub fn len(&self) -> usize {
        match self {
            Type::Exactly(_) => 1,
            Type::EitherOf(v) => v.len(),
        }
    }
}

impl PrimitiveType {
    pub fn new(name: Name) -> Self {
        Self(name)
    }
}

impl Default for Type {
    fn default() -> Self {
        Self::Exactly(TYPE_OBJECT)
    }
}

impl From<&str> for Type {
    fn from(value: &str) -> Self {
        Self::Exactly(value.into())
    }
}

impl From<Vec<&str>> for Type {
    fn from(value: Vec<&str>) -> Self {
        Self::EitherOf(value.iter().map(|&x| PrimitiveType::from(x)).collect())
    }
}

impl From<PrimitiveType> for Type {
    fn from(value: PrimitiveType) -> Self {
        Self::Exactly(value)
    }
}

impl From<Vec<PrimitiveType>> for Type {
    fn from(value: Vec<PrimitiveType>) -> Self {
        Self::EitherOf(value)
    }
}

impl<'a, P> FromIterator<P> for Type
where
    P: Into<PrimitiveType>,
{
    fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Self::EitherOf(iter.into_iter().map(|x| x.into()).collect())
    }
}

impl<'a, T> From<T> for PrimitiveType
where
    T: Into<Name>,
{
    #[inline(always)]
    fn from(value: T) -> Self {
        PrimitiveType::new(value.into())
    }
}

impl AsRef<str> for PrimitiveType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for PrimitiveType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoIterator for Type {
    type Item = PrimitiveType;
    type IntoIter = FlatteningIntoIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Type::Exactly(item) => FlatteningIntoIterator::new(item),
            Type::EitherOf(vec) => FlatteningIntoIterator::new_vec(vec),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::Span;
    use crate::Parser;

    #[test]
    fn flatten_with_single_element_works() {
        let (_, t) = Type::parse(Span::new("object")).unwrap();

        let mut iter = t.into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    #[test]
    fn flatten_with_many_elements_works() {
        let (_, t) = Type::parse(Span::new("(either object number)")).unwrap();

        let mut iter = t.into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }
}
