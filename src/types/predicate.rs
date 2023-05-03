//! Contains predicates via the [`Predicate`] type.

use crate::types::Name;
use std::ops::Deref;

/// A predicate name.
///
/// ## Usage
/// Used by [`AtomicFormulaSkeleton`](crate::AtomicFormulaSkeleton) and [`AtomicFormula`](crate::AtomicFormula).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Predicate<'a>(Name<'a>);

impl<'a> Predicate<'a> {
    #[inline(always)]
    pub fn new(name: Name<'a>) -> Self {
        Self(name)
    }

    #[inline(always)]
    pub const fn from_str(name: &'a str) -> Self {
        Self(Name::new(name))
    }

    #[inline(always)]
    pub const fn from_name(name: Name<'a>) -> Self {
        Self(name)
    }
}

impl<'a, T> From<T> for Predicate<'a>
where
    T: Into<Name<'a>>,
{
    #[inline(always)]
    fn from(value: T) -> Self {
        Predicate::new(value.into())
    }
}

impl<'a> AsRef<Name<'a>> for Predicate<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &Name<'a> {
        &self.0
    }
}

impl<'a> AsRef<str> for Predicate<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> Deref for Predicate<'a> {
    type Target = Name<'a>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
