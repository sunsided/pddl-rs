//! Contains typed elements.

use crate::types::{PrimitiveType, Type};
use std::ops::Deref;

/// A typed element.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Typed<'a, O>(O, Type<'a>);

impl<'a, O> Typed<'a, O> {
    pub const fn new(value: O, r#type: Type<'a>) -> Self {
        Self(value, r#type)
    }

    pub const fn new_object(value: O) -> Self {
        Self::new(value, Type::OBJECT)
    }

    pub const fn value_ref(&self) -> &O {
        &self.0
    }

    pub const fn type_ref(&self) -> &Type<'a> {
        &self.1
    }
}

pub trait ToTyped<'a, T> {
    /// Wraps the value into a [`Typed`] as [`Type::Exactly`] the specified type.
    fn to_typed<I: Into<Type<'a>>>(self, r#type: I) -> Typed<'a, T>;
    /// Wraps the value into a [`Typed`] as [`Type::EitherOf`] the specified types.
    fn to_typed_either<I: IntoIterator<Item = P>, P: Into<PrimitiveType<'a>>>(
        self,
        r#type: I,
    ) -> Typed<'a, T>;
}

impl<'a, O> From<O> for Typed<'a, O> {
    fn from(value: O) -> Self {
        Typed::new_object(value)
    }
}

impl<'a, O> Deref for Typed<'a, O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        self.value_ref()
    }
}
