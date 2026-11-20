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
    /// One parameter or quantifier list declares a variable twice, such as `(?x ?x)`.
    DuplicateVariable(String),
    /// An action effect or the initial state sets a derived predicate, which only its
    /// rule may define.
    DerivedPredicateSet(String),
    /// An effect or the initial state sets an equality `(= a b)`.
    EqualitySet,
    /// Two timed initial literals at the same time make one atom both true and false,
    /// such as `(at 1 (q))` and `(at 1 (not (q)))`.
    ContradictoryTimedLiterals {
        /// The ground atom, such as `(at r1 base)`.
        atom: String,
        /// The time of the first of the two literals, formatted as a decimal number.
        time: String,
    },
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
            Self::DuplicateVariable(name) => write!(f, "variable `?{name}` is declared twice"),
            Self::DerivedPredicateSet(name) => {
                write!(f, "derived predicate `{name}` cannot be set directly")
            }
            Self::EqualitySet => f.write_str("`=` cannot be set"),
            Self::ContradictoryTimedLiterals { atom, time } => {
                write!(
                    f,
                    "timed literals make `{atom}` both true and false at {time}"
                )
            }
        }
    }
}

impl core::error::Error for ModelError {}

#[cfg(test)]
mod tests {
    //! Pins the exact text of every model error, as users read it.

    use alloc::string::ToString;

    use super::*;

    #[test]
    fn model_errors_display_location_and_kind() {
        let name = || "drive".to_string();
        let cases = [
            (
                ModelErrorKind::DomainMismatch {
                    domain: "trucks".to_string(),
                    problem_domain: "boats".to_string(),
                },
                "the problem is for domain `boats`, not `trucks`",
            ),
            (ModelErrorKind::UnknownType(name()), "unknown type `drive`"),
            (
                ModelErrorKind::UnknownPredicate(name()),
                "unknown predicate `drive`",
            ),
            (
                ModelErrorKind::UnknownFunction(name()),
                "unknown function `drive`",
            ),
            (
                ModelErrorKind::UnknownObject(name()),
                "unknown object `drive`",
            ),
            (
                ModelErrorKind::UnboundVariable(name()),
                "unbound variable `?drive`",
            ),
            (
                ModelErrorKind::Arity {
                    symbol: name(),
                    expected: 2,
                    found: 3,
                },
                "`drive` takes 2 arguments but is given 3",
            ),
            (
                ModelErrorKind::DuplicateAction(name()),
                "action `drive` is defined twice",
            ),
            (
                ModelErrorKind::DuplicateVariable(name()),
                "variable `?drive` is declared twice",
            ),
            (
                ModelErrorKind::DerivedPredicateSet(name()),
                "derived predicate `drive` cannot be set directly",
            ),
            (ModelErrorKind::EqualitySet, "`=` cannot be set"),
            (
                ModelErrorKind::ContradictoryTimedLiterals {
                    atom: "(open shop)".to_string(),
                    time: "1.5".to_string(),
                },
                "timed literals make `(open shop)` both true and false at 1.5",
            ),
        ];

        for (kind, text) in cases {
            let error = ModelError::new("action `drive`", kind);
            assert_eq!(error.to_string(), alloc::format!("action `drive`: {text}"));
        }
    }
}
