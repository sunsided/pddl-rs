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
pub struct Effects<'a>(Vec<CEffect<'a>>);

impl<'a> Effects<'a> {
    /// Constructs a new instance from the value.
    pub fn new(effect: CEffect<'a>) -> Self {
        Self(vec![effect])
    }

    /// Constructs a new instance from the provided vector of values.
    pub fn new_and(effects: Vec<CEffect<'a>>) -> Self {
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
    pub fn iter(&'a self) -> std::slice::Iter<'a, CEffect<'a>> {
        self.0.iter()
    }

    /// Get the only element of this list if the list has
    /// exactly one element. Returns [`None`] in all other cases.
    pub fn try_get_single(self) -> Option<CEffect<'a>> {
        if self.len() == 1 {
            self.into_iter().next()
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for Effects<'a> {
    type Item = CEffect<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> Deref for Effects<'a> {
    type Target = [CEffect<'a>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a> AsRef<[CEffect<'a>]> for Effects<'a> {
    fn as_ref(&self) -> &[CEffect<'a>] {
        self.0.as_slice()
    }
}

impl<'a> From<CEffect<'a>> for Effects<'a> {
    fn from(value: CEffect<'a>) -> Self {
        Effects::new(value)
    }
}

impl<'a> From<Vec<CEffect<'a>>> for Effects<'a> {
    fn from(value: Vec<CEffect<'a>>) -> Self {
        Effects::new_and(value)
    }
}

impl<'a> FromIterator<CEffect<'a>> for Effects<'a> {
    fn from_iter<T: IntoIterator<Item = CEffect<'a>>>(iter: T) -> Self {
        Effects::new_and(iter.into_iter().collect())
    }
}

impl<'a> TryInto<CEffect<'a>> for Effects<'a> {
    type Error = ();

    fn try_into(self) -> Result<CEffect<'a>, Self::Error> {
        self.try_get_single().ok_or(())
    }
}
