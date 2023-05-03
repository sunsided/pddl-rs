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
/// [NumericFluents](crate::Requirement::NumericFluents) for the default `number` type, or
/// [ObjectFluents](crate::Requirement::ObjectFluents) and [Typing](crate::Requirement::Typing)
/// for an arbitrary type.
///
/// ## Usage
/// Used by [`Functions`](crate::Functions).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionTypedList<'a, T>(Vec<FunctionTyped<'a, T>>);

impl<'a, T> Default for FunctionTypedList<'a, T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl<'a, T> FunctionTypedList<'a, T> {
    pub const fn new(list: Vec<FunctionTyped<'a, T>>) -> Self {
        Self(list)
    }

    /// Gets the values.
    pub fn values(&self) -> &[FunctionTyped<'a, T>] {
        self.0.as_slice()
    }
}

impl<'a, T> From<Vec<FunctionTyped<'a, T>>> for FunctionTypedList<'a, T> {
    fn from(iter: Vec<FunctionTyped<'a, T>>) -> Self {
        FunctionTypedList::new(iter)
    }
}

impl<'a, T> FromIterator<FunctionTyped<'a, T>> for FunctionTypedList<'a, T> {
    fn from_iter<I: IntoIterator<Item = FunctionTyped<'a, T>>>(iter: I) -> Self {
        FunctionTypedList::new(iter.into_iter().collect())
    }
}

impl<'a, T> Deref for FunctionTypedList<'a, T> {
    type Target = [FunctionTyped<'a, T>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a, T> PartialEq<Vec<FunctionTyped<'_, T>>> for FunctionTypedList<'a, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Vec<FunctionTyped<'_, T>>) -> bool {
        self.0.eq(other)
    }
}

impl<'a, T> PartialEq<[FunctionTyped<'_, T>]> for FunctionTypedList<'a, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &[FunctionTyped<'_, T>]) -> bool {
        self.0.eq(other)
    }
}
