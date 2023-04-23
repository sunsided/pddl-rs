//! Contains the [`Interval`] type.

use std::fmt::{Display, Formatter};

/// An interval.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Interval {
    All,
}

pub mod names {
    pub const ALL: &'static str = "all";
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::All => write!(f, "{}", names::ALL),
        }
    }
}

impl TryFrom<&str> for Interval {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::ALL => Ok(Self::All),
            _ => Err(ParseError::InvalidInterval),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid interval")]
    InvalidInterval,
}