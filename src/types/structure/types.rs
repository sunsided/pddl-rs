//! Provides type definitions;

use crate::types::TypedNames;
use std::ops::Deref;

/// A set of types.
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Types(TypedNames);

impl Types {
    /// Creates a new set of types, i.e. the content of `(:types <typed list (name)>)`;
    /// each type name is typed with its parent type.
    #[must_use]
    pub const fn new(predicates: TypedNames) -> Self {
        Self(predicates)
    }

    /// Gets the values.
    #[must_use]
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
