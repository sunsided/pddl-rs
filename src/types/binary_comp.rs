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

pub mod names {
    pub const GREATER_THAN: &'static str = ">";
    pub const LESS_THAN: &'static str = "<";
    pub const EQUAL: &'static str = "=";
    pub const GREATER_THAN_OR_EQUAL: &'static str = ">=";
    pub const LESS_THAN_OR_EQUAL: &'static str = "<=";
}

impl Display for BinaryComp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryComp::GreaterThan => write!(f, "{}", names::GREATER_THAN),
            BinaryComp::LessThan => write!(f, "{}", names::LESS_THAN),
            BinaryComp::Equal => write!(f, "{}", names::EQUAL),
            BinaryComp::GreaterOrEqual => write!(f, "{}", names::GREATER_THAN_OR_EQUAL),
            BinaryComp::LessThanOrEqual => write!(f, "{}", names::LESS_THAN_OR_EQUAL),
        }
    }
}

impl TryFrom<&str> for BinaryComp {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::GREATER_THAN => Ok(Self::GreaterThan),
            names::LESS_THAN => Ok(Self::LessThan),
            names::EQUAL => Ok(Self::Equal),
            names::GREATER_THAN_OR_EQUAL => Ok(Self::GreaterOrEqual),
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
