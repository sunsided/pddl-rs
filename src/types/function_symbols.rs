//! Contains function symbols via the [`FunctionSymbol`] type..

use crate::types::Name;
use std::ops::Deref;

/// A function symbol name.
///
/// ## Usage
/// Used by [`FunctionTerm`](crate::FunctionTerm), [`FHead`](crate::FHead),
/// [`BasicFunctionTerm`](crate::BasicFunctionTerm) and [`MetricFExp`](crate::MetricFExp).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct FunctionSymbol(Name);

impl FunctionSymbol {
    #[inline(always)]
    pub fn new(name: Name) -> Self {
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

impl<'a, T> From<T> for FunctionSymbol
where
    T: Into<Name>,
{
    #[inline(always)]
    fn from(value: T) -> Self {
        FunctionSymbol::new(value.into())
    }
}

impl AsRef<Name> for FunctionSymbol {
    #[inline(always)]
    fn as_ref(&self) -> &Name {
        &self.0
    }
}

impl AsRef<str> for FunctionSymbol {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for FunctionSymbol {
    type Target = Name;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
