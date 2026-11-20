//! Errors in the domain or problem, found before any plan runs.

use alloc::string::String;
use core::fmt;

/// The domain or problem is malformed, so no plan can be validated against it.
///
/// [`Validator::new`](crate::Validator::new) checks every name and arity up front, so a
/// plan's verdict never depends on a typo in the domain.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ModelError {
    /// Where the problem is, such as ``action `drive` `` or `goal`.
    pub location: String,
    /// What is wrong.
    pub kind: ModelErrorKind,
}

impl ModelError {
    /// Builds an error at `location`.
    pub(crate) fn new(location: impl Into<String>, kind: ModelErrorKind) -> Self {
        Self {
            location: location.into(),
            kind,
        }
    }
}

/// The kinds of [`ModelError`]. All names are in ASCII lowercase.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ModelErrorKind {
    /// The problem names another domain in `(:domain ...)`.
    DomainMismatch {
        /// The name of the domain given to the validator.
        domain: String,
        /// The domain the problem names.
        problem_domain: String,
    },
    /// A type is used but not declared.
    UnknownType(String),
    /// A predicate is used but not declared.
    UnknownPredicate(String),
    /// A function is used but not declared.
    UnknownFunction(String),
    /// An object or constant is used but not declared.
    UnknownObject(String),
    /// A variable is used outside the parameters and quantifiers that bind it.
    UnboundVariable(String),
    /// A predicate or function is applied to the wrong number of arguments.
    Arity {
        /// The predicate or function.
        symbol: String,
        /// The number of parameters it is declared with.
        expected: usize,
        /// The number of arguments it is applied to.
        found: usize,
    },
    /// Two actions share a name.
    DuplicateAction(String),
    /// An action effect or the initial state sets a derived predicate, which only its
    /// rule may define.
    DerivedPredicateSet(String),
    /// An effect or the initial state sets an equality `(= a b)`.
    EqualitySet,
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.location, self.kind)
    }
}

impl fmt::Display for ModelErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DomainMismatch {
                domain,
                problem_domain,
            } => write!(
                f,
                "the problem is for domain `{problem_domain}`, not `{domain}`"
            ),
            Self::UnknownType(name) => write!(f, "unknown type `{name}`"),
            Self::UnknownPredicate(name) => write!(f, "unknown predicate `{name}`"),
            Self::UnknownFunction(name) => write!(f, "unknown function `{name}`"),
            Self::UnknownObject(name) => write!(f, "unknown object `{name}`"),
            Self::UnboundVariable(name) => write!(f, "unbound variable `?{name}`"),
            Self::Arity {
                symbol,
                expected,
                found,
            } => write!(
                f,
                "`{symbol}` takes {expected} arguments but is given {found}"
            ),
            Self::DuplicateAction(name) => write!(f, "action `{name}` is defined twice"),
            Self::DerivedPredicateSet(name) => {
                write!(f, "derived predicate `{name}` cannot be set directly")
            }
            Self::EqualitySet => f.write_str("`=` cannot be set"),
        }
    }
}

impl core::error::Error for ModelError {}
