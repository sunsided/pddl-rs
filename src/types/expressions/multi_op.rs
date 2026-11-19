//! Contains the [`MultiOp`] type.

use crate::types::keyword_error::UnknownKeyword;
use std::fmt::{Display, Formatter};

/// An operation with multiple operands.
///
/// ## Usage
/// Used by [`MetricFluentExpression`](crate::MetricFluentExpression) and [`FExpDa`](crate::FExpDa).
/// Implicitly used by [`BinaryOp`](crate::BinaryOp).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MultiOp {
    /// The `*` operation.
    Multiplication,
    /// The `+` operation.
    Addition,
}

/// The PDDL keywords for [`MultiOp`].
pub mod names {
    /// The `*` operation.
    pub const MULTIPLICATION: &str = "*";
    /// The `+` operation.
    pub const ADDITION: &str = "+";
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
    type Error = UnknownKeyword;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::MULTIPLICATION => Ok(Self::Multiplication),
            names::ADDITION => Ok(Self::Addition),
            _ => Err(UnknownKeyword::new("multi-operand operator", value)),
        }
    }
}
