//! Contains the [`DurationOperator`] type.

use std::fmt::{Display, Formatter};

/// ## Usage
/// Used by [`SimpleDurationConstraint`](crate::SimpleDurationConstraint).
#[doc(alias("d-op"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DurationOperator {
    /// The `=` operator: the duration equals the value.
    Equal,
    /// ## Requirements
    /// Requires [Duration Inequalities](crate::Requirement::DurationInequalities);
    GreaterOrEqual,
    /// ## Requirements
    /// Requires [Duration Inequalities](crate::Requirement::DurationInequalities);
    LessThanOrEqual,
}

/// The PDDL keywords for [`DurationOperator`].
pub mod names {
    /// The `=` operator.
    pub const EQUAL: &str = "=";
    /// The `>=` operator.
    pub const GREATER_OR_EQUAL: &str = ">=";
    /// The `<=` operator.
    pub const LESS_THAN_OR_EQUAL: &str = "<=";
}

impl Display for DurationOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DurationOperator::Equal => write!(f, "{}", names::EQUAL),
            DurationOperator::GreaterOrEqual => write!(f, "{}", names::GREATER_OR_EQUAL),
            DurationOperator::LessThanOrEqual => write!(f, "{}", names::LESS_THAN_OR_EQUAL),
        }
    }
}

impl TryFrom<&str> for DurationOperator {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::EQUAL => Ok(Self::Equal),
            names::GREATER_OR_EQUAL => Ok(Self::GreaterOrEqual),
            names::LESS_THAN_OR_EQUAL => Ok(Self::LessThanOrEqual),
            _ => Err(ParseError::InvalidOperation),
        }
    }
}

/// An error when parsing a [`DurationOperator`] from a string.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid operation")]
    /// The string is not a valid duration operator.
    InvalidOperation,
}

/// Alias for [`DurationOperator`]; matches BNF `<d-op>`.
#[deprecated(since = "0.2.0", note = "Use `DurationOperator` instead")]
pub type DOp = DurationOperator;
