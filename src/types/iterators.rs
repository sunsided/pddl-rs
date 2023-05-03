//! Contains iterator types.

use std::iter::once;
use std::vec;

/// An iterator that can represent instances with either
/// one or many values, such as [`ConditionalEffect`](crate::ConditionalEffect),
/// [`DurationConstraint`](crate::DurationConstraint),
/// [`Effect`](crate::Effect) and others.
pub enum FlatteningIntoIterator<T> {
    /// The iterator is already empty (similar to [`std::iter::Empty`]).
    Never,
    /// The iterator will produce exactly one value (similar to [`std::iter::Once`]).
    Once(std::iter::Once<T>),
    /// The iterator will produce none, one or many values.
    Vec(vec::IntoIter<T>),
}

impl<T> FlatteningIntoIterator<T> {
    /// Creates an iterator that will create exactly one item.
    pub fn new(item: T) -> Self {
        Self::Once(once(item))
    }

    /// Creates an iterator that will create none, one or many values.
    pub fn new_vec(vec: Vec<T>) -> Self {
        if vec.is_empty() {
            Self::Never
        } else {
            Self::Vec(vec.into_iter())
        }
    }
}

impl<T> Iterator for FlatteningIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Never => None,
            Self::Once(ref mut iter) => iter.next(),
            Self::Vec(ref mut iter) => iter.next(),
        }
    }
}
