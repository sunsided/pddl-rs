//! Contains function symbols.

use crate::types::Name;
use std::ops::Deref;

/// A function symbol name.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct FunctionSymbol<'a>(Name<'a>);

impl<'a> From<Name<'a>> for FunctionSymbol<'a> {
    fn from(value: Name<'a>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for FunctionSymbol<'a> {
    fn from(value: &'a str) -> Self {
        Self(Name::from_str(value))
    }
}

impl<'a> AsRef<Name<'a>> for FunctionSymbol<'a> {
    fn as_ref(&self) -> &Name<'a> {
        &self.0
    }
}

impl<'a> Deref for FunctionSymbol<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
