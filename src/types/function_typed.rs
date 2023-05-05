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
pub struct FunctionTyped<O>(O, FunctionType);

impl<O> FunctionTyped<O> {
    pub const fn new(value: O, r#type: FunctionType) -> Self {
        Self(value, r#type)
    }

    pub const fn new_number(value: O) -> Self {
        Self::new(value, FunctionType::NUMBER)
    }

    pub const fn from_type(value: O, r#type: Type) -> Self {
        Self::new(value, FunctionType::from(r#type))
    }

    pub const fn value_ref(&self) -> &O {
        &self.0
    }

    pub const fn type_ref(&self) -> &FunctionType {
        &self.1
    }
}

impl<O> From<O> for FunctionTyped<O> {
    fn from(value: O) -> Self {
        FunctionTyped::new_number(value)
    }
}

impl<O> Deref for FunctionTyped<O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        self.value_ref()
    }
}
