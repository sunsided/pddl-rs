//! Contains preference goal definitions.

use crate::types::{Preference, GD};

/// A preference goal definition.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PrefGD<'a> {
    GoalDefinition(GD<'a>),
    Preference(Preference<'a>),
}

impl<'a> PrefGD<'a> {
    pub const fn from_gd(gd: GD<'a>) -> Self {
        Self::GoalDefinition(gd)
    }

    pub fn from_preference(pref: Preference<'a>) -> Self {
        Self::Preference(pref)
    }
}

impl<'a> From<GD<'a>> for PrefGD<'a> {
    fn from(value: GD<'a>) -> Self {
        PrefGD::from_gd(value)
    }
}

impl<'a> From<Preference<'a>> for PrefGD<'a> {
    fn from(value: Preference<'a>) -> Self {
        PrefGD::from_preference(value)
    }
}
