//! Contains the [`MultiOp`] type.

use std::fmt::{Display, Formatter};

/// An operation with multiple operands.
///
/// ## Usage
/// Used by [`MetricFExp`](crate::MetricFExp) and [`FExpDa`](crate::FExpDa).
/// Implicitly used by [`BinaryOp`](crate::BinaryOp).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MultiOp {
    Multiplication,
    Addition,
}

pub mod names {
    pub const MULTIPLICATION: &'static str = "*";
    pub const ADDITION: &'static str = "+";
}

impl Display for MultiOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiOp::Multiplication => write!(f, "{}", names::MULTIPLICATION),
            MultiOp::Addition => write!(f, "{}", names::ADDITION),
        }
    }
}

impl TryFrom<&str> for MultiOp {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::MULTIPLICATION => Ok(Self::Multiplication),
            names::ADDITION => Ok(Self::Addition),
            _ => Err(ParseError::InvalidOperation),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid operation")]
    InvalidOperation,
}
