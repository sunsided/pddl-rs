//! Contains the [`DOp`] type..

use std::fmt::{Display, Formatter};

/// A binary comparison operation.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DOp {
    Equal,
    /// Requires [DurationInequalities](crate::types::Requirement::DurationInequalities);
    GreaterOrEqual,
    /// Requires [DurationInequalities](crate::types::Requirement::DurationInequalities);
    LessThanOrEqual,
}

impl Display for DOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DOp::Equal => write!(f, "="),
            DOp::GreaterOrEqual => write!(f, ">="),
            DOp::LessThanOrEqual => write!(f, "<="),
        }
    }
}

impl TryFrom<&str> for DOp {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "=" => Ok(Self::Equal),
            ">=" => Ok(Self::GreaterOrEqual),
            "<=" => Ok(Self::LessThanOrEqual),
            _ => Err(ParseError::InvalidOperation),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid operation")]
    InvalidOperation,
}
