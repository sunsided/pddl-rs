//! Contains typed function elements via the [`FunctionTypedList`] type.

use crate::types::function_typed::FunctionTyped;
use std::ops::Deref;

/// A list of typed elements.
///
/// ## Example
/// ```
/// # use pddl::{FunctionTypedList, FunctionTyped, FunctionType};
/// # use pddl::{TypedList, Name};
/// let tl = FunctionTypedList::from_iter([
///     FunctionTyped::new(Name::from("location"), FunctionType::NUMBER),
///     FunctionTyped::new(Name::from("physob"), FunctionType::NUMBER),
/// ]);
///
/// assert_eq!(tl.len(), 2);
/// assert_eq!(tl[0].value_ref(), &Name::from("location"));
/// assert_eq!(tl[1].value_ref(), &Name::from("physob"));
/// ```
///
/// ## Requirements
/// Requires [Fluents](crate::Requirement::Fluents) and either
/// [Numeric Fluents](crate::Requirement::NumericFluents) for the default `number` type, or
/// [Object Fluents](crate::Requirement::ObjectFluents) and [Typing](crate::Requirement::Typing)
/// for an arbitrary type.
///
/// ## Usage
/// Used by [`Functions`](crate::Functions).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionTypedList<T>(Vec<FunctionTyped<T>>);

impl<'a, T> Default for FunctionTypedList<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl<'a, T> FunctionTypedList<T> {
    pub const fn new(list: Vec<FunctionTyped<T>>) -> Self {
        Self(list)
    }

    /// Gets the values.
    pub fn values(&self) -> &[FunctionTyped<T>] {
        self.0.as_slice()
    }
}

impl<'a, T> From<Vec<FunctionTyped<T>>> for FunctionTypedList<T> {
    fn from(iter: Vec<FunctionTyped<T>>) -> Self {
        FunctionTypedList::new(iter)
    }
}

impl<'a, T> FromIterator<FunctionTyped<T>> for FunctionTypedList<T> {
    fn from_iter<I: IntoIterator<Item = FunctionTyped<T>>>(iter: I) -> Self {
        FunctionTypedList::new(iter.into_iter().collect())
    }
}

impl<'a, T> Deref for FunctionTypedList<T> {
    type Target = [FunctionTyped<T>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a, T> PartialEq<Vec<FunctionTyped<T>>> for FunctionTypedList<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Vec<FunctionTyped<T>>) -> bool {
        self.0.eq(other)
    }
}

impl<'a, T> PartialEq<[FunctionTyped<T>]> for FunctionTypedList<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &[FunctionTyped<T>]) -> bool {
        self.0.eq(other)
    }
}
