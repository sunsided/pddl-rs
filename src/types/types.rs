//! Provides type definitions;

use crate::types::TypedNames;
use std::ops::Deref;

/// A set of types.
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Types(TypedNames);

impl Types {
    pub const fn new(predicates: TypedNames) -> Self {
        Self(predicates)
    }

    /// Gets the values.
    pub fn values(&self) -> &TypedNames {
        &self.0
    }
}

impl Deref for Types {
    type Target = TypedNames;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TypedNames> for Types {
    fn from(value: TypedNames) -> Self {
        Types::new(value)
    }
}
