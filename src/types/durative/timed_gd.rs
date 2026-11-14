use crate::types::{GoalDescription, Interval, TimeSpecifier};

/// A timed goal description.
///
/// ## Usage
/// Used by [`PreferenceTimedGoalDescription`](crate::PreferenceTimedGoalDescription).
#[doc(alias("timed-GD"))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimedGoalDescription {
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
    At(TimeSpecifier, GoalDescription),
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
    Over(Interval, GoalDescription),
}

impl TimedGoalDescription {
    /// Creates an `(at <time-specifier> <GD>)` goal, e.g. `(at start <GD>)`.
    #[must_use]
    pub const fn at(time: TimeSpecifier, gd: GoalDescription) -> Self {
        Self::At(time, gd)
    }

    /// Deprecated alias for [`Self::at`].
    #[deprecated(since = "0.2.0", note = "Use `at` instead")]
    #[must_use]
    pub const fn new_at(time: TimeSpecifier, gd: GoalDescription) -> Self {
        Self::at(time, gd)
    }

    /// Creates an `(over <interval> <GD>)` goal, e.g. `(over all <GD>)`.
    #[must_use]
    pub const fn over(interval: Interval, gd: GoalDescription) -> Self {
        Self::Over(interval, gd)
    }

    /// Deprecated alias for [`Self::over`].
    #[deprecated(since = "0.2.0", note = "Use `over` instead")]
    #[must_use]
    pub const fn new_over(interval: Interval, gd: GoalDescription) -> Self {
        Self::over(interval, gd)
    }
}

impl From<(TimeSpecifier, GoalDescription)> for TimedGoalDescription {
    fn from(value: (TimeSpecifier, GoalDescription)) -> Self {
        TimedGoalDescription::At(value.0, value.1)
    }
}

impl From<(Interval, GoalDescription)> for TimedGoalDescription {
    fn from(value: (Interval, GoalDescription)) -> Self {
        TimedGoalDescription::Over(value.0, value.1)
    }
}

/// Alias for [`TimedGoalDescription`]; matches BNF `<timed-GD>`.
#[deprecated(since = "0.2.0", note = "Use `TimedGoalDescription` instead")]
pub type TimedGD = TimedGoalDescription;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GoalDescription;
    use assert2::check;

    #[test]
    fn at_start() {
        let gd = GoalDescription::and(Vec::new());
        let timed = TimedGoalDescription::at(TimeSpecifier::Start, gd.clone());
        check!(let TimedGoalDescription::At(TimeSpecifier::Start, _) = timed);
    }

    #[test]
    fn at_end() {
        let gd = GoalDescription::and(Vec::new());
        let timed = TimedGoalDescription::at(TimeSpecifier::End, gd.clone());
        check!(let TimedGoalDescription::At(TimeSpecifier::End, _) = timed);
    }

    #[test]
    fn over_all() {
        let gd = GoalDescription::and(Vec::new());
        let timed = TimedGoalDescription::over(Interval::All, gd.clone());
        check!(let TimedGoalDescription::Over(Interval::All, _) = timed);
    }

    #[test]
    fn from_over_tuple() {
        let gd = GoalDescription::and(Vec::new());
        let timed = TimedGoalDescription::from((Interval::All, gd));
        check!(let TimedGoalDescription::Over(Interval::All, _) = timed);
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_over() {
        let gd = GoalDescription::and(Vec::new());
        let timed = TimedGoalDescription::new_over(Interval::All, gd);
        check!(let TimedGoalDescription::Over(Interval::All, _) = timed);
    }

    #[test]
    fn from_at_tuple() {
        let gd = GoalDescription::and(Vec::new());
        let timed = TimedGoalDescription::from((TimeSpecifier::Start, gd));
        check!(let TimedGoalDescription::At(TimeSpecifier::Start, _) = timed);
    }

    #[test]
    fn clone_works() {
        let gd = GoalDescription::and(Vec::new());
        let timed = TimedGoalDescription::at(TimeSpecifier::Start, gd);
        let clone = timed.clone();
        check!(timed == clone);
    }

    #[test]
    fn debug_impl() {
        let gd = GoalDescription::and(Vec::new());
        let timed = TimedGoalDescription::at(TimeSpecifier::Start, gd);
        let dbg = format!("{timed:?}");
        check!(dbg.contains("At"));
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_alias_exists() {
        fn _assert_alias(_: TimedGD) {}
    }
}
