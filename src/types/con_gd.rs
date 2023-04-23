use crate::types::{GoalDefinition, Number, TypedList, Variable};

#[derive(Debug, Clone, PartialEq)]
pub enum ConGD<'a> {
    And(Vec<ConGD<'a>>),
    Forall(TypedList<'a, Variable<'a>>, Box<ConGD<'a>>),
    AtEnd(GoalDefinition<'a>),
    Always(Con2GD<'a>),
    Sometime(Con2GD<'a>),
    Within(Number, Con2GD<'a>),
    AtMostOnce(Con2GD<'a>),
    SometimeAfter(Con2GD<'a>, Con2GD<'a>),
    SometimeBefore(Con2GD<'a>, Con2GD<'a>),
    AlwaysWithin(Number, Con2GD<'a>, Con2GD<'a>),
    HoldDuring(Number, Number, Con2GD<'a>),
    HoldAfter(Number, Con2GD<'a>),
}

impl<'a> Default for ConGD<'a> {
    fn default() -> Self {
        Self::And(Vec::default())
    }
}

/// A type that represents either a [`GoalDefinition`] or an embedded [`ConGD`].
#[derive(Debug, Clone, PartialEq)]
pub enum Con2GD<'a> {
    Goal(GoalDefinition<'a>),
    Nested(Box<ConGD<'a>>),
}

impl<'a> ConGD<'a> {
    pub fn new_and<G: IntoIterator<Item = ConGD<'a>>>(goals: G) -> Self {
        Self::And(goals.into_iter().collect())
    }

    pub fn new_for_all(variables: TypedList<'a, Variable<'a>>, gd: ConGD<'a>) -> Self {
        Self::Forall(variables, Box::new(gd))
    }

    pub const fn new_at_end(gd: GoalDefinition<'a>) -> Self {
        Self::AtEnd(gd)
    }

    pub const fn new_always(gd: Con2GD<'a>) -> Self {
        Self::Always(gd)
    }

    pub const fn new_sometime(gd: Con2GD<'a>) -> Self {
        Self::Sometime(gd)
    }

    pub const fn new_within(number: Number, gd: Con2GD<'a>) -> Self {
        Self::Within(number, gd)
    }

    pub const fn new_at_most_once(gd: Con2GD<'a>) -> Self {
        Self::AtMostOnce(gd)
    }

    pub const fn new_sometime_after(first: Con2GD<'a>, then: Con2GD<'a>) -> Self {
        Self::SometimeAfter(first, then)
    }

    pub const fn new_sometime_before(later: Con2GD<'a>, earlier: Con2GD<'a>) -> Self {
        Self::SometimeBefore(later, earlier)
    }

    pub const fn new_always_within(number: Number, first: Con2GD<'a>, second: Con2GD<'a>) -> Self {
        Self::AlwaysWithin(number, first, second)
    }

    pub const fn new_hold_during(begin: Number, end: Number, gd: Con2GD<'a>) -> Self {
        Self::HoldDuring(begin, end, gd)
    }

    pub const fn new_hold_after(number: Number, gd: Con2GD<'a>) -> Self {
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

impl<'a> Con2GD<'a> {
    pub fn new_nested(gd: ConGD<'a>) -> Self {
        Self::Nested(Box::new(gd))
    }

    pub const fn new_goal(gd: GoalDefinition<'a>) -> Self {
        Self::Goal(gd)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Goal(gd) => gd.is_empty(),
            Self::Nested(gd) => gd.is_empty(),
        }
    }
}

impl<'a> From<ConGD<'a>> for Con2GD<'a> {
    fn from(value: ConGD<'a>) -> Self {
        Con2GD::new_nested(value)
    }
}

impl<'a> From<GoalDefinition<'a>> for Con2GD<'a> {
    fn from(value: GoalDefinition<'a>) -> Self {
        Con2GD::new_goal(value)
    }
}
