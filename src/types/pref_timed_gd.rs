//! Contains the [`PrefTimedGD`] type.

use crate::types::{PreferenceName, TimedGD};

/// A (preferred) timed goal definition.
///
/// ## Usage
/// Used by [`DurativeActionGoalDefinition`](crate::DurativeActionGoalDefinition).
#[derive(Debug, Clone, PartialEq)]
pub enum PrefTimedGD {
    Required(TimedGD),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Option<PreferenceName>, TimedGD),
}

impl PrefTimedGD {
    pub const fn new_required(gd: TimedGD) -> Self {
        Self::Required(gd)
    }

    pub const fn new_preference(name: Option<PreferenceName>, gd: TimedGD) -> Self {
        Self::Preference(name, gd)
    }
}

impl From<TimedGD> for PrefTimedGD {
    fn from(value: TimedGD) -> Self {
        PrefTimedGD::Required(value)
    }
}

impl From<(Option<PreferenceName>, TimedGD)> for PrefTimedGD {
    fn from(value: (Option<PreferenceName>, TimedGD)) -> Self {
        PrefTimedGD::Preference(value.0, value.1)
    }
}
