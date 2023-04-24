use crate::types::{GoalDefinition, Interval, TimeSpecifier};

/// A timed goal definition.
#[derive(Debug, Clone, PartialEq)]
pub enum TimedGD<'a> {
    /// ## `at start`
    /// An expression or predicate with `at start` prefixed to it means that the condition
    /// must be true at the start of the action in order for the action to be applied. e.g.
    ///
    /// ```pddl
    /// (at start (at ?rover ?from-waypoint))
    /// ```
    ///
    /// expresses that `at start` the given rover is `at` the `from-waypoint`.
    /// Confusingly in this particular domain, the `at` is a predicate representing the
    /// location of an object `at` a point, whilst `at start` is a keyword.
    ///
    /// `at start` is usually applied per predicate.
    ///
    /// ## `at end`
    /// An expression or predicate with `at end` prefixed to it means that the condition
    /// must be true at the end of the action in order for the action to be applied e.g.
    ///
    /// ```pddl
    /// (at end (>= (battery-amount ?rover) 0))
    /// ```
    ///
    /// expresses that whilst this fact doesn't have to be true at the start or during the action,
    /// it must be true at the end. In this case, we're expressing that the battery amount at the
    /// end of the action must be greater than zero.
    At(TimeSpecifier, GoalDefinition<'a>),
    /// ## `over all`
    /// An expression or predicate with an overall prefixed to it, means that the condition
    /// must be true throughout the action, including at the start and end. e.g.
    ///
    /// ```pddl
    /// (over all (can-move ?from-waypoint ?to-waypoint))
    /// ```
    ///
    /// At all points in the execution of the action the given expression must evaluate to true.
    /// In the case above, we are expressing that it must be possible to move from the from
    /// waypoint to the to waypoint all the way through the action. I.e. we don't want to get
    /// half way through the action to find that after a certain point a path has become blocked.
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
