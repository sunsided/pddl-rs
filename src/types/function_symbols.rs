//! Contains function symbols via the [`FunctionSymbol`] type..

use crate::types::Name;
use std::ops::Deref;

/// A function symbol name.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct FunctionSymbol<'a>(Name<'a>);

impl<'a> FunctionSymbol<'a> {
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

impl<'a, T> From<T> for FunctionSymbol<'a>
where
    T: Into<Name<'a>>,
{
    #[inline(always)]
    fn from(value: T) -> Self {
        FunctionSymbol::new(value.into())
    }
}

impl<'a> AsRef<Name<'a>> for FunctionSymbol<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &Name<'a> {
        &self.0
    }
}

impl<'a> AsRef<str> for FunctionSymbol<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> Deref for FunctionSymbol<'a> {
    type Target = Name<'a>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
