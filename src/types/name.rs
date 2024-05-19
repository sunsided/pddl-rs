//! Contains names via the [`Name`] type.

use crate::types::{PrimitiveType, ToTyped, Type, Typed};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

#[cfg(feature = "interning")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "interning")]
lazy_static::lazy_static! {
    /// Used in [`Name::new_string_interned`] to deduplicate string occurrences.
    static ref STRING_INTERNING: Mutex<Vec<Arc<String>>> = Mutex::new(Vec::default());
}

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
    String(InternedString),
    Static(&'static str),
}

#[cfg(feature = "interning")]
type InternedString = Arc<String>;

#[cfg(not(feature = "interning"))]
type InternedString = String;

impl Name {
    /// Constructs a new [`Name`] from a provided string.
    ///
    /// ## Interning
    /// If the `interning` crate feature is enabled, strings passed to this
    /// method will be deduplicated, reducing memory footprint.
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
            Self::new_string_interned(name)
        }
    }

    /// Like [`new`] but makes use of the fact that if the string provided
    /// is `'static`, the method can be `const`.
    ///
    /// ## Interning
    /// Note that strings passed to this method are not themselves interned
    /// even if the create feature `interning` is enabled, and they will be invisible
    /// to other strings partaking in interning. To ensure that a name value
    /// exists exactly once, use the (non-const) [`Name::new`] function instead.
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

    /// Takes the provided `name` and interns the string.
    ///
    /// This uses a simple binary search approach to identify the correct position of
    /// the input in question and inserts the element if it wasn't found before.
    #[cfg_attr(not(feature = "interning"), inline(always))]
    fn new_string_interned<S: Into<String> + AsRef<str>>(name: S) -> Self {
        #[cfg(feature = "interning")]
        {
            let mut guard = STRING_INTERNING.lock().expect("failed to obtain lock");
            let name_ref = name.as_ref();
            let pos = guard.binary_search_by(|name| name_ref.cmp(name.as_str()));
            let pos = match pos {
                Ok(pos) => pos,
                Err(pos) => {
                    guard.insert(pos, Arc::new(name.into()));
                    pos
                }
            };

            Self(NameVariant::String(guard[pos].clone()))
        }

        #[cfg(not(feature = "interning"))]
        {
            Self(NameVariant::String(name.into()))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Gets the length of the name, in chars.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Maps the provided to a well-known `'static` string if possible.
    fn map_to_static(value: &str) -> Option<&'static str> {
        match value {
            "object" => Some(well_known::OBJECT),
            "number" => Some(well_known::NUMBER),
            _ => None,
        }
    }
}

/// Provides well-known names for string interning.
mod well_known {
    pub static OBJECT: &str = "object";
    pub static NUMBER: &str = "number";
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
            NameVariant::Static(str) => (*str).eq(other),
            NameVariant::String(str) => {
                #[cfg(feature = "interning")]
                {
                    str.as_ref().eq(other)
                }
                #[cfg(not(feature = "interning"))]
                {
                    str.eq(other)
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use nom_greedyerror::AsStr;

    #[test]
    fn map_to_static_works() {
        let object = Name::map_to_static("object").expect("mapping works");
        let number = Name::map_to_static("number").expect("mapping works");
        assert!(std::ptr::eq(object.as_str(), well_known::OBJECT));
        assert!(std::ptr::eq(number.as_str(), well_known::NUMBER));
    }
}
