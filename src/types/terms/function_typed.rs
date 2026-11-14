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
    /// Creates a new typed function element.
    pub const fn new(value: O, r#type: FunctionType) -> Self {
        Self(value, r#type)
    }

    /// Creates a new typed function element of type `number`.
    pub const fn number(value: O) -> Self {
        Self::new(value, FunctionType::NUMBER)
    }

    /// Creates a new typed function element of type `number`.
    #[deprecated(since = "0.2.0", note = "Use `number` instead")]
    pub const fn new_number(value: O) -> Self {
        Self::number(value)
    }

    /// Creates a new typed function element from a [`Type`].
    pub const fn from_type(value: O, r#type: Type) -> Self {
        Self::new(value, FunctionType::from(r#type))
    }

    /// Gets the typed value.
    pub const fn value_ref(&self) -> &O {
        &self.0
    }

    /// Gets the function type.
    pub const fn type_ref(&self) -> &FunctionType {
        &self.1
    }
}

impl<O> From<O> for FunctionTyped<O> {
    fn from(value: O) -> Self {
        FunctionTyped::number(value)
    }
}

impl<O> Deref for FunctionTyped<O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        self.value_ref()
    }
}
