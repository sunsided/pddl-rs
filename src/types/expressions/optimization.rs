//! Contains the [`Optimization`] type.

use std::fmt::{Display, Formatter};

/// An optimization instruction.
///
/// ## Requirements
/// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
///
/// ## Usage
/// Used by [`MetricSpec`](crate::MetricSpec).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Optimization {
    /// The associated metric should be minimized.
    Minimize,
    /// The associated metric should be maximized.
    Maximize,
}

/// The PDDL keywords for [`Optimization`].
pub mod names {
    /// The `minimize` keyword.
    pub const MINIMIZE: &str = "minimize";
    /// The `maximize` keyword.
    pub const MAXIMIZE: &str = "maximize";
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

/// An error when parsing an [`Optimization`] from a string.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid goal")]
    /// The string is neither `minimize` nor `maximize`.
    InvalidGoal,
}
