//! Contains the types [`PreferenceConstraintGoalDefinition`] and [`PreferenceConstraintGoalDefinitions`].

use crate::types::{ConstraintGoalDefinition, PreferenceName, TypedVariables};
use std::ops::Deref;

/// A list of [`PreferenceConstraintGoalDefinition`] values. This represents the `(and ...)` variant
/// of the PDDL definition, modeling cases of zero, one or many values.
///
/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
///
/// ## Usage
/// Used by [`ProblemConstraintsDef`](crate::ProblemConstraintsDef).
#[derive(Debug, Clone, PartialEq, Default)]
#[doc(alias = "pref-con-GD")]
pub struct PreferenceConstraintGoalDefinitions(Vec<PreferenceConstraintGoalDefinition>);

impl PreferenceConstraintGoalDefinitions {
    /// Constructs a new instance from the provided vector of values.
    pub const fn new(gds: Vec<PreferenceConstraintGoalDefinition>) -> Self {
        Self(gds)
    }

    /// Constructs a list containing a single [`PreferenceConstraintGoalDefinition::Goal`] variant.
    pub fn new_goal(gd: ConstraintGoalDefinition) -> Self {
        Self::new(vec![PreferenceConstraintGoalDefinition::new_goal(gd)])
    }

    /// Constructs a list containing a single [`PreferenceConstraintGoalDefinition::Preference`] variant.
    ///
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    pub fn new_preference(name: Option<PreferenceName>, gd: ConstraintGoalDefinition) -> Self {
        Self::new(vec![PreferenceConstraintGoalDefinition::new_preference(
            name, gd,
        )])
    }

    /// Constructs a list containing a single [`PreferenceConstraintGoalDefinition::Forall`] variant.
    ///
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    pub fn new_forall(variables: TypedVariables, gd: PreferenceConstraintGoalDefinitions) -> Self {
        Self::new(vec![PreferenceConstraintGoalDefinition::new_forall(
            variables, gd,
        )])
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
    pub fn iter(&self) -> std::slice::Iter<'_, PreferenceConstraintGoalDefinition> {
        self.0.iter()
    }

    /// Get the only element of this list if the list has
    /// exactly one element. Returns [`None`] in all other cases.
    pub fn try_get_single(self) -> Option<PreferenceConstraintGoalDefinition> {
        if self.len() == 1 {
            self.into_iter().next()
        } else {
            None
        }
    }
}

impl Deref for PreferenceConstraintGoalDefinitions {
    type Target = [PreferenceConstraintGoalDefinition];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl AsRef<[PreferenceConstraintGoalDefinition]> for PreferenceConstraintGoalDefinitions {
    fn as_ref(&self) -> &[PreferenceConstraintGoalDefinition] {
        self.0.as_slice()
    }
}

impl IntoIterator for PreferenceConstraintGoalDefinitions {
    type Item = PreferenceConstraintGoalDefinition;
    type IntoIter = std::vec::IntoIter<PreferenceConstraintGoalDefinition>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<PreferenceConstraintGoalDefinitions> for Vec<PreferenceConstraintGoalDefinition> {
    fn from(value: PreferenceConstraintGoalDefinitions) -> Self {
        value.0
    }
}

impl From<PreferenceConstraintGoalDefinition> for PreferenceConstraintGoalDefinitions {
    fn from(value: PreferenceConstraintGoalDefinition) -> Self {
        PreferenceConstraintGoalDefinitions::new(vec![value])
    }
}

impl From<Vec<PreferenceConstraintGoalDefinition>> for PreferenceConstraintGoalDefinitions {
    fn from(value: Vec<PreferenceConstraintGoalDefinition>) -> Self {
        PreferenceConstraintGoalDefinitions::new(value)
    }
}

impl FromIterator<PreferenceConstraintGoalDefinition> for PreferenceConstraintGoalDefinitions {
    fn from_iter<T: IntoIterator<Item = PreferenceConstraintGoalDefinition>>(iter: T) -> Self {
        PreferenceConstraintGoalDefinitions::new(iter.into_iter().collect())
    }
}

impl From<Option<PreferenceConstraintGoalDefinition>> for PreferenceConstraintGoalDefinitions {
    fn from(value: Option<PreferenceConstraintGoalDefinition>) -> Self {
        match value {
            None => PreferenceConstraintGoalDefinitions::default(),
            Some(value) => value.into(),
        }
    }
}

impl From<Option<PreferenceConstraintGoalDefinitions>> for PreferenceConstraintGoalDefinitions {
    fn from(value: Option<PreferenceConstraintGoalDefinitions>) -> Self {
        value.unwrap_or_default()
    }
}

impl From<ConstraintGoalDefinition> for PreferenceConstraintGoalDefinitions {
    fn from(value: ConstraintGoalDefinition) -> Self {
        PreferenceConstraintGoalDefinitions::new_goal(value)
    }
}

impl TryInto<PreferenceConstraintGoalDefinition> for PreferenceConstraintGoalDefinitions {
    type Error = ();

    fn try_into(self) -> Result<PreferenceConstraintGoalDefinition, Self::Error> {
        self.try_get_single().ok_or(())
    }
}

/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
///
/// ## Usage
/// Used by [`PreferenceConstraintGoalDefinitions`] itself, as well as [`ProblemConstraintsDef`](crate::ProblemConstraintsDef).
#[derive(Debug, Clone, PartialEq)]
#[doc(alias = "pref-con-GD")]
pub enum PreferenceConstraintGoalDefinition {
    Goal(ConstraintGoalDefinition),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables, PreferenceConstraintGoalDefinitions),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Option<PreferenceName>, ConstraintGoalDefinition),
}

impl PreferenceConstraintGoalDefinition {
    pub const fn new_goal(gd: ConstraintGoalDefinition) -> Self {
        Self::Goal(gd)
    }

    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    pub fn new_forall(variables: TypedVariables, gd: PreferenceConstraintGoalDefinitions) -> Self {
        Self::Forall(variables, gd)
    }

    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    pub const fn new_preference(
        name: Option<PreferenceName>,
        gd: ConstraintGoalDefinition,
    ) -> Self {
        Self::Preference(name, gd)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            PreferenceConstraintGoalDefinition::Forall(_, x) => x.is_empty(),
            PreferenceConstraintGoalDefinition::Preference(_, x) => x.is_empty(),
            PreferenceConstraintGoalDefinition::Goal(x) => x.is_empty(),
        }
    }
}

/// Alias for [`PreferenceConstraintGoalDefinition`]; matches BNF `<pref-con-GD>`.
#[deprecated(
    since = "0.2.0",
    note = "Use `PreferenceConstraintGoalDefinition` instead"
)]
pub type PrefConGD = PreferenceConstraintGoalDefinition;

/// Alias for [`PreferenceConstraintGoalDefinitions`]; matches BNF `<pref-con-GD>` (list form).
#[deprecated(
    since = "0.2.0",
    note = "Use `PreferenceConstraintGoalDefinitions` instead"
)]
pub type PrefConGDs = PreferenceConstraintGoalDefinitions;
