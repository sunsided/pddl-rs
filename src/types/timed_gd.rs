use crate::types::{Interval, TimeSpecifier, GD};

/// A timed goal definition.
#[derive(Debug, Clone, PartialEq)]
pub enum TimedGD<'a> {
    At(TimeSpecifier, GD<'a>),
    Over(Interval, GD<'a>),
}

impl<'a> TimedGD<'a> {
    pub const fn new_at(time: TimeSpecifier, gd: GD<'a>) -> Self {
        Self::At(time, gd)
    }

    pub const fn new_over(interval: Interval, gd: GD<'a>) -> Self {
        Self::Over(interval, gd)
    }
}

impl<'a> From<(TimeSpecifier, GD<'a>)> for TimedGD<'a> {
    fn from(value: (TimeSpecifier, GD<'a>)) -> Self {
        TimedGD::At(value.0, value.1)
    }
}

impl<'a> From<(Interval, GD<'a>)> for TimedGD<'a> {
    fn from(value: (Interval, GD<'a>)) -> Self {
        TimedGD::Over(value.0, value.1)
    }
}
