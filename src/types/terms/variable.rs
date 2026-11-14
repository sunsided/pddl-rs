//! Contains variables.

use crate::types::{Name, PrimitiveType, ToTyped, Type, Typed};
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// A variable name.
///
/// ## Usage
/// Used by [`AtomicFormulaSkeleton`](crate::AtomicFormulaSkeleton),
/// [`AtomicFunctionSkeleton`](crate::AtomicFunctionSkeleton), [`ActionDefinition`](crate::ActionDefinition),
/// [`PreconditionGoalDescription`](crate::PreconditionGoalDescription), [`GoalDescription`](crate::GoalDescription),
/// [`Term`](crate::Term), [`CEffect`](crate::CEffect), [`DurativeActionDefinition`](crate::DurativeActionDefinition),
/// [`DurativeActionGoalDescription`](crate::DurativeActionGoalDescription), [`DurativeActionEffect`](crate::DurativeActionEffect),
/// [`PrefConGD`](crate::PrefConGD) and [`ConGD`](crate::ConGD).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Variable(Name);

impl Variable {
    /// Creates a variable from a [`Name`] (without the leading `?`).
    #[inline]
    #[must_use]
    pub const fn new(name: Name) -> Self {
        Self(name)
    }

    /// Creates a variable from a string slice (without the leading `?`).
    #[inline]
    #[doc(alias = "new_string")]
    #[must_use]
    pub fn string(name: &str) -> Self {
        Self(Name::new(name))
    }

    /// Alias for [`Self::string`].
    #[inline]
    #[must_use]
    pub fn new_string(name: &str) -> Self {
        Self::string(name)
    }

    /// Creates a variable from a `'static` string; usable in `const` contexts.
    #[inline]
    #[must_use]
    pub const fn from_static(name: &'static str) -> Self {
        Self(Name::new_static(name))
    }

    /// Creates a variable from a [`Name`]; same as [`Self::new`].
    #[inline]
    #[must_use]
    pub const fn from_name(name: Name) -> Self {
        Self(name)
    }
}

impl ToTyped<Variable> for Variable {
    fn to_typed<I: Into<Type>>(self, r#type: I) -> Typed<Variable> {
        Typed::new(self, r#type.into())
    }
    fn to_typed_either<I: IntoIterator<Item = P>, P: Into<PrimitiveType>>(
        self,
        types: I,
    ) -> Typed<Variable> {
        Typed::new(self, Type::from_iter(types))
    }
}

impl<T> From<T> for Variable
where
    T: Into<Name>,
{
    #[inline]
    fn from(value: T) -> Self {
        Variable::new(value.into())
    }
}

impl AsRef<Name> for Variable {
    #[inline]
    fn as_ref(&self) -> &Name {
        &self.0
    }
}

impl AsRef<str> for Variable {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for Variable {
    type Target = Name;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn it_works() {
        let var = Variable::string("x");
        check!(format!("{var}") == "?x");
    }
}
