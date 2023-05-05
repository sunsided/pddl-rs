//! Contains variables.

use crate::types::{Name, PrimitiveType, ToTyped, Type, Typed};
use std::ops::Deref;

/// A variable name.
///
/// ## Usage
/// Used by [`AtomicFormulaSkeleton`](crate::AtomicFormulaSkeleton),
/// [`AtomicFunctionSkeleton`](crate::AtomicFunctionSkeleton), [`ActionDefinition`](crate::ActionDefinition),
/// [`PreconditionGoalDefinition`](crate::PreconditionGoalDefinition), [`GoalDefinition`](crate::GoalDefinition),
/// [`Term`](crate::Term), [`CEffect`](crate::CEffect), [`DurativeActionDefinition`](crate::DurativeActionDefinition),
/// [`DurativeActionGoalDefinition`](crate::DurativeActionGoalDefinition), [`DurativeActionEffect`](crate::DurativeActionEffect),
/// [`PrefConGD`](crate::PrefConGD) and [`ConGD`](crate::ConGD).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Variable(Name);

impl Variable {
    #[inline(always)]
    pub const fn new(name: Name) -> Self {
        Self(name)
    }

    #[inline(always)]
    pub fn from_str(name: &str) -> Self {
        Self(Name::new(name))
    }

    #[inline(always)]
    pub const fn from_static(name: &'static str) -> Self {
        Self(Name::new_static(name))
    }

    #[inline(always)]
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

impl<'a, T> From<T> for Variable
where
    T: Into<Name>,
{
    #[inline(always)]
    fn from(value: T) -> Self {
        Variable::new(value.into())
    }
}

impl AsRef<Name> for Variable {
    #[inline(always)]
    fn as_ref(&self) -> &Name {
        &self.0
    }
}

impl AsRef<str> for Variable {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for Variable {
    type Target = Name;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
