use crate::types::{GoalDefinition, Interval, TimeSpecifier};

/// A timed goal definition.
///
/// ## Usage
/// Used by [`PreferenceTimedGoalDefinition`](crate::PreferenceTimedGoalDefinition).
#[doc(alias("timed-GD"))]
#[derive(Debug, Clone, PartialEq)]
pub enum TimedGoalDefinition {
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
    At(TimeSpecifier, GoalDefinition),
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
    Over(Interval, GoalDefinition),
}

impl TimedGoalDefinition {
    pub const fn at(time: TimeSpecifier, gd: GoalDefinition) -> Self {
        Self::At(time, gd)
    }

    #[deprecated(since = "0.2.0", note = "Use `at` instead")]
    pub const fn new_at(time: TimeSpecifier, gd: GoalDefinition) -> Self {
        Self::at(time, gd)
    }

    pub const fn over(interval: Interval, gd: GoalDefinition) -> Self {
        Self::Over(interval, gd)
    }

    #[deprecated(since = "0.2.0", note = "Use `over` instead")]
    pub const fn new_over(interval: Interval, gd: GoalDefinition) -> Self {
        Self::over(interval, gd)
    }
}

impl From<(TimeSpecifier, GoalDefinition)> for TimedGoalDefinition {
    fn from(value: (TimeSpecifier, GoalDefinition)) -> Self {
        TimedGoalDefinition::At(value.0, value.1)
    }
}

impl From<(Interval, GoalDefinition)> for TimedGoalDefinition {
    fn from(value: (Interval, GoalDefinition)) -> Self {
        TimedGoalDefinition::Over(value.0, value.1)
    }
}

/// Alias for [`TimedGoalDefinition`]; matches BNF `<timed-GD>`.
#[deprecated(since = "0.2.0", note = "Use `TimedGoalDefinition` instead")]
pub type TimedGD = TimedGoalDefinition;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GoalDefinition;

    #[test]
    fn at_start() {
        let gd = GoalDefinition::and(Vec::new());
        let timed = TimedGoalDefinition::at(TimeSpecifier::Start, gd.clone());
        assert!(matches!(
            timed,
            TimedGoalDefinition::At(TimeSpecifier::Start, _)
        ));
    }

    #[test]
    fn at_end() {
        let gd = GoalDefinition::and(Vec::new());
        let timed = TimedGoalDefinition::at(TimeSpecifier::End, gd.clone());
        assert!(matches!(
            timed,
            TimedGoalDefinition::At(TimeSpecifier::End, _)
        ));
    }

    #[test]
    fn over_all() {
        let gd = GoalDefinition::and(Vec::new());
        let timed = TimedGoalDefinition::over(Interval::All, gd.clone());
        assert!(matches!(timed, TimedGoalDefinition::Over(Interval::All, _)));
    }

    #[test]
    fn from_over_tuple() {
        let gd = GoalDefinition::and(Vec::new());
        let timed = TimedGoalDefinition::from((Interval::All, gd));
        assert!(matches!(timed, TimedGoalDefinition::Over(Interval::All, _)));
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_over() {
        let gd = GoalDefinition::and(Vec::new());
        let timed = TimedGoalDefinition::new_over(Interval::All, gd);
        assert!(matches!(timed, TimedGoalDefinition::Over(Interval::All, _)));
    }

    #[test]
    fn from_at_tuple() {
        let gd = GoalDefinition::and(Vec::new());
        let timed = TimedGoalDefinition::from((TimeSpecifier::Start, gd));
        assert!(matches!(
            timed,
            TimedGoalDefinition::At(TimeSpecifier::Start, _)
        ));
    }

    #[test]
    fn clone_works() {
        let gd = GoalDefinition::and(Vec::new());
        let timed = TimedGoalDefinition::at(TimeSpecifier::Start, gd);
        let clone = timed.clone();
        assert_eq!(timed, clone);
    }

    #[test]
    fn debug_impl() {
        let gd = GoalDefinition::and(Vec::new());
        let timed = TimedGoalDefinition::at(TimeSpecifier::Start, gd);
        let dbg = format!("{timed:?}");
        assert!(dbg.contains("At"));
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_alias_exists() {
        fn _assert_alias(_: TimedGD) {}
    }
}
