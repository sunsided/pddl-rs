//! Contains action symbols via the [`ActionSymbol`] type.

use crate::types::Name;
use std::ops::Deref;

/// An action symbol name.
///
/// ## Usage
/// Used by [`ActionDefinition`](crate::ActionDefinition).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct ActionSymbol<'a>(Name<'a>);

impl<'a> ActionSymbol<'a> {
    #[inline(always)]
    pub const fn new(name: Name<'a>) -> Self {
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

impl<'a, T> From<T> for ActionSymbol<'a>
where
    T: Into<Name<'a>>,
{
    #[inline(always)]
    fn from(value: T) -> Self {
        ActionSymbol::new(value.into())
    }
}

impl<'a> AsRef<Name<'a>> for ActionSymbol<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &Name<'a> {
        &self.0
    }
}

impl<'a> AsRef<str> for ActionSymbol<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> Deref for ActionSymbol<'a> {
    type Target = Name<'a>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
