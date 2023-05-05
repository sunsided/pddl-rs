//! Contains precondition goal definitions.

use crate::types::TypedVariables;
use crate::types::{Preference, PreferenceGD};
use std::ops::Deref;

/// Zero, one or many precondition goal definitions.
///
/// ## Usage
/// Used by [`GoalDef`](crate::GoalDef), as well as [`ActionDefinition`](crate::ActionDefinition).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreconditionGoalDefinitions(Vec<PreconditionGoalDefinition>);

impl PreconditionGoalDefinitions {
    /// Constructs a new instance from the provided vector of values.
    pub const fn new(values: Vec<PreconditionGoalDefinition>) -> Self {
        Self(values)
    }

    /// Constructs a list containing a single [`PreconditionGoalDefinition::Preference`] variant.
    pub fn new_preference(pref: PreferenceGD) -> Self {
        PreconditionGoalDefinition::new_preference(pref).into()
    }

    /// Constructs a list containing a single [`PreconditionGoalDefinition::Forall`] variant.
    pub fn new_forall(variables: TypedVariables, gd: PreconditionGoalDefinitions) -> Self {
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
    pub fn iter(&self) -> std::slice::Iter<PreconditionGoalDefinition> {
        self.0.iter()
    }

    /// Get the only element of this list if the list has
    /// exactly one element. Returns [`None`] in all other cases.
    pub fn try_get_single(self) -> Option<PreconditionGoalDefinition> {
        if self.len() == 1 {
            self.into_iter().next()
        } else {
            None
        }
    }
}

impl FromIterator<PreconditionGoalDefinition> for PreconditionGoalDefinitions {
    fn from_iter<T: IntoIterator<Item = PreconditionGoalDefinition>>(iter: T) -> Self {
        PreconditionGoalDefinitions::new(iter.into_iter().collect())
    }
}

impl Deref for PreconditionGoalDefinitions {
    type Target = [PreconditionGoalDefinition];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl AsRef<[PreconditionGoalDefinition]> for PreconditionGoalDefinitions {
    fn as_ref(&self) -> &[PreconditionGoalDefinition] {
        self.0.as_slice()
    }
}

impl IntoIterator for PreconditionGoalDefinitions {
    type Item = PreconditionGoalDefinition;
    type IntoIter = std::vec::IntoIter<PreconditionGoalDefinition>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<PreconditionGoalDefinition> for PreconditionGoalDefinitions {
    fn from(value: PreconditionGoalDefinition) -> Self {
        PreconditionGoalDefinitions::new(vec![value])
    }
}

impl From<Option<PreconditionGoalDefinition>> for PreconditionGoalDefinitions {
    fn from(value: Option<PreconditionGoalDefinition>) -> Self {
        match value {
            None => PreconditionGoalDefinitions::default(),
            Some(value) => value.into(),
        }
    }
}

impl From<Option<PreconditionGoalDefinitions>> for PreconditionGoalDefinitions {
    fn from(value: Option<PreconditionGoalDefinitions>) -> Self {
        match value {
            None => PreconditionGoalDefinitions::default(),
            Some(values) => values,
        }
    }
}

impl From<PreconditionGoalDefinitions> for Vec<PreconditionGoalDefinition> {
    fn from(value: PreconditionGoalDefinitions) -> Self {
        value.0
    }
}

impl TryInto<PreconditionGoalDefinition> for PreconditionGoalDefinitions {
    type Error = ();

    fn try_into(self) -> Result<PreconditionGoalDefinition, Self::Error> {
        self.try_get_single().ok_or(())
    }
}

/// A precondition goal definition.
///
/// ## Usage
/// Used by [`PreconditionGoalDefinitions`].
#[derive(Debug, Clone, PartialEq)]
pub enum PreconditionGoalDefinition {
    /// ## Requirements
    /// None per se: this branch may expand into [`PreferenceGD::Goal`](PreferenceGD::Goal),
    /// which has no requirements.
    Preference(PreferenceGD),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables, PreconditionGoalDefinitions),
}

impl PreconditionGoalDefinition {
    pub fn new_and<I: IntoIterator<Item = PreconditionGoalDefinition>>(
        iter: I,
    ) -> PreconditionGoalDefinitions {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        PreconditionGoalDefinitions::from_iter(iter)
    }

    /// Constructs a new [`Preference`](Self::Preference) variant.
    pub const fn new_preference(pref: PreferenceGD) -> Self {
        Self::Preference(pref)
    }

    /// Constructs a new [`Forall`](Self::Forall) variant.
    pub const fn new_forall(variables: TypedVariables, gd: PreconditionGoalDefinitions) -> Self {
        Self::Forall(variables, gd)
    }
}

impl From<PreferenceGD> for PreconditionGoalDefinition {
    fn from(value: PreferenceGD) -> Self {
        PreconditionGoalDefinition::new_preference(value)
    }
}

impl From<Preference> for PreconditionGoalDefinition {
    fn from(value: Preference) -> Self {
        PreconditionGoalDefinition::new_preference(PreferenceGD::from_preference(value))
    }
}
