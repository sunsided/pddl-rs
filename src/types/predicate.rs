//! Contains predicates via the [`Predicate`] type.

use crate::types::Name;
use std::ops::Deref;

/// A predicate name.
///
/// ## Usage
/// Used by [`AtomicFormulaSkeleton`](crate::AtomicFormulaSkeleton) and [`AtomicFormula`](crate::AtomicFormula).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Predicate(Name);

impl Predicate {
    #[inline(always)]
    pub const fn new(name: Name) -> Self {
        Self(name)
    }

    #[inline(always)]
    pub fn from_str(name: &str) -> Self {
        Self(Name::new(name))
    }

    #[inline(always)]
    pub const fn from_static(name: &'static str) -> Self {
        Self(Name::new_static(name))
    }

    #[inline(always)]
    pub const fn from_name(name: Name) -> Self {
        Self(name)
    }
}

impl<'a, T> From<T> for Predicate
where
    T: Into<Name>,
{
    #[inline(always)]
    fn from(value: T) -> Self {
        Predicate::new(value.into())
    }
}

impl AsRef<Name> for Predicate {
    #[inline(always)]
    fn as_ref(&self) -> &Name {
        &self.0
    }
}

impl AsRef<str> for Predicate {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for Predicate {
    type Target = Name;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
