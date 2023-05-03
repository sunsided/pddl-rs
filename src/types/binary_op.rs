//! Contains binary operations via the [`BinaryOp`] type.

use crate::types::MultiOp;
use std::fmt::{Display, Formatter};

/// A binary operation.
///
/// ## Usage
/// Used by [`FExp`](crate::Fexp), [`FExpDa`](crate::FExpDa) and [`Optimization`](crate::Optimization).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinaryOp {
    Multiplication,
    Addition,
    Subtraction,
    Division,
}

pub mod names {
    pub const MULTIPLICATION: &'static str = "*";
    pub const ADDITION: &'static str = "+";
    pub const SUBTRACTION: &'static str = "-";
    pub const DIVISION: &'static str = "/";
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Multiplication => write!(f, "{}", names::MULTIPLICATION),
            BinaryOp::Addition => write!(f, "{}", names::ADDITION),
            BinaryOp::Subtraction => write!(f, "{}", names::SUBTRACTION),
            BinaryOp::Division => write!(f, "{}", names::DIVISION),
        }
    }
}

impl TryFrom<&str> for BinaryOp {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::MULTIPLICATION => Ok(Self::Multiplication),
            names::ADDITION => Ok(Self::Addition),
            names::SUBTRACTION => Ok(Self::Subtraction),
            names::DIVISION => Ok(Self::Division),
            _ => Err(ParseError::InvalidOperation),
        }
    }
}

impl From<MultiOp> for BinaryOp {
    fn from(value: MultiOp) -> Self {
        match value {
            MultiOp::Multiplication => BinaryOp::Multiplication,
            MultiOp::Addition => BinaryOp::Addition,
        }
    }
}

impl TryFrom<BinaryOp> for MultiOp {
    type Error = TryFromError;

    fn try_from(value: BinaryOp) -> Result<Self, Self::Error> {
        match value {
            BinaryOp::Multiplication => Ok(MultiOp::Multiplication),
            BinaryOp::Addition => Ok(MultiOp::Addition),
            _ => Err(TryFromError::NotAMultiOp),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid operation")]
    InvalidOperation,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum TryFromError {
    #[error("Operation is not a MultiOp")]
    NotAMultiOp,
}
