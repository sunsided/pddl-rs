use crate::types::{Type, Typed};
use std::fmt::{Display, Formatter};
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
/// [`ActionDefinition`](crate::ActionDefinition), [`PreconditionGoalDefinition`](crate::PreconditionGoalDefinition),
/// [`GoalDefinition`](crate::GoalDefinition), [`CEffect`](crate::CEffect),
/// [`DurativeActionDefinition`](crate::DurativeActionDefinition), [`DurativeActionEffect`](crate::DurativeActionEffect),
/// [`Objects`](crate::Objects) in [`Problem`](crate::Problem), [`PrefConGD`](crate::PrefConGD) and
/// [`ConGD`](crate::ConGD).
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TypedList<T>(Vec<Typed<T>>);

impl<T> TypedList<T> {
    pub const fn new(list: Vec<Typed<T>>) -> Self {
        Self(list)
    }

    /// Gets the values.
    pub fn value(&self) -> &[Typed<T>] {
        self.0.as_slice()
    }
}

impl<T> From<Vec<Typed<T>>> for TypedList<T> {
    fn from(iter: Vec<Typed<T>>) -> Self {
        TypedList::new(iter)
    }
}

impl<T> FromIterator<Typed<T>> for TypedList<T> {
    fn from_iter<I: IntoIterator<Item = Typed<T>>>(iter: I) -> Self {
        TypedList::new(iter.into_iter().collect())
    }
}

impl<T> Deref for TypedList<T> {
    type Target = [Typed<T>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<T> PartialEq<Vec<Typed<T>>> for TypedList<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Vec<Typed<T>>) -> bool {
        self.0.eq(other)
    }
}

impl<T> PartialEq<[Typed<T>]> for TypedList<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &[Typed<T>]) -> bool {
        self.0.eq(other)
    }
}

impl<T> IntoIterator for TypedList<T> {
    type Item = Typed<T>;
    type IntoIter = std::vec::IntoIter<Typed<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Display for TypedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        let mut stack = Vec::with_capacity(self.len());
        let mut type_required = false;
        let mut last_type = &Type::OBJECT;

        for item in self.iter().rev() {
            // The `object` type is the default (for non-fluents) but can only
            // be omitted if it is the last type displayed.
            if !type_required && item.type_().eq(&Type::OBJECT) {
                stack.push(format!("{}", item.value()));
                continue;
            }

            // The moment a non-`object` type is noticed, we are required to list names.
            type_required = true;

            // If the type is identical to the one of the previous iteration,
            // it was already stated (since we are iterating backwards).
            // Therefore, we can skip it.
            if last_type.eq(item.type_()) {
                stack.push(format!("{}", item.value()))
            } else {
                stack.push(format!("{} - {}", item.value(), item.type_()));
                last_type = item.type_();
            }
        }

        stack = stack.into_iter().rev().collect();
        write!(f, "{}", stack.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Name, ToTyped, Type, Variable};

    #[test]
    fn object_is_default_works() {
        let name = TypedList::from_iter([Name::new("x").to_typed(Type::OBJECT)]);
        assert_eq!(format!("{name}"), "x");
    }

    #[test]
    fn multiple_object_works() {
        let name = TypedList::from_iter([
            Name::new("x").to_typed(Type::OBJECT),
            Name::new("y").to_typed(Type::OBJECT),
            Name::new("z").to_typed(Type::OBJECT),
        ]);
        assert_eq!(format!("{name}"), "x y z");
    }

    #[test]
    fn simple_works() {
        let name = TypedList::from_iter([Name::new("x").to_typed(Type::new_exactly("letter"))]);
        assert_eq!(format!("{name}"), "x - letter");
    }

    #[test]
    fn multiple_same_works() {
        let name = TypedList::from_iter([
            Name::new("x").to_typed(Type::new_exactly("letter")),
            Name::new("y").to_typed(Type::new_exactly("letter")),
            Name::new("z").to_typed(Type::new_exactly("letter")),
        ]);
        assert_eq!(format!("{name}"), "x y z - letter");
    }

    #[test]
    fn interleaved_at_end() {
        let name = TypedList::from_iter([
            Name::new("x").to_typed(Type::new_exactly("letter")),
            Name::new("y").to_typed(Type::new_exactly("letter")),
            Name::new("z").to_typed(Type::new_exactly("car")),
        ]);
        assert_eq!(format!("{name}"), "x y - letter z - car");
    }

    #[test]
    fn interleaved() {
        let name = TypedList::from_iter([
            Name::new("x").to_typed(Type::new_exactly("letter")),
            Name::new("y").to_typed(Type::new_exactly("car")),
            Name::new("z").to_typed(Type::new_exactly("letter")),
        ]);
        assert_eq!(format!("{name}"), "x - letter y - car z - letter");
    }

    #[test]
    fn object_at_end() {
        let name = TypedList::from_iter([
            Name::new("x").to_typed(Type::new_exactly("letter")),
            Name::new("y").to_typed(Type::OBJECT),
            Name::new("z").to_typed(Type::OBJECT),
        ]);
        assert_eq!(format!("{name}"), "x - letter y z");
    }

    #[test]
    fn object_at_start() {
        let name = TypedList::from_iter([
            Variable::from_str("x").to_typed(Type::OBJECT),
            Variable::from_str("y").to_typed(Type::OBJECT),
            Variable::from_str("z").to_typed(Type::new_exactly("letter")),
        ]);
        assert_eq!(format!("{name}"), "?x ?y - object ?z - letter");
    }
}
