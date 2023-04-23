//! Contains binary comparison operations.

use std::fmt::{Display, Formatter};

/// A binary comparison operation.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinaryComp {
    GreaterThan,
    LessThan,
    Equal,
    GreaterOrEqual,
    LessThanOrEqual,
}

impl Display for BinaryComp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryComp::GreaterThan => write!(f, ">"),
            BinaryComp::LessThan => write!(f, "<"),
            BinaryComp::Equal => write!(f, "="),
            BinaryComp::GreaterOrEqual => write!(f, ">="),
            BinaryComp::LessThanOrEqual => write!(f, "<="),
        }
    }
}

impl TryFrom<&str> for BinaryComp {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ">" => Ok(Self::GreaterThan),
            "<" => Ok(Self::LessThan),
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
