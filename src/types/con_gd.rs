//! Contains the constraint goal definition types [`ConstraintGoalDefinition`] and [`ConstraintGoalDefinitionInner`].

use crate::types::{GoalDefinition, Number, TypedVariables};

/// A constraint goal definition.
///
/// Represents temporal constraint goals for problem-level constraints in PDDL 3.1.
/// Supports `always`, `sometime`, `within`, `at-most-once`, `sometime-after`,
/// `sometime-before`, `always-within`, `hold-during`, and `hold-after`.
///
/// # BNF
/// Corresponds to `<con-GD>` in the PDDL 3.1 specification.
///
/// ## Usage
/// Used by [`PreferenceConstraintGoalDefinition`](crate::types::PreferenceConstraintGoalDefinition) and [`ConstraintGoalDefinitionInner`].
#[doc(alias = "con-GD")]
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintGoalDefinition {
    And(Vec<ConstraintGoalDefinition>),
    Forall(TypedVariables, Box<ConstraintGoalDefinition>),
    AtEnd(GoalDefinition),
    Always(ConstraintGoalDefinitionInner),
    Sometime(ConstraintGoalDefinitionInner),
    Within(Number, ConstraintGoalDefinitionInner),
    AtMostOnce(ConstraintGoalDefinitionInner),
    SometimeAfter(ConstraintGoalDefinitionInner, ConstraintGoalDefinitionInner),
    SometimeBefore(ConstraintGoalDefinitionInner, ConstraintGoalDefinitionInner),
    AlwaysWithin(
        Number,
        ConstraintGoalDefinitionInner,
        ConstraintGoalDefinitionInner,
    ),
    HoldDuring(Number, Number, ConstraintGoalDefinitionInner),
    HoldAfter(Number, ConstraintGoalDefinitionInner),
}

/// Alias for [`ConstraintGoalDefinition`]; matches BNF `<con-GD>`.
#[deprecated(since = "0.2.0", note = "Use `ConstraintGoalDefinition` instead")]
pub type ConGD = ConstraintGoalDefinition;

/// A type that represents either a [`GoalDefinition`] or an embedded [`ConstraintGoalDefinition`].
///
/// # BNF
/// Corresponds to `<con2-GD>` in the PDDL 3.1 specification.
///
/// ## Usage
/// Used by [`ConstraintGoalDefinition`].
#[doc(alias = "con2-GD")]
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintGoalDefinitionInner {
    Goal(GoalDefinition),
    Nested(Box<ConstraintGoalDefinition>),
}

/// Alias for [`ConstraintGoalDefinitionInner`]; matches BNF `<con2-GD>`.
#[deprecated(since = "0.2.0", note = "Use `ConstraintGoalDefinitionInner` instead")]
pub type Con2GD = ConstraintGoalDefinitionInner;

impl Default for ConstraintGoalDefinition {
    fn default() -> Self {
        Self::And(Vec::default())
    }
}

impl ConstraintGoalDefinition {
    #[doc(alias = "new_and")]
    pub fn and<G: IntoIterator<Item = ConstraintGoalDefinition>>(goals: G) -> Self {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        Self::And(goals.into_iter().collect())
    }

    pub fn new_and<G: IntoIterator<Item = ConstraintGoalDefinition>>(goals: G) -> Self {
        Self::and(goals)
    }

    #[doc(alias = "new_forall")]
    pub fn r#forall(variables: TypedVariables, gd: ConstraintGoalDefinition) -> Self {
        Self::Forall(variables, Box::new(gd))
    }

    pub fn new_forall(variables: TypedVariables, gd: ConstraintGoalDefinition) -> Self {
        Self::r#forall(variables, gd)
    }

    #[doc(alias = "new_at_end")]
    pub const fn at_end(gd: GoalDefinition) -> Self {
        Self::AtEnd(gd)
    }

    pub fn new_at_end(gd: GoalDefinition) -> Self {
        Self::at_end(gd)
    }

    #[doc(alias = "new_always")]
    pub const fn always(gd: ConstraintGoalDefinitionInner) -> Self {
        Self::Always(gd)
    }

    pub fn new_always(gd: ConstraintGoalDefinitionInner) -> Self {
        Self::always(gd)
    }

    #[doc(alias = "new_sometime")]
    pub const fn sometime(gd: ConstraintGoalDefinitionInner) -> Self {
        Self::Sometime(gd)
    }

    pub fn new_sometime(gd: ConstraintGoalDefinitionInner) -> Self {
        Self::sometime(gd)
    }

    #[doc(alias = "new_within")]
    pub const fn within(number: Number, gd: ConstraintGoalDefinitionInner) -> Self {
        Self::Within(number, gd)
    }

    pub fn new_within(number: Number, gd: ConstraintGoalDefinitionInner) -> Self {
        Self::within(number, gd)
    }

    #[doc(alias = "new_at_most_once")]
    pub const fn at_most_once(gd: ConstraintGoalDefinitionInner) -> Self {
        Self::AtMostOnce(gd)
    }

    pub fn new_at_most_once(gd: ConstraintGoalDefinitionInner) -> Self {
        Self::at_most_once(gd)
    }

    #[doc(alias = "new_sometime_after")]
    pub const fn sometime_after(
        first: ConstraintGoalDefinitionInner,
        then: ConstraintGoalDefinitionInner,
    ) -> Self {
        Self::SometimeAfter(first, then)
    }

    pub fn new_sometime_after(
        first: ConstraintGoalDefinitionInner,
        then: ConstraintGoalDefinitionInner,
    ) -> Self {
        Self::sometime_after(first, then)
    }

    #[doc(alias = "new_sometime_before")]
    pub const fn sometime_before(
        later: ConstraintGoalDefinitionInner,
        earlier: ConstraintGoalDefinitionInner,
    ) -> Self {
        Self::SometimeBefore(later, earlier)
    }

    pub fn new_sometime_before(
        later: ConstraintGoalDefinitionInner,
        earlier: ConstraintGoalDefinitionInner,
    ) -> Self {
        Self::sometime_before(later, earlier)
    }

    #[doc(alias = "new_always_within")]
    pub const fn always_within(
        number: Number,
        first: ConstraintGoalDefinitionInner,
        second: ConstraintGoalDefinitionInner,
    ) -> Self {
        Self::AlwaysWithin(number, first, second)
    }

    pub fn new_always_within(
        number: Number,
        first: ConstraintGoalDefinitionInner,
        second: ConstraintGoalDefinitionInner,
    ) -> Self {
        Self::always_within(number, first, second)
    }

    #[doc(alias = "new_hold_during")]
    pub const fn hold_during(
        begin: Number,
        end: Number,
        gd: ConstraintGoalDefinitionInner,
    ) -> Self {
        Self::HoldDuring(begin, end, gd)
    }

    pub fn new_hold_during(begin: Number, end: Number, gd: ConstraintGoalDefinitionInner) -> Self {
        Self::hold_during(begin, end, gd)
    }

    #[doc(alias = "new_hold_after")]
    pub const fn hold_after(number: Number, gd: ConstraintGoalDefinitionInner) -> Self {
        Self::HoldAfter(number, gd)
    }

    pub fn new_hold_after(number: Number, gd: ConstraintGoalDefinitionInner) -> Self {
        Self::hold_after(number, gd)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ConstraintGoalDefinition::And(x) => x.iter().all(|y| y.is_empty()),
            ConstraintGoalDefinition::Forall(_, x) => x.is_empty(),
            ConstraintGoalDefinition::AtEnd(x) => x.is_empty(),
            ConstraintGoalDefinition::Always(x) => x.is_empty(),
            ConstraintGoalDefinition::Sometime(x) => x.is_empty(),
            ConstraintGoalDefinition::Within(_, x) => x.is_empty(),
            ConstraintGoalDefinition::AtMostOnce(x) => x.is_empty(),
            ConstraintGoalDefinition::SometimeAfter(x, y) => x.is_empty() && y.is_empty(),
            ConstraintGoalDefinition::SometimeBefore(x, y) => x.is_empty() && y.is_empty(),
            ConstraintGoalDefinition::AlwaysWithin(_, x, y) => x.is_empty() && y.is_empty(),
            ConstraintGoalDefinition::HoldDuring(_, _, x) => x.is_empty(),
            ConstraintGoalDefinition::HoldAfter(_, x) => x.is_empty(),
        }
    }
}

impl ConstraintGoalDefinitionInner {
    #[doc(alias = "new_nested")]
    pub fn nested(gd: ConstraintGoalDefinition) -> Self {
        Self::Nested(Box::new(gd))
    }

    pub fn new_nested(gd: ConstraintGoalDefinition) -> Self {
        Self::nested(gd)
    }

    #[doc(alias = "new_goal")]
    pub const fn goal(gd: GoalDefinition) -> Self {
        Self::Goal(gd)
    }

    pub fn new_goal(gd: GoalDefinition) -> Self {
        Self::goal(gd)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Goal(gd) => gd.is_empty(),
            Self::Nested(gd) => gd.is_empty(),
        }
    }
}

impl From<ConstraintGoalDefinition> for ConstraintGoalDefinitionInner {
    fn from(value: ConstraintGoalDefinition) -> Self {
        ConstraintGoalDefinitionInner::nested(value)
    }
}

impl From<GoalDefinition> for ConstraintGoalDefinitionInner {
    fn from(value: GoalDefinition) -> Self {
        ConstraintGoalDefinitionInner::goal(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraint_goal_definition_is_empty() {
        let empty = ConstraintGoalDefinition::default();
        assert!(empty.is_empty());

        let gd = GoalDefinition::and(Vec::new());
        let non_empty = ConstraintGoalDefinition::at_end(gd);
        assert!(non_empty.is_empty());
    }

    #[test]
    fn constraint_goal_definition_is_empty_nested() {
        let inner = ConstraintGoalDefinitionInner::goal(GoalDefinition::and(Vec::new()));
        let always = ConstraintGoalDefinition::always(inner);
        assert!(always.is_empty());
    }

    #[test]
    fn constraint_goal_definition_inner_is_empty() {
        let goal_inner = ConstraintGoalDefinitionInner::goal(GoalDefinition::and(Vec::new()));
        assert!(goal_inner.is_empty());

        let nested = ConstraintGoalDefinitionInner::nested(ConstraintGoalDefinition::default());
        assert!(nested.is_empty());
    }

    #[test]
    fn constraint_goal_definition_constructors() {
        let inner = ConstraintGoalDefinitionInner::goal(GoalDefinition::and(Vec::new()));

        let _ = ConstraintGoalDefinition::sometime(inner.clone());
        let _ = ConstraintGoalDefinition::at_most_once(inner.clone());
        let _ = ConstraintGoalDefinition::sometime_after(inner.clone(), inner.clone());
        let _ = ConstraintGoalDefinition::sometime_before(inner.clone(), inner.clone());
        let _ =
            ConstraintGoalDefinition::always_within(Number::from(5), inner.clone(), inner.clone());
        let _ =
            ConstraintGoalDefinition::hold_during(Number::from(0), Number::from(10), inner.clone());
        let _ = ConstraintGoalDefinition::hold_after(Number::from(5), inner);
    }

    #[test]
    fn constraint_goal_definition_inner_from() {
        let gd = GoalDefinition::and(Vec::new());
        let from_gd: ConstraintGoalDefinitionInner = gd.clone().into();
        assert!(matches!(from_gd, ConstraintGoalDefinitionInner::Goal(_)));

        let con_gd = ConstraintGoalDefinition::default();
        let from_con_gd: ConstraintGoalDefinitionInner = con_gd.into();
        assert!(matches!(
            from_con_gd,
            ConstraintGoalDefinitionInner::Nested(_)
        ));
    }
}
