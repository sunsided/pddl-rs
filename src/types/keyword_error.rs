//! Contains the [`UnknownKeyword`] and [`NotAMultiOp`] errors of the keyword types.

use crate::BinaryOp;

/// A string that is not one of the keywords of a PDDL type, e.g. `"incr"` for an
/// [`AssignOp`](crate::AssignOp).
///
/// The error of every keyword type's `TryFrom<&str>`.
///
/// ## Example
/// ```
/// # use pddl::AssignOp;
/// let error = AssignOp::try_from("incr").unwrap_err();
/// assert_eq!(error.found(), "incr");
/// assert_eq!(error.expected(), "assignment operator");
/// assert_eq!(error.to_string(), "`incr` is not a valid assignment operator");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("`{found}` is not a valid {expected}")]
pub struct UnknownKeyword {
    /// What kind of keyword was expected, e.g. `"assignment operator"`.
    expected: &'static str,
    /// The string that was found instead.
    found: String,
}

impl UnknownKeyword {
    /// Creates the error for `found`, which is not a valid `expected`.
    pub(crate) fn new(expected: &'static str, found: &str) -> Self {
        Self {
            expected,
            found: found.to_string(),
        }
    }

    /// Returns what kind of keyword was expected, e.g. `"assignment operator"`.
    #[must_use]
    pub const fn expected(&self) -> &'static str {
        self.expected
    }

    /// Returns the string that was found instead.
    #[must_use]
    pub fn found(&self) -> &str {
        &self.found
    }
}

/// A [`BinaryOp`] that has no [`MultiOp`](crate::MultiOp) equivalent: `-` or `/`.
///
/// The error of `MultiOp::try_from(BinaryOp)`.
///
/// ## Example
/// ```
/// # use pddl::{BinaryOp, MultiOp};
/// let error = MultiOp::try_from(BinaryOp::Division).unwrap_err();
/// assert_eq!(error.op(), BinaryOp::Division);
/// assert_eq!(error.to_string(), "`/` is not a multi-operand operation");
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, thiserror::Error)]
#[error("`{0}` is not a multi-operand operation")]
pub struct NotAMultiOp(pub(crate) BinaryOp);

impl NotAMultiOp {
    /// Returns the operation that has no [`MultiOp`](crate::MultiOp) equivalent.
    #[must_use]
    pub const fn op(&self) -> BinaryOp {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use assert2::check;

    /// Every keyword type reports the input and what it expected.
    #[test]
    fn keyword_types_report_unknown_keywords() {
        macro_rules! unknown {
            ($ty:ty, $expected:literal) => {{
                let error = <$ty>::try_from("incr").unwrap_err();
                check!(error == UnknownKeyword::new($expected, "incr"));
                check!(error.to_string() == format!("`incr` is not a valid {}", $expected));
            }};
        }
        unknown!(AssignOp, "assignment operator");
        unknown!(TimedAssignOperator, "timed assignment operator");
        unknown!(BinaryComparison, "binary comparison");
        unknown!(BinaryOp, "binary operator");
        unknown!(MultiOp, "multi-operand operator");
        unknown!(Optimization, "optimization");
        unknown!(DurationOperator, "duration operator");
        unknown!(Interval, "interval");
        unknown!(TimeSpecifier, "time specifier");
        unknown!(Requirement, "requirement key");
    }

    /// Only `-` and `/` have no multi-operand form.
    #[test]
    fn subtraction_is_not_a_multi_op() {
        check!(MultiOp::try_from(BinaryOp::Subtraction) == Err(NotAMultiOp(BinaryOp::Subtraction)));
        check!(MultiOp::try_from(BinaryOp::Addition) == Ok(MultiOp::Addition));
    }
}
