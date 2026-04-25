//! Contains binary comparison operations via the [`BinaryComparison`] type.

use std::fmt::{Display, Formatter};

/// A binary comparison operation.
///
/// ## Usage
/// Used by [`FluentComparison`](crate::FluentComparison).
#[doc(alias("binary-comp"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinaryComparison {
    GreaterThan,
    LessThan,
    Equal,
    GreaterOrEqual,
    LessThanOrEqual,
}

pub mod names {
    pub const GREATER_THAN: &str = ">";
    pub const LESS_THAN: &str = "<";
    pub const EQUAL: &str = "=";
    pub const GREATER_THAN_OR_EQUAL: &str = ">=";
    pub const LESS_THAN_OR_EQUAL: &str = "<=";
}

impl Display for BinaryComparison {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryComparison::GreaterThan => write!(f, "{}", names::GREATER_THAN),
            BinaryComparison::LessThan => write!(f, "{}", names::LESS_THAN),
            BinaryComparison::Equal => write!(f, "{}", names::EQUAL),
            BinaryComparison::GreaterOrEqual => write!(f, "{}", names::GREATER_THAN_OR_EQUAL),
            BinaryComparison::LessThanOrEqual => write!(f, "{}", names::LESS_THAN_OR_EQUAL),
        }
    }
}

impl TryFrom<&str> for BinaryComparison {
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

/// Alias for [`BinaryComparison`].
#[deprecated(since = "0.2.0", note = "Use `BinaryComparison` instead")]
#[allow(dead_code)]
pub type BinaryComp = BinaryComparison;
