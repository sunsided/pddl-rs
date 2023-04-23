//! Contains variables.

use crate::types::utility::{Name, PrimitiveType, ToTyped, Type, Typed};
use std::ops::Deref;

/// A variable name.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Variable<'a>(Name<'a>);

impl<'a> Variable<'a> {
    #[inline(always)]
    pub const fn from_str(name: &'a str) -> Self {
        Self(Name::new(name))
    }

    #[inline(always)]
    pub const fn from_name(name: Name<'a>) -> Self {
        Self(name)
    }
}

impl<'a> ToTyped<'a, Variable<'a>> for Variable<'a> {
    fn to_typed<I: Into<Type<'a>>>(self, r#type: I) -> Typed<'a, Variable<'a>> {
        Typed::new(self, r#type.into())
    }
    fn to_typed_either<I: IntoIterator<Item = P>, P: Into<PrimitiveType<'a>>>(
        self,
        types: I,
    ) -> Typed<'a, Variable<'a>> {
        Typed::new(self, Type::from_iter(types))
    }
}

impl<'a> From<Name<'a>> for Variable<'a> {
    #[inline(always)]
    fn from(value: Name<'a>) -> Self {
        Variable::from_name(value)
    }
}

impl<'a> From<&'a str> for Variable<'a> {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Variable::from_str(value)
    }
}

impl<'a> AsRef<Name<'a>> for Variable<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &Name<'a> {
        &self.0
    }
}

impl<'a> AsRef<str> for Variable<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> Deref for Variable<'a> {
    type Target = Name<'a>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
