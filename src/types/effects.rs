//! Contains effects via the [`Effects`] type.

use crate::types::CEffect;
use std::ops::Deref;

/// An effect. Occurs e.g. in a [`ActionDefinition`](crate::types::ActionDefinition).
///
/// This represents the `(and ...)` variant of the PDDL definition,
/// modeling cases of zero, one or many effects.
///
/// ## Usage
/// Used by [`ActionDefinition`](crate::ActionDefinition).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Effects(Vec<CEffect>);

impl Effects {
    /// Constructs a new instance from the value.
    pub fn new(effect: CEffect) -> Self {
        Self(vec![effect])
    }

    /// Constructs a new instance from the provided vector of values.
    pub fn new_and(effects: Vec<CEffect>) -> Self {
        Self(effects)
    }

    /// Returns `true` if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of elements in the list, also referred to
    /// as its 'length'.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the list.
    ///
    /// The iterator yields all items from start to end.
    pub fn iter(&self) -> std::slice::Iter<CEffect> {
        self.0.iter()
    }

    /// Get the only element of this list if the list has
    /// exactly one element. Returns [`None`] in all other cases.
    pub fn try_get_single(self) -> Option<CEffect> {
        if self.len() == 1 {
            self.into_iter().next()
        } else {
            None
        }
    }
}

impl IntoIterator for Effects {
    type Item = CEffect;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for Effects {
    type Target = [CEffect];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl AsRef<[CEffect]> for Effects {
    fn as_ref(&self) -> &[CEffect] {
        self.0.as_slice()
    }
}

impl From<CEffect> for Effects {
    fn from(value: CEffect) -> Self {
        Effects::new(value)
    }
}

impl From<Vec<CEffect>> for Effects {
    fn from(value: Vec<CEffect>) -> Self {
        Effects::new_and(value)
    }
}

impl FromIterator<CEffect> for Effects {
    fn from_iter<T: IntoIterator<Item = CEffect>>(iter: T) -> Self {
        Effects::new_and(iter.into_iter().collect())
    }
}

impl TryInto<CEffect> for Effects {
    type Error = ();

    fn try_into(self) -> Result<CEffect, Self::Error> {
        self.try_get_single().ok_or(())
    }
}
