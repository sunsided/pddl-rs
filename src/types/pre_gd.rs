//! Contains precondition goal definitions.

use crate::types::TypedVariables;
use crate::types::{Preference, PreferenceGD};
use std::ops::Deref;

/// Zero, one or many precondition goal definitions.
///
/// ## Usage
/// Used by [`GoalDef`](crate::GoalDef), as well as [`ActionDefinition`](crate::ActionDefinition).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreconditionGoalDefinitions<'a>(Vec<PreconditionGoalDefinition<'a>>);

impl<'a> PreconditionGoalDefinitions<'a> {
    /// Constructs a new instance from the provided vector of values.
    pub const fn new(values: Vec<PreconditionGoalDefinition<'a>>) -> Self {
        Self(values)
    }

    /// Constructs a list containing a single [`PreconditionGoalDefinition::Preference`] variant.
    pub fn new_preference(pref: PreferenceGD<'a>) -> Self {
        PreconditionGoalDefinition::new_preference(pref).into()
    }

    /// Constructs a list containing a single [`PreconditionGoalDefinition::Forall`] variant.
    pub fn new_forall(variables: TypedVariables<'a>, gd: PreconditionGoalDefinitions<'a>) -> Self {
        PreconditionGoalDefinition::new_forall(variables, gd).into()
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
    pub fn iter(&'a self) -> std::slice::Iter<'a, PreconditionGoalDefinition<'a>> {
        self.0.iter()
    }

    /// Get the only element of this list if the list has
    /// exactly one element. Returns [`None`] in all other cases.
    pub fn try_get_single(self) -> Option<PreconditionGoalDefinition<'a>> {
        if self.len() == 1 {
            self.into_iter().next()
        } else {
            None
        }
    }
}

impl<'a> FromIterator<PreconditionGoalDefinition<'a>> for PreconditionGoalDefinitions<'a> {
    fn from_iter<T: IntoIterator<Item = PreconditionGoalDefinition<'a>>>(iter: T) -> Self {
        PreconditionGoalDefinitions::new(iter.into_iter().collect())
    }
}

impl<'a> Deref for PreconditionGoalDefinitions<'a> {
    type Target = [PreconditionGoalDefinition<'a>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a> IntoIterator for PreconditionGoalDefinitions<'a> {
    type Item = PreconditionGoalDefinition<'a>;
    type IntoIter = std::vec::IntoIter<PreconditionGoalDefinition<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> From<PreconditionGoalDefinition<'a>> for PreconditionGoalDefinitions<'a> {
    fn from(value: PreconditionGoalDefinition<'a>) -> Self {
        PreconditionGoalDefinitions::new(vec![value])
    }
}

impl<'a> From<Option<PreconditionGoalDefinition<'a>>> for PreconditionGoalDefinitions<'a> {
    fn from(value: Option<PreconditionGoalDefinition<'a>>) -> Self {
        match value {
            None => PreconditionGoalDefinitions::default(),
            Some(value) => value.into(),
        }
    }
}

impl<'a> From<Option<PreconditionGoalDefinitions<'a>>> for PreconditionGoalDefinitions<'a> {
    fn from(value: Option<PreconditionGoalDefinitions<'a>>) -> Self {
        match value {
            None => PreconditionGoalDefinitions::default(),
            Some(values) => values,
        }
    }
}

impl<'a> From<PreconditionGoalDefinitions<'a>> for Vec<PreconditionGoalDefinition<'a>> {
    fn from(value: PreconditionGoalDefinitions<'a>) -> Self {
        value.0
    }
}

impl<'a> TryInto<PreconditionGoalDefinition<'a>> for PreconditionGoalDefinitions<'a> {
    type Error = ();

    fn try_into(self) -> Result<PreconditionGoalDefinition<'a>, Self::Error> {
        self.try_get_single().ok_or(())
    }
}

/// A precondition goal definition.
///
/// ## Usage
/// Used by [`PreconditionGoalDefinitions`].
#[derive(Debug, Clone, PartialEq)]
pub enum PreconditionGoalDefinition<'a> {
    /// ## Requirements
    /// None per se: this branch may expand into [`PreferenceGD::Goal`](PreferenceGD::Goal),
    /// which has no requirements.
    Preference(PreferenceGD<'a>),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables<'a>, PreconditionGoalDefinitions<'a>),
}

impl<'a> PreconditionGoalDefinition<'a> {
    pub fn new_and<I: IntoIterator<Item = PreconditionGoalDefinition<'a>>>(
        iter: I,
    ) -> PreconditionGoalDefinitions<'a> {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        PreconditionGoalDefinitions::from_iter(iter)
    }

    /// Constructs a new [`Preference`](Self::Preference) variant.
    pub const fn new_preference(pref: PreferenceGD<'a>) -> Self {
        Self::Preference(pref)
    }

    /// Constructs a new [`Forall`](Self::Forall) variant.
    pub const fn new_forall(
        variables: TypedVariables<'a>,
        gd: PreconditionGoalDefinitions<'a>,
    ) -> Self {
        Self::Forall(variables, gd)
    }
}

impl<'a> From<PreferenceGD<'a>> for PreconditionGoalDefinition<'a> {
    fn from(value: PreferenceGD<'a>) -> Self {
        PreconditionGoalDefinition::new_preference(value)
    }
}

impl<'a> From<Preference<'a>> for PreconditionGoalDefinition<'a> {
    fn from(value: Preference<'a>) -> Self {
        PreconditionGoalDefinition::new_preference(PreferenceGD::from_preference(value))
    }
}
