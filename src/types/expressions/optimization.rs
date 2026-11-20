//! Contains the [`Optimization`] type.

use crate::types::keyword_error::UnknownKeyword;
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
    type Error = UnknownKeyword;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        UnknownKeyword::lookup(
            "optimization",
            value,
            &[
                (names::MINIMIZE, Self::Minimize),
                (names::MAXIMIZE, Self::Maximize),
            ],
        )
    }
}
