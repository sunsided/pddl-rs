//! Contains binary operations via the [`BinaryOp`] type.

use crate::types::MultiOp;
use std::fmt::{Display, Formatter};

/// A binary operation.
///
/// ## Usage
/// Used by [`FExp`](crate::FExp), [`FExpDa`](crate::FExpDa) and [`Optimization`](crate::Optimization).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BinaryOp {
    /// The `*` operation.
    Multiplication,
    /// The `+` operation.
    Addition,
    /// The `-` operation.
    Subtraction,
    /// The `/` operation.
    Division,
}

/// The PDDL keywords for [`BinaryOp`].
pub mod names {
    /// The `*` operation.
    pub const MULTIPLICATION: &str = "*";
    /// The `+` operation.
    pub const ADDITION: &str = "+";
    /// The `-` operation.
    pub const SUBTRACTION: &str = "-";
    /// The `/` operation.
    pub const DIVISION: &str = "/";
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

/// An error when parsing a [`BinaryOp`] from a string.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid operation")]
    /// The string is not a valid binary operator.
    InvalidOperation,
}

/// An error when converting a [`BinaryOp`] into a [`MultiOp`].
#[derive(Debug, Clone, thiserror::Error)]
pub enum TryFromError {
    #[error("Operation is not a MultiOp")]
    /// The operation is `-` or `/`, which has no [`MultiOp`] equivalent.
    NotAMultiOp,
}
