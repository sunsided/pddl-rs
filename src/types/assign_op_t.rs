//! Contains the timed effect assignment operation type [`AssignOpT`].

use std::fmt::{Display, Formatter};

/// An assignment operation.
///
/// ## Usage
/// Used by [`TimedEffect`](crate::TimedEffect).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AssignOpT {
    Increase,
    Decrease,
}

pub mod names {
    pub const INCREASE: &str = "increase";
    pub const DECREASE: &str = "decrease";
}

impl Display for AssignOpT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignOpT::Increase => write!(f, "{}", names::INCREASE),
            AssignOpT::Decrease => write!(f, "{}", names::DECREASE),
        }
    }
}

impl TryFrom<&str> for AssignOpT {
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
