//! Contains the [`Interval`] type.

use crate::types::keyword_error::UnknownKeyword;
use std::fmt::{Display, Formatter};

/// An interval used in [`TimedGD::Over`](crate::types::TimedGD::Over).
///
/// ## Usage
/// Used by [`TimedGD`](crate::TimedGD).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    type Error = UnknownKeyword;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        UnknownKeyword::lookup("interval", value, &[(names::ALL, Self::All)])
    }
}
