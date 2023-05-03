//! Contains the [`DOp`] type.

use std::fmt::{Display, Formatter};

/// ## Usage
/// Used by [`SimpleDurationConstraint`](crate::SimpleDurationConstraint).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DOp {
    Equal,
    /// ## Requirements
    /// Requires [Duration Inequalities](crate::Requirement::DurationInequalities);
    GreaterOrEqual,
    /// ## Requirements
    /// Requires [Duration Inequalities](crate::Requirement::DurationInequalities);
    LessThanOrEqual,
}

pub mod names {
    pub const EQUAL: &'static str = "=";
    pub const GREATER_OR_EQUAL: &'static str = ">=";
    pub const LESS_THAN_OR_EQUAL: &'static str = "<=";
}

impl Display for DOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DOp::Equal => write!(f, "{}", names::EQUAL),
            DOp::GreaterOrEqual => write!(f, "{}", names::GREATER_OR_EQUAL),
            DOp::LessThanOrEqual => write!(f, "{}", names::LESS_THAN_OR_EQUAL),
        }
    }
}

impl TryFrom<&str> for DOp {
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

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid operation")]
    InvalidOperation,
}
