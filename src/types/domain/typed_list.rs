use crate::types::domain::Typed;
use std::ops::Deref;

/// A list of typed elements.
///
/// ## Example
/// ```
/// # use pddl::types::domain::{TypedList, Name, Typed, Type};
/// let tl = TypedList::from_iter([
///     Typed::new(Name::from("location"), Type::OBJECT),
///     Typed::new(Name::from("physob"), Type::OBJECT),
/// ]);
///
/// assert_eq!(tl.len(), 2);
/// assert_eq!(tl[0].value_ref(), &Name::from("location"));
/// assert_eq!(tl[1].value_ref(), &Name::from("physob"));
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TypedList<'a, T>(Vec<Typed<'a, T>>);

impl<'a, T> TypedList<'a, T> {
    pub const fn new(list: Vec<Typed<'a, T>>) -> Self {
        Self(list)
    }
}

impl<'a, T> From<Vec<Typed<'a, T>>> for TypedList<'a, T> {
    fn from(iter: Vec<Typed<'a, T>>) -> Self {
        TypedList::new(iter)
    }
}

impl<'a, T> FromIterator<Typed<'a, T>> for TypedList<'a, T> {
    fn from_iter<I: IntoIterator<Item = Typed<'a, T>>>(iter: I) -> Self {
        TypedList::new(iter.into_iter().collect())
    }
}

impl<'a, T> Deref for TypedList<'a, T> {
    type Target = [Typed<'a, T>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a, T> PartialEq<Vec<Typed<'_, T>>> for TypedList<'a, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Vec<Typed<'_, T>>) -> bool {
        self.0.eq(other)
    }
}

impl<'a, T> PartialEq<[Typed<'_, T>]> for TypedList<'a, T>
where
    T: PartialEq,
{
    fn eq(&self, other: &[Typed<'_, T>]) -> bool {
        self.0.eq(other)
    }
}
