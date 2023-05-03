use crate::types::Typed;
use std::ops::Deref;

/// A list of typed elements.
///
/// ## Example
/// ```
/// # use pddl::{Name, TypedList, Typed, Type};
/// let tl = TypedList::from_iter([
///     Typed::new(Name::from("location"), Type::OBJECT),
///     Typed::new(Name::from("physob"), Type::OBJECT),
/// ]);
///
/// assert_eq!(tl.len(), 2);
/// assert_eq!(tl[0].value(), &Name::from("location"));
/// assert_eq!(tl[1].value(), &Name::from("physob"));
/// ```
///
/// ## Usage
/// Used by [`Types`](crate::Types) and [`Constants`](crate::Constants) in [`Domain`](crate::Domain),
/// [`AtomicFormulaSkeleton`](crate::AtomicFormulaSkeleton), [`AtomicFunctionSkeleton`](crate::AtomicFunctionSkeleton),
/// [`ActionDefinition`](crate::ActionDefinition), [`PreGD`](crate::PreconditionGoalDefinition),
/// [`GoalDefinition`](crate::GoalDefinition), [`CEffect`](crate::CEffect),
/// [`DurativeActionDefinition`](crate::DurativeActionDefinition), [`DurativeActionEffect`](crate::DurativeActionEffect),
/// [`Objects`](crate::Objects) in [`Problem`](crate::Problem), [`PrefConGD`](crate::PrefConGD) and
/// [`ConGD`](crate::ConGD).
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TypedList<'a, T>(Vec<Typed<'a, T>>);

impl<'a, T> TypedList<'a, T> {
    pub const fn new(list: Vec<Typed<'a, T>>) -> Self {
        Self(list)
    }

    /// Gets the values.
    pub fn value(&self) -> &[Typed<'a, T>] {
        self.0.as_slice()
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

impl<'a, T> IntoIterator for TypedList<'a, T> {
    type Item = Typed<'a, T>;
    type IntoIter = std::vec::IntoIter<Typed<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
