use crate::types::{PreferenceName, TimedGD};

/// A (preferred) timed goal definition.
#[derive(Debug, Clone, PartialEq)]
pub enum PrefTimedGD<'a> {
    Required(TimedGD<'a>),
    Preference(Option<PreferenceName<'a>>, TimedGD<'a>),
}

impl<'a> PrefTimedGD<'a> {
    pub const fn new_required(gd: TimedGD<'a>) -> Self {
        Self::Required(gd)
    }

    pub const fn new_preference(name: Option<PreferenceName<'a>>, gd: TimedGD<'a>) -> Self {
        Self::Preference(name, gd)
    }
}

impl<'a> From<TimedGD<'a>> for PrefTimedGD<'a> {
    fn from(value: TimedGD<'a>) -> Self {
        PrefTimedGD::Required(value)
    }
}

impl<'a> From<(Option<PreferenceName<'a>>, TimedGD<'a>)> for PrefTimedGD<'a> {
    fn from(value: (Option<PreferenceName<'a>>, TimedGD<'a>)) -> Self {
        PrefTimedGD::Preference(value.0, value.1)
    }
}
