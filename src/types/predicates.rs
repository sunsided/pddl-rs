//! Contains predicates.

use crate::types::Name;
use std::ops::Deref;

/// A predicate name.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Predicate<'a>(Name<'a>);

impl<'a> From<Name<'a>> for Predicate<'a> {
    fn from(value: Name<'a>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for Predicate<'a> {
    fn from(value: &'a str) -> Self {
        Self(Name::from_str(value))
    }
}

impl<'a> AsRef<Name<'a>> for Predicate<'a> {
    fn as_ref(&self) -> &Name<'a> {
        &self.0
    }
}

impl<'a> Deref for Predicate<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
