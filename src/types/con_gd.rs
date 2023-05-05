//! Contains the conditional problem/goal definition types [`ConGD`] and [`Con2GD`].

use crate::types::{GoalDefinition, Number, TypedVariables};

/// ## Usage
/// Used by [`ConGD`](ConGD) itself, as well as [`PrefConGD`](crate::types::PrefConGD) and [`Con2GD`](Con2GD).
#[derive(Debug, Clone, PartialEq)]
pub enum ConGD {
    And(Vec<ConGD>),
    Forall(TypedVariables, Box<ConGD>),
    AtEnd(GoalDefinition),
    Always(Con2GD),
    Sometime(Con2GD),
    Within(Number, Con2GD),
    AtMostOnce(Con2GD),
    SometimeAfter(Con2GD, Con2GD),
    SometimeBefore(Con2GD, Con2GD),
    AlwaysWithin(Number, Con2GD, Con2GD),
    HoldDuring(Number, Number, Con2GD),
    HoldAfter(Number, Con2GD),
}

impl Default for ConGD {
    fn default() -> Self {
        Self::And(Vec::default())
    }
}

/// A type that represents either a [`GoalDefinition`] or an embedded [`ConGD`].
///
/// ## Usage
/// Used by [`ConGD`](ConGD).
#[derive(Debug, Clone, PartialEq)]
pub enum Con2GD {
    Goal(GoalDefinition),
    Nested(Box<ConGD>),
}

impl ConGD {
    pub fn new_and<G: IntoIterator<Item = ConGD>>(goals: G) -> Self {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        Self::And(goals.into_iter().collect())
    }

    pub fn new_forall(variables: TypedVariables, gd: ConGD) -> Self {
        Self::Forall(variables, Box::new(gd))
    }

    pub const fn new_at_end(gd: GoalDefinition) -> Self {
        Self::AtEnd(gd)
    }

    pub const fn new_always(gd: Con2GD) -> Self {
        Self::Always(gd)
    }

    pub const fn new_sometime(gd: Con2GD) -> Self {
        Self::Sometime(gd)
    }

    pub const fn new_within(number: Number, gd: Con2GD) -> Self {
        Self::Within(number, gd)
    }

    pub const fn new_at_most_once(gd: Con2GD) -> Self {
        Self::AtMostOnce(gd)
    }

    pub const fn new_sometime_after(first: Con2GD, then: Con2GD) -> Self {
        Self::SometimeAfter(first, then)
    }

    pub const fn new_sometime_before(later: Con2GD, earlier: Con2GD) -> Self {
        Self::SometimeBefore(later, earlier)
    }

    pub const fn new_always_within(number: Number, first: Con2GD, second: Con2GD) -> Self {
        Self::AlwaysWithin(number, first, second)
    }

    pub const fn new_hold_during(begin: Number, end: Number, gd: Con2GD) -> Self {
        Self::HoldDuring(begin, end, gd)
    }

    pub const fn new_hold_after(number: Number, gd: Con2GD) -> Self {
        Self::HoldAfter(number, gd)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ConGD::And(x) => x.iter().all(|y| y.is_empty()),
            ConGD::Forall(_, x) => x.is_empty(),
            ConGD::AtEnd(x) => x.is_empty(),
            ConGD::Always(x) => x.is_empty(),
            ConGD::Sometime(x) => x.is_empty(),
            ConGD::Within(_, x) => x.is_empty(),
            ConGD::AtMostOnce(x) => x.is_empty(),
            ConGD::SometimeAfter(x, y) => x.is_empty() && y.is_empty(),
            ConGD::SometimeBefore(x, y) => x.is_empty() && y.is_empty(),
            ConGD::AlwaysWithin(_, x, y) => x.is_empty() && y.is_empty(),
            ConGD::HoldDuring(_, _, x) => x.is_empty(),
            ConGD::HoldAfter(_, x) => x.is_empty(),
        }
    }
}

impl Con2GD {
    pub fn new_nested(gd: ConGD) -> Self {
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

impl From<ConGD> for Con2GD {
    fn from(value: ConGD) -> Self {
        Con2GD::new_nested(value)
    }
}

impl From<GoalDefinition> for Con2GD {
    fn from(value: GoalDefinition) -> Self {
        Con2GD::new_goal(value)
    }
}
