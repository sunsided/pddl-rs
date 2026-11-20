//! Contains the timed effect assignment operation type [`TimedAssignOperator`].

use crate::types::keyword_error::UnknownKeyword;
use std::fmt::{Display, Formatter};

/// An assignment operation.
///
/// ## Usage
/// Used by [`TimedEffect`](crate::TimedEffect).
#[doc(alias("assign-op-t"))]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TimedAssignOperator {
    /// The `increase` operator, e.g. `(increase (fuel) (* #t (rate)))`.
    Increase,
    /// The `decrease` operator, e.g. `(decrease (fuel) (* #t (rate)))`.
    Decrease,
}

/// The PDDL keywords for [`TimedAssignOperator`].
pub mod names {
    /// The `increase` operator.
    pub const INCREASE: &str = "increase";
    /// The `decrease` operator.
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
    type Error = UnknownKeyword;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::INCREASE => Ok(Self::Increase),
            names::DECREASE => Ok(Self::Decrease),
            _ => Err(UnknownKeyword::new("timed assignment operator", value)),
        }
    }
}

/// Alias for [`TimedAssignOperator`]; matches BNF `<assign-op-t>`.
#[deprecated(since = "0.2.0", note = "Use `TimedAssignOperator` instead")]
pub type AssignOpT = TimedAssignOperator;
