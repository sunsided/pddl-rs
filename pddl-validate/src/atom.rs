//! Ground atoms: a predicate or function symbol applied to objects.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

/// A predicate or function symbol applied to objects, such as `(at t1 home)` or `(fuel t1)`.
///
/// The validator stores every name in ASCII lowercase, because PDDL names are
/// case-insensitive; [`Atom::new`] lowercases its input to match. Atoms order by
/// symbol, then by arguments, so traces and diagnostics list them deterministically.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct Atom {
    /// The predicate or function symbol, in ASCII lowercase.
    pub symbol: String,
    /// The objects the symbol is applied to, in ASCII lowercase. Empty for a nullary symbol.
    pub arguments: Vec<String>,
}

impl Atom {
    /// Builds an atom, lowercasing the symbol and every argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use pddl_validate::Atom;
    ///
    /// let atom = Atom::new("AT", ["T1", "home"]);
    /// assert_eq!(atom.to_string(), "(at t1 home)");
    /// ```
    #[must_use]
    pub fn new<S, I, A>(symbol: S, arguments: I) -> Self
    where
        S: AsRef<str>,
        I: IntoIterator<Item = A>,
        A: AsRef<str>,
    {
        Self {
            symbol: lower(symbol.as_ref()),
            arguments: arguments.into_iter().map(|a| lower(a.as_ref())).collect(),
        }
    }

    /// Builds an atom from names that are lowercase already.
    pub(crate) const fn lowered(symbol: String, arguments: Vec<String>) -> Self {
        Self { symbol, arguments }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}", self.symbol)?;
        for argument in &self.arguments {
            write!(f, " {argument}")?;
        }

        f.write_str(")")
    }
}

/// Returns `name` in ASCII lowercase, the form in which the validator compares PDDL names.
pub(crate) fn lower(name: &str) -> String {
    name.to_ascii_lowercase()
}
