//! Contains the [`Objects`] type.

use crate::types::{Name, Typed, TypedNames};
use std::ops::Deref;

/// A list of objects.
///
/// ## Usage
/// Used by [`Problem`](crate::Problem).
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Objects(TypedNames);

impl Objects {
    // TODO: Convert to const again that takes `TypedNames` directly.
    pub fn new<I: IntoIterator<Item = Typed<Name>>>(objects: I) -> Self {
        Self(objects.into_iter().collect())
    }

    pub fn values(&self) -> &TypedNames {
        &self.0
    }
}

impl From<TypedNames> for Objects {
    fn from(value: TypedNames) -> Self {
        Self(value)
    }
}

impl FromIterator<Typed<Name>> for Objects {
    fn from_iter<T: IntoIterator<Item = Typed<Name>>>(iter: T) -> Self {
        Objects::new(TypedNames::from_iter(iter))
    }
}

impl Deref for Objects {
    type Target = TypedNames;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
