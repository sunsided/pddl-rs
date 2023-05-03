//! Contains function typed elements via the [`FunctionTyped`] type.

use crate::types::FunctionType;
use crate::types::Type;
use std::ops::Deref;

/// A typed function element.
///
/// ## Requirements
/// Requires [Fluents](crate::Requirement::Fluents), as well as the same requirements as [`FunctionType`].
///
/// ## Usage
/// Used by [`FunctionTypedList`](crate::FunctionTypedList).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionTyped<'a, O>(O, FunctionType<'a>);

impl<'a, O> FunctionTyped<'a, O> {
    pub const fn new(value: O, r#type: FunctionType<'a>) -> Self {
        Self(value, r#type)
    }

    pub const fn new_number(value: O) -> Self {
        Self::new(value, FunctionType::NUMBER)
    }

    pub const fn from_type(value: O, r#type: Type<'a>) -> Self {
        Self::new(value, FunctionType::from(r#type))
    }

    pub const fn value_ref(&self) -> &O {
        &self.0
    }

    pub const fn type_ref(&self) -> &FunctionType<'a> {
        &self.1
    }
}

impl<'a, O> From<O> for FunctionTyped<'a, O> {
    fn from(value: O) -> Self {
        FunctionTyped::new_number(value)
    }
}

impl<'a, O> Deref for FunctionTyped<'a, O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        self.value_ref()
    }
}
