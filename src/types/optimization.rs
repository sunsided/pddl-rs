//! Contains the [`Optimization`] type.

use std::fmt::{Display, Formatter};

/// An optimization goal.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Optimization {
    Minimize,
    Maximize,
}

pub mod names {
    pub const MINIMIZE: &'static str = "minimize";
    pub const MAXIMIZE: &'static str = "maximize";
}

impl Display for Optimization {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Optimization::Minimize => write!(f, "{}", names::MINIMIZE),
            Optimization::Maximize => write!(f, "{}", names::MAXIMIZE),
        }
    }
}

impl TryFrom<&str> for Optimization {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::MINIMIZE => Ok(Self::Minimize),
            names::MAXIMIZE => Ok(Self::Maximize),
            _ => Err(ParseError::InvalidGoal),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid goal")]
    InvalidGoal,
}
