//! Contains the timed effect assignment operation type [`TimedAssignOperator`].

use std::fmt::{Display, Formatter};

/// An assignment operation.
///
/// ## Usage
/// Used by [`TimedEffect`](crate::TimedEffect).
#[doc(alias("assign-op-t"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TimedAssignOperator {
    Increase,
    Decrease,
}

pub mod names {
    pub const INCREASE: &str = "increase";
    pub const DECREASE: &str = "decrease";
}

impl Display for TimedAssignOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimedAssignOperator::Increase => write!(f, "{}", names::INCREASE),
            TimedAssignOperator::Decrease => write!(f, "{}", names::DECREASE),
        }
    }
}

impl TryFrom<&str> for TimedAssignOperator {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::INCREASE => Ok(Self::Increase),
            names::DECREASE => Ok(Self::Decrease),
            _ => Err(ParseError::InvalidOperation),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid operation")]
    InvalidOperation,
}

/// Alias for [`TimedAssignOperator`]; matches BNF `<assign-op-t>`.
#[deprecated(since = "0.2.0", note = "Use `TimedAssignOperator` instead")]
pub type AssignOpT = TimedAssignOperator;
