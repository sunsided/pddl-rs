use crate::types::{GoalDefinition, Interval, TimeSpecifier};

/// A timed goal definition.
#[derive(Debug, Clone, PartialEq)]
pub enum TimedGD<'a> {
    At(TimeSpecifier, GoalDefinition<'a>),
    Over(Interval, GoalDefinition<'a>),
}

impl<'a> TimedGD<'a> {
    pub const fn new_at(time: TimeSpecifier, gd: GoalDefinition<'a>) -> Self {
        Self::At(time, gd)
    }

    pub const fn new_over(interval: Interval, gd: GoalDefinition<'a>) -> Self {
        Self::Over(interval, gd)
    }
}

impl<'a> From<(TimeSpecifier, GoalDefinition<'a>)> for TimedGD<'a> {
    fn from(value: (TimeSpecifier, GoalDefinition<'a>)) -> Self {
        TimedGD::At(value.0, value.1)
    }
}

impl<'a> From<(Interval, GoalDefinition<'a>)> for TimedGD<'a> {
    fn from(value: (Interval, GoalDefinition<'a>)) -> Self {
        TimedGD::Over(value.0, value.1)
    }
}
