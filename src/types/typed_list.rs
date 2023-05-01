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

impl<'a, T> Display for TypedList<'a, T>
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
            Variable::from("x").to_typed(Type::OBJECT),
            Variable::from("y").to_typed(Type::OBJECT),
            Variable::from("z").to_typed(Type::new_exactly("letter")),
        ]);
        assert_eq!(format!("{name}"), "?x ?y - object ?z - letter");
    }
}
