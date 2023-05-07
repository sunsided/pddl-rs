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
    /// Constructs a new [`Name`] from a provided string.
    ///
    /// ## Arguments
    /// * `name` - The name to wrap.
    ///
    /// ## Returns
    /// A new [`Name`].
    #[inline(always)]
    pub fn new<S: Into<String> + AsRef<str>>(name: S) -> Self {
        if let Some(str) = Self::map_to_static(name.as_ref()) {
            Self::new_static(str)
        } else {
            Self(NameVariant::String(name.into()))
        }
    }

    /// Like [`new`] but makes use of the fact that if the string provided
    /// is `'static`, the method can be `const`.
    ///
    /// ## Arguments
    /// * `name` - The name to wrap.
    ///
    /// ## Returns
    /// A new [`Name`].
    ///
    /// ## Example
    /// ```
    /// # use pddl::Name;
    /// assert_eq!(Name::new("name"), "name");
    /// ```
    #[inline(always)]
    pub const fn new_static(name: &'static str) -> Self {
        Self(NameVariant::Static(name))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets the length of the name, in chars.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Maps the provided to a well-known `'static` string if possible.
    fn map_to_static<'a>(value: &'a str) -> Option<&'static str> {
        match value {
            well_known::OBJECT => Some(well_known::OBJECT),
            well_known::NUMBER => Some(well_known::NUMBER),
            _ => None,
        }
    }
}

/// Provides well-known names for string interning.
mod well_known {
    pub const OBJECT: &'static str = "object";
    pub const NUMBER: &'static str = "number";
}

impl<T> From<T> for Name
where
    T: Into<String> + AsRef<str>,
{
    #[inline(always)]
    fn from(value: T) -> Self {
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

impl PartialEq<str> for Name {
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<&str> for Name {
    #[inline(always)]
    fn eq(&self, other: &&str) -> bool {
        self.0.eq(*other)
    }
}

impl PartialEq<String> for Name {
    #[inline(always)]
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other.as_str())
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

impl NameVariant {
    /// Gets the length of the name, in chars.
    fn len(&self) -> usize {
        match self {
            NameVariant::String(s) => s.chars().count(),
            NameVariant::Static(s) => s.chars().count(),
        }
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
