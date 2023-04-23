//! Contains the BinaryOp type.

use crate::types::domain::MultiOp;
use std::fmt::{Display, Formatter};

/// A binary operation.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinaryOp {
    Multiplication,
    Addition,
    Subtraction,
    Division,
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Multiplication => write!(f, "*"),
            BinaryOp::Addition => write!(f, "+"),
            BinaryOp::Subtraction => write!(f, "-"),
            BinaryOp::Division => write!(f, "/"),
        }
    }
}

impl TryFrom<&str> for BinaryOp {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "*" => Ok(Self::Multiplication),
            "+" => Ok(Self::Addition),
            "-" => Ok(Self::Subtraction),
            "/" => Ok(Self::Division),
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
