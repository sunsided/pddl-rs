//! Contains preferences.

use crate::types::{PreferenceName, GD};

/// A preference.
#[derive(Debug, Clone, PartialEq)]
pub struct Preference<'a>(Option<PreferenceName<'a>>, GD<'a>);

impl<'a> Preference<'a> {
    pub const fn new(name: Option<PreferenceName<'a>>, gd: GD<'a>) -> Self {
        Self(name, gd)
    }
}

impl<'a> From<(Option<PreferenceName<'a>>, GD<'a>)> for Preference<'a> {
    fn from(value: (Option<PreferenceName<'a>>, GD<'a>)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<'a> From<(PreferenceName<'a>, GD<'a>)> for Preference<'a> {
    fn from(value: (PreferenceName<'a>, GD<'a>)) -> Self {
        Self::new(Some(value.0), value.1)
    }
}

impl<'a> From<GD<'a>> for Preference<'a> {
    fn from(value: GD<'a>) -> Self {
        Self::new(None, value)
    }
}
