//! Contains the [`TimeSpecifier`] type.

use crate::types::keyword_error::UnknownKeyword;
use std::fmt::{Display, Formatter};

/// A time specifier used in e.g. [`TimedGD::At`](crate::types::TimedGD::At) and [`TimedEffect`](crate::types::TimedEffect).
///
/// ## Usage
/// Used by [`TimedGD`](crate::TimedGD) and [`TimedEffect`](crate::TimedEffect).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TimeSpecifier {
    /// The condition or effect holds or applies at the beginning of a plan.
    Start,
    /// The condition or effect must hold or apply at the end of a plan.
    End,
}

/// The PDDL keywords for [`TimeSpecifier`].
pub mod names {
    /// The `start` time specifier.
    pub const START: &str = "start";
    /// The `end` time specifier.
    pub const END: &str = "end";
}

impl Display for TimeSpecifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeSpecifier::Start => write!(f, "{}", names::START),
            TimeSpecifier::End => write!(f, "{}", names::END),
        }
    }
}

impl TryFrom<&str> for TimeSpecifier {
    type Error = UnknownKeyword;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        UnknownKeyword::lookup(
            "time specifier",
            value,
            &[(names::START, Self::Start), (names::END, Self::End)],
        )
    }
}
