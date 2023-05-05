//! Contains action symbols via the [`ActionSymbol`] type.

use crate::types::Name;
use std::ops::Deref;

/// An action symbol name.
///
/// ## Usage
/// Used by [`ActionDefinition`](crate::ActionDefinition).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct ActionSymbol(Name);

impl ActionSymbol {
    #[inline(always)]
    pub const fn new(name: Name) -> Self {
        Self(name)
    }

    #[inline(always)]
    pub fn from_str(name: &str) -> Self {
        Self(Name::new(name))
    }

    #[inline(always)]
    pub const fn from_name(name: Name) -> Self {
        Self(name)
    }
}

impl<'a, T> From<T> for ActionSymbol
where
    T: Into<Name>,
{
    #[inline(always)]
    fn from(value: T) -> Self {
        ActionSymbol::new(value.into())
    }
}

impl AsRef<Name> for ActionSymbol {
    #[inline(always)]
    fn as_ref(&self) -> &Name {
        &self.0
    }
}

impl AsRef<str> for ActionSymbol {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for ActionSymbol {
    type Target = Name;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
