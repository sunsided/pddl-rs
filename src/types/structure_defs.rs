//! Provides structure definitions.

use crate::types::StructureDef;
use std::ops::Deref;

/// A set of structure definitions.
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct StructureDefs(Vec<StructureDef>);

impl StructureDefs {
    pub fn new<I: IntoIterator<Item = StructureDef>>(defs: I) -> Self {
        Self(defs.into_iter().collect())
    }

    /// Gets the values.
    pub fn values(&self) -> &[StructureDef] {
        self.0.as_slice()
    }
}

impl Deref for StructureDefs {
    type Target = [StructureDef];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<StructureDef>> for StructureDefs {
    fn from(value: Vec<StructureDef>) -> Self {
        StructureDefs::new(value)
    }
}

impl FromIterator<StructureDef> for StructureDefs {
    fn from_iter<T: IntoIterator<Item = StructureDef>>(iter: T) -> Self {
        StructureDefs::new(iter)
    }
}
