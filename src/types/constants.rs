//! Provides constant definitions via the [`Constants`] type.

use crate::types::{Name, Typed, TypedNames};
use std::ops::Deref;

/// A set of constants.
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Constants(TypedNames);

impl Constants {
    pub const fn new(predicates: TypedNames) -> Self {
        Self(predicates)
    }
}

impl Deref for Constants {
    type Target = TypedNames;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<TypedNames> for Constants {
    fn from(value: TypedNames) -> Self {
        Constants::new(value)
    }
}

impl FromIterator<Typed<Name>> for Constants {
    fn from_iter<T: IntoIterator<Item = Typed<Name>>>(iter: T) -> Self {
        Constants::new(TypedNames::from_iter(iter))
    }
}
