//! Contains names.

use crate::types::domain::{PrimitiveType, ToTyped, Type, Typed};
use std::ops::Deref;

/// A name.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Name<'a>(&'a str);

impl<'a> Name<'a> {
    #[inline(always)]
    pub const fn new(name: &'a str) -> Self {
        Self(name)
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> ToTyped<'a, Name<'a>> for Name<'a> {
    fn to_typed<I: Into<Type<'a>>>(self, r#type: I) -> Typed<'a, Name<'a>> {
        Typed::new(self, r#type.into())
    }
    fn to_typed_either<I: IntoIterator<Item = P>, P: Into<PrimitiveType<'a>>>(
        self,
        types: I,
    ) -> Typed<'a, Name<'a>> {
        Typed::new(self, Type::from_iter(types))
    }
}

impl<'a> From<&'a str> for Name<'a> {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Self::new(value)
    }
}

impl<'a> AsRef<str> for Name<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> Deref for Name<'a> {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
