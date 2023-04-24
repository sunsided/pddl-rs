//! Provide assignment operations via the [`AssignOp`] type.

use std::borrow::Borrow;
use std::fmt::{Display, Formatter};

/// An assignment operation.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AssignOp {
    /// An assign effect assigns the value of a numeric variable to the given amount.
    /// ```pddl
    /// (assign (battery-level ?r) 10)
    /// ```
    ///
    /// It is possible to use another numeric variable as the assign value, for example:
    /// ```pddl
    /// (assign (battery-level ?r) (max-charge ?r))
    /// ```
    Assign,
    /// A scale up effect increases the value of the numeric variable by the given scale factor.
    /// ```pddl
    /// (scale-up (battery-level ?r) 2)
    /// ```
    ///
    /// The scale factor can be another numeric variable.
    /// ```pddl
    /// (scale-up (battery-level ?r) (charge-rate ?r))
    /// ```
    ScaleUp,
    /// A scale down effect decreases the value of the numeric variable by the given scale factor.
    /// ```pddl
    /// (scale-down (battery-level ?r) 2)
    /// ```
    ///
    /// The scale factor can be another numeric variable.
    /// ```pddl
    /// (scale-down (battery-level ?r) (consumption-rate ?r))
    /// ```
    ScaleDown,
    /// An increase effect increases the value of a numeric variable by the given amount.
    /// ```pddl
    /// (increase (battery-level ?r) 10)
    /// ```
    ///
    /// It is possible to use another numeric variable as the increase value for example.
    /// ```pddl
    /// (increase (battery-level ?r) (charge-available - ?solarpanel))
    /// ```
    Increase,
    /// A decrease effect decreases the value of a numeric variable by the given amount.
    /// ```pddl
    /// (decrease (battery-level ?r) 10)
    /// ```
    ///
    /// It is possible to use another numeric variable as the decrease value for example.
    /// ```pddl
    /// (decrease (battery-level ?r) (power-needed-for-work - ?task))
    /// ```
    Decrease,
}

pub mod names {
    pub const ASSIGN: &'static str = "assign";
    pub const SCALE_UP: &'static str = "scale-up";
    pub const SCALE_DOWN: &'static str = "scale-down";
    pub const INCREASE: &'static str = "increase";
    pub const DECREASE: &'static str = "decrease";
}

impl AssignOp {
    pub fn as_str(&self) -> &'static str {
        match self {
            AssignOp::Assign => names::ASSIGN,
            AssignOp::ScaleUp => names::SCALE_UP,
            AssignOp::ScaleDown => names::SCALE_DOWN,
            AssignOp::Increase => names::INCREASE,
            AssignOp::Decrease => names::DECREASE,
        }
    }
}

impl TryFrom<&str> for AssignOp {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            names::ASSIGN => Ok(AssignOp::Assign),
            names::SCALE_UP => Ok(AssignOp::ScaleUp),
            names::SCALE_DOWN => Ok(AssignOp::ScaleDown),
            names::INCREASE => Ok(AssignOp::Increase),
            names::DECREASE => Ok(AssignOp::Decrease),
            _ => Err(ParseError::InvalidOperation),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid operation")]
    InvalidOperation,
}

impl Borrow<str> for AssignOp {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for AssignOp {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for AssignOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl PartialEq<AssignOp> for &AssignOp {
    #[inline(always)]
    fn eq(&self, other: &AssignOp) -> bool {
        other.eq(*self)
    }
}

impl PartialEq<&AssignOp> for AssignOp {
    #[inline(always)]
    fn eq(&self, other: &&AssignOp) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<String> for AssignOp {
    #[inline(always)]
    fn eq(&self, other: &String) -> bool {
        other.eq(self.as_ref())
    }
}

impl PartialEq<&str> for AssignOp {
    #[inline(always)]
    fn eq(&self, other: &&str) -> bool {
        (*other).eq(self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_equals_works() {
        assert_eq!(AssignOp::Assign, "assign");
        assert_eq!(AssignOp::ScaleUp, "scale-up");
        assert_eq!(AssignOp::ScaleDown, "scale-down");
        assert_eq!(AssignOp::Increase, "increase");
        assert_eq!(AssignOp::Decrease, "decrease");
    }
}
