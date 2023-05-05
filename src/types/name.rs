//! Contains names via the [`Name`] type.

use crate::types::{PrimitiveType, ToTyped, Type, Typed};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

/// A name.
///
/// ## Usage
/// Used by [`Domain`](crate::Domain), [`InitElement`](crate::InitElement),
/// [`BasicFunctionTerm`](crate::BasicFunctionTerm), [`MetricFExp`](crate::MetricFExp),
/// [`PrimitiveType`](PrimitiveType), [`Predicate`](crate::Predicate), [`Variable`](crate::Variable),
/// [`FunctionSymbol`](crate::FunctionSymbol), [`ActionSymbol`](crate::ActionSymbol),
/// [`PreferenceName`](crate::PreferenceName), [`Term`](crate::Term),
/// [`DurativeActionSymbol`](crate::DurativeActionSymbol) and [`Objects`](crate::Objects).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Name(NameVariant);

#[derive(Clone, PartialEq, Eq, Hash)]
enum NameVariant {
    String(String),
    Static(&'static str),
}

impl Name {
    #[inline(always)]
    pub fn new(name: &str) -> Self {
        Self::new_string(name.into())
    }

    #[inline(always)]
    pub const fn new_string(name: String) -> Self {
        Self(NameVariant::String(name))
    }

    #[inline(always)]
    pub const fn new_static(name: &'static str) -> Self {
        Self(NameVariant::Static(name))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Default for NameVariant {
    fn default() -> Self {
        Self::Static("")
    }
}

impl Deref for NameVariant {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            NameVariant::String(str) => str.as_str(),
            NameVariant::Static(str) => str,
        }
    }
}

impl PartialEq<str> for NameVariant {
    fn eq(&self, other: &str) -> bool {
        match self {
            NameVariant::String(str) => str.eq(other),
            NameVariant::Static(str) => (*str).eq(other),
        }
    }
}

impl Debug for NameVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for NameVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NameVariant::String(str) => write!(f, "{}", str),
            NameVariant::Static(str) => write!(f, "{}", str),
        }
    }
}

impl ToTyped<Name> for Name {
    fn to_typed<I: Into<Type>>(self, r#type: I) -> Typed<Name> {
        Typed::new(self, r#type.into())
    }
    fn to_typed_either<I: IntoIterator<Item = P>, P: Into<PrimitiveType>>(
        self,
        types: I,
    ) -> Typed<Name> {
        Typed::new(self, Type::from_iter(types))
    }
}

impl From<&str> for Name {
    #[inline(always)]
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for Name {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Name {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
