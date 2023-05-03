//! Provides structure definitions.

use crate::types::StructureDef;
use std::ops::Deref;

/// A set of structure definitions.
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StructureDefs<'a>(Vec<StructureDef<'a>>);

impl<'a> StructureDefs<'a> {
    pub fn new<I: IntoIterator<Item = StructureDef<'a>>>(defs: I) -> Self {
        Self(defs.into_iter().collect())
    }

    /// Gets the values.
    pub fn values(&self) -> &[StructureDef<'a>] {
        self.0.as_slice()
    }
}

impl<'a> Deref for StructureDefs<'a> {
    type Target = [StructureDef<'a>];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<Vec<StructureDef<'a>>> for StructureDefs<'a> {
    fn from(value: Vec<StructureDef<'a>>) -> Self {
        StructureDefs::new(value)
    }
}

impl<'a> FromIterator<StructureDef<'a>> for StructureDefs<'a> {
    fn from_iter<T: IntoIterator<Item = StructureDef<'a>>>(iter: T) -> Self {
        StructureDefs::new(iter)
    }
}
