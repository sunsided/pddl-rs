//! Contains names via the [`Name`] type.

use crate::types::{PrimitiveType, ToTyped, Type, Typed};
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[cfg(feature = "interning")]
use std::collections::HashSet;
#[cfg(feature = "interning")]
use std::sync::{Arc, LazyLock, Mutex};

/// The global interner used by [`Name::new`] when the `interning` feature is enabled.
#[cfg(feature = "interning")]
static STRING_INTERNING: LazyLock<Mutex<Interner>> = LazyLock::new(Mutex::default);

/// The interned spellings, and the size at which unused ones are dropped next.
#[cfg(feature = "interning")]
#[derive(Default)]
struct Interner {
    /// The interned spellings.
    names: HashSet<Arc<str>>,
    /// The set size at which unused spellings are purged next.
    purge_at: usize,
}

#[cfg(feature = "interning")]
impl Interner {
    /// Returns the shared copy of `name`, inserting it if it is new.
    ///
    /// Spellings no [`Name`] uses any more are dropped only here, on a miss, whenever
    /// the set has doubled since the last purge. Reclamation is lazy: names dropped
    /// while no new spellings are interned stay resident until later inserts.
    fn intern(&mut self, name: &str) -> Arc<str> {
        if let Some(interned) = self.names.get(name) {
            return interned.clone();
        }

        // A count of 1 means only the set holds it; no `Name` exists to clone it meanwhile.
        if self.names.len() >= self.purge_at {
            self.names.retain(|s| Arc::strong_count(s) > 1);
            self.purge_at = (2 * self.names.len()).max(1024);
        }

        let interned: Arc<str> = Arc::from(name);
        self.names.insert(interned.clone());
        interned
    }
}

/// A name.
///
/// ## Usage
/// Used by [`Domain`](crate::Domain), [`InitElement`](crate::InitElement),
/// [`BasicFunctionTerm`](crate::BasicFunctionTerm), [`MetricFluentExpression`](crate::MetricFluentExpression),
/// [`PrimitiveType`], [`Predicate`](crate::Predicate), [`Variable`](crate::Variable),
/// [`FunctionSymbol`](crate::FunctionSymbol), [`ActionSymbol`](crate::ActionSymbol),
/// [`PreferenceName`](crate::PreferenceName), [`Term`](crate::Term),
/// [`DurativeActionSymbol`](crate::DurativeActionSymbol) and [`Objects`](crate::Objects).
///
/// ## Case
/// PDDL names are case-insensitive. A [`Name`] keeps the spelling it was
/// created with (e.g. for pretty-printing), but compares and hashes
/// ignoring ASCII case, so `AT` and `at` are the same name.
///
/// Its hash is therefore that of the lowercased name, not that of
/// [`as_ref()`](AsRef::as_ref): don't look up a `Name`-keyed map or set with
/// `&str` keys (which is also why `Name` does not implement `Borrow<str>`).
///
/// Storage is per spelling: [`Name::new`] interns `AT` and `at` separately,
/// and only the exact spellings `object` and `number` map to the built-in
/// static names. The two values still compare equal.
///
/// ```
/// # use pddl::Name;
/// assert_eq!(Name::new("AT"), Name::new("at"));
/// assert_eq!(Name::new("AT"), "at");
/// assert_eq!(Name::new("AT").to_string(), "AT");
/// ```
#[derive(Debug, Clone, Default)]
pub struct Name(NameVariant);

/// Storage only: name identity (equality, hashing) is defined on [`Name`], which ignores case.
#[derive(Clone)]
enum NameVariant {
    /// A dynamically created name; deduplicated when the `interning` feature is enabled.
    String(InternedString),
    /// A `'static` name, e.g. a well-known name or one from [`Name::new_static`].
    Static(&'static str),
}

/// Shared, reference-counted storage of an interned spelling.
#[cfg(feature = "interning")]
type InternedString = Arc<str>;

/// Owned storage of a spelling (the `interning` feature is disabled).
#[cfg(not(feature = "interning"))]
type InternedString = String;

impl Name {
    /// Constructs a new [`Name`] from a provided string.
    ///
    /// ## Interning
    /// If the `interning` crate feature is enabled, strings passed to this
    /// method will be deduplicated, reducing memory footprint. Deduplication
    /// is by exact spelling, so `AT` and `at` are stored separately (but still
    /// compare equal, see [`Name`]).
    ///
    /// ## Arguments
    /// * `name` - The name to wrap.
    ///
    /// ## Returns
    /// A new [`Name`].
    #[inline]
    #[must_use]
    pub fn new<S: Into<String> + AsRef<str>>(name: S) -> Self {
        if let Some(str) = Self::map_to_static(name.as_ref()) {
            Self::new_static(str)
        } else {
            Self::new_string_interned(name)
        }
    }

    /// Like [`Self::new`] but makes use of the fact that if the string provided
    /// is `'static`, the method can be `const`.
    ///
    /// ## Interning
    /// Note that strings passed to this method are not themselves interned
    /// even if the create feature `interning` is enabled, and they will be invisible
    /// to other strings partaking in interning. To ensure that each spelling
    /// is stored only once, use the (non-const) [`Name::new`] function instead.
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
    #[inline]
    #[must_use]
    pub const fn new_static(name: &'static str) -> Self {
        Self(NameVariant::Static(name))
    }

    /// Takes the provided `name` and interns the string.
    ///
    /// Looks the spelling up in a global hash set and inserts it if it wasn't found
    /// before. Unused spellings are dropped lazily on later inserts, so memory stays
    /// within about twice the names alive while interning continues.
    #[cfg_attr(not(feature = "interning"), inline)]
    #[allow(
        clippy::needless_pass_by_value,
        reason = "without the `interning` feature, the string is consumed"
    )]
    fn new_string_interned<S: Into<String> + AsRef<str>>(name: S) -> Self {
        #[cfg(feature = "interning")]
        {
            let mut interner = STRING_INTERNING.lock().expect("failed to obtain lock");
            Self(NameVariant::String(interner.intern(name.as_ref())))
        }

        #[cfg(not(feature = "interning"))]
        {
            Self(NameVariant::String(name.into()))
        }
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
    /// The built-in `object` type name.
    pub static OBJECT: &str = "object";
    /// The built-in `number` type name.
    pub static NUMBER: &str = "number";
}

impl<T> From<T> for Name
where
    T: Into<String> + AsRef<str>,
{
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl AsRef<str> for Name {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for Name {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Name {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.eq_ignore_ascii_case(other)
    }
}

impl Eq for Name {}

impl Hash for Name {
    /// Hashes the ASCII-lowercased name, consistent with the case-insensitive [`PartialEq`].
    fn hash<H: Hasher>(&self, state: &mut H) {
        for byte in self.bytes() {
            state.write_u8(byte.to_ascii_lowercase());
        }
        // Terminator, as `str`'s own `Hash` does, so ("ab", "c") and ("a", "bc") differ in tuples.
        state.write_u8(0xff);
    }
}

impl PartialEq<str> for Name {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.eq_ignore_ascii_case(other)
    }
}

impl PartialEq<&str> for Name {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self.eq_ignore_ascii_case(other)
    }
}

impl PartialEq<String> for Name {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        self.eq_ignore_ascii_case(other)
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

impl Default for NameVariant {
    fn default() -> Self {
        Self::Static("")
    }
}

impl Deref for NameVariant {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            NameVariant::String(str) => str,
            NameVariant::Static(str) => str,
        }
    }
}

impl Debug for NameVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Display for NameVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NameVariant::String(str) => write!(f, "{str}"),
            NameVariant::Static(str) => write!(f, "{str}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn len_is_in_bytes_like_str() {
        // `&name[..name.len()]` must not panic on non-ASCII names.
        let name = Name::new("é");
        check!(name.len() == 2);
        check!(&name[..name.len()] == "é");
        check!(Name::default().is_empty());
    }

    #[test]
    fn eq_and_hash_ignore_ascii_case() {
        use std::collections::HashSet;
        let names: HashSet<Name> = ["AT", "at", "At", "next", "NEXT"]
            .into_iter()
            .map(Name::from)
            .collect();
        check!(names.len() == 2);
        check!(names.contains(&Name::from("aT")));
        check!(Name::from("ab") != Name::from("abc"));
    }

    #[test]
    fn keeps_original_spelling() {
        check!(&*Name::from("NeXt") == "NeXt");
        check!(Name::from("NeXt").to_string() == "NeXt");
    }

    #[cfg(feature = "interning")]
    #[test]
    fn unused_spellings_are_dropped() {
        // A local interner, so tests interning names in parallel can't interfere.
        let mut interner = Interner::default();
        let kept = interner.intern("kept");
        for i in 0..10_000 {
            drop(interner.intern(&format!("unused-{i}")));
        }
        check!(interner.names.len() <= 1024, "{}", interner.names.len());
        check!(!interner.names.contains("unused-0"));
        check!(Arc::ptr_eq(&kept, &interner.intern("kept")));
    }

    #[cfg(feature = "interning")]
    #[test]
    fn same_spelling_shares_storage() {
        let (a, b) = (Name::new("shared-spelling"), Name::new("shared-spelling"));
        check!(std::ptr::eq(a.as_ptr(), b.as_ptr()));
    }

    #[test]
    fn test_display() {
        let name = Name::new("x");
        check!(format!("{name}") == "x");
    }

    #[test]
    fn map_to_static_works() {
        let object = Name::map_to_static("object").expect("mapping works");
        let number = Name::map_to_static("number").expect("mapping works");
        check!(object == well_known::OBJECT);
        check!(number == well_known::NUMBER);
    }
}
