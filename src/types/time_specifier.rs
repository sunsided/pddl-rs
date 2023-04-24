//! Contains the [`TimeSpecifier`] type.

use std::fmt::{Display, Formatter};

/// A time specifier used in e.g. [TimedGD::At](crate::types::TimedGD::At) and [TimedEffect](crate::types::TimedEffect).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TimeSpecifier {
    /// The condition or effect holds or applies at the beginning of a plan.
    Start,
    /// The condition or effect must hold or apply at the end of a plan.
    End,
}

pub mod names {
    pub const START: &'static str = "start";
    pub const END: &'static str = "end";
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
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::START => Ok(Self::Start),
            names::END => Ok(Self::End),
            _ => Err(ParseError::InvalidSpecifier),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid specifier")]
    InvalidSpecifier,
}
