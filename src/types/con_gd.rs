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
    pub fn new_and<G: IntoIterator<Item = ConstraintGoalDefinition>>(goals: G) -> Self {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        Self::And(goals.into_iter().collect())
    }

    pub fn new_forall(variables: TypedVariables, gd: ConstraintGoalDefinition) -> Self {
        Self::Forall(variables, Box::new(gd))
    }

    pub const fn new_at_end(gd: GoalDefinition) -> Self {
        Self::AtEnd(gd)
    }

    pub const fn new_always(gd: ConstraintGoalDefinitionInner) -> Self {
        Self::Always(gd)
    }

    pub const fn new_sometime(gd: ConstraintGoalDefinitionInner) -> Self {
        Self::Sometime(gd)
    }

    pub const fn new_within(number: Number, gd: ConstraintGoalDefinitionInner) -> Self {
        Self::Within(number, gd)
    }

    pub const fn new_at_most_once(gd: ConstraintGoalDefinitionInner) -> Self {
        Self::AtMostOnce(gd)
    }

    pub const fn new_sometime_after(
        first: ConstraintGoalDefinitionInner,
        then: ConstraintGoalDefinitionInner,
    ) -> Self {
        Self::SometimeAfter(first, then)
    }

    pub const fn new_sometime_before(
        later: ConstraintGoalDefinitionInner,
        earlier: ConstraintGoalDefinitionInner,
    ) -> Self {
        Self::SometimeBefore(later, earlier)
    }

    pub const fn new_always_within(
        number: Number,
        first: ConstraintGoalDefinitionInner,
        second: ConstraintGoalDefinitionInner,
    ) -> Self {
        Self::AlwaysWithin(number, first, second)
    }

    pub const fn new_hold_during(
        begin: Number,
        end: Number,
        gd: ConstraintGoalDefinitionInner,
    ) -> Self {
        Self::HoldDuring(begin, end, gd)
    }

    pub const fn new_hold_after(number: Number, gd: ConstraintGoalDefinitionInner) -> Self {
        Self::HoldAfter(number, gd)
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
    pub fn new_nested(gd: ConstraintGoalDefinition) -> Self {
        Self::Nested(Box::new(gd))
    }

    pub const fn new_goal(gd: GoalDefinition) -> Self {
        Self::Goal(gd)
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
        ConstraintGoalDefinitionInner::new_nested(value)
    }
}

impl From<GoalDefinition> for ConstraintGoalDefinitionInner {
    fn from(value: GoalDefinition) -> Self {
        ConstraintGoalDefinitionInner::new_goal(value)
    }
}
