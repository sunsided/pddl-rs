use crate::types::{GoalDefinition, Number};

#[derive(Debug, Clone, PartialEq)]
pub enum ConGD<'a> {
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

/// A type that represents either a [`GoalDefinition`] or an embedded [`ConGD`].
#[derive(Debug, Clone, PartialEq)]
pub enum Con2GD<'a> {
    Goal(GoalDefinition<'a>),
    Nested(Box<ConGD<'a>>),
}

impl<'a> ConGD<'a> {
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

    pub const fn new_after(number: Number, gd: Con2GD<'a>) -> Self {
        Self::HoldAfter(number, gd)
    }
}

impl<'a> Con2GD<'a> {
    pub fn new_con_gd(gd: ConGD<'a>) -> Self {
        Self::Nested(Box::new(gd))
    }

    pub const fn new_gd(gd: GoalDefinition<'a>) -> Self {
        Self::Goal(gd)
    }
}

impl<'a> From<ConGD<'a>> for Con2GD<'a> {
    fn from(value: ConGD<'a>) -> Self {
        Con2GD::new_con_gd(value)
    }
}

impl<'a> From<GoalDefinition<'a>> for Con2GD<'a> {
    fn from(value: GoalDefinition<'a>) -> Self {
        Con2GD::new_gd(value)
    }
}
