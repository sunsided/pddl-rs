//! Contains the [`Interval`] type.

use std::fmt::{Display, Formatter};

/// An interval used in [`TimedGD::Over`](crate::types::TimedGD::Over).
///
/// ## Usage
/// Used by [`TimedGD`](crate::TimedGD).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Interval {
    /// The condition must be true during the entire plan.
    All,
}

/// The PDDL keywords for [`Interval`].
pub mod names {
    /// The `all` interval.
    pub const ALL: &str = "all";
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

/// An error when parsing an [`Interval`] from a string.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid interval")]
    /// The string is not a valid interval.
    InvalidInterval,
}
