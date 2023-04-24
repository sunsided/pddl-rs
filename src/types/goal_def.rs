//! Contains the [`GoalDef`] type.

use crate::types::PreGD;
use std::ops::Deref;

/// A problem goal definition; wraps a [`PreGD`].
#[derive(Debug, Clone, PartialEq)]
pub struct GoalDef<'a>(PreGD<'a>);

impl<'a> GoalDef<'a> {
    pub const fn new(gd: PreGD<'a>) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &PreGD<'a> {
        &self.0
    }
}

impl<'a> PartialEq<PreGD<'a>> for GoalDef<'a> {
    fn eq(&self, other: &PreGD<'a>) -> bool {
        self.0.eq(other)
    }
}

impl<'a> From<PreGD<'a>> for GoalDef<'a> {
    fn from(value: PreGD<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> Deref for GoalDef<'a> {
    type Target = PreGD<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Into<PreGD<'a>> for GoalDef<'a> {
    fn into(self) -> PreGD<'a> {
        self.0
    }
}
