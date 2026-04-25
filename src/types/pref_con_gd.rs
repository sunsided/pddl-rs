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
    #[doc(alias = "new_goal")]
    pub fn goal(gd: ConstraintGoalDefinition) -> Self {
        Self::new(vec![PreferenceConstraintGoalDefinition::goal(gd)])
    }

    pub fn new_goal(gd: ConstraintGoalDefinition) -> Self {
        Self::goal(gd)
    }

    /// Constructs a list containing a single [`PreferenceConstraintGoalDefinition::Preference`] variant.
    ///
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    #[doc(alias = "new_preference")]
    pub fn preference(name: Option<PreferenceName>, gd: ConstraintGoalDefinition) -> Self {
        Self::new(vec![PreferenceConstraintGoalDefinition::preference(
            name, gd,
        )])
    }

    pub fn new_preference(name: Option<PreferenceName>, gd: ConstraintGoalDefinition) -> Self {
        Self::preference(name, gd)
    }

    /// Constructs a list containing a single [`PreferenceConstraintGoalDefinition::Forall`] variant.
    ///
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    #[doc(alias = "new_forall")]
    pub fn r#forall(variables: TypedVariables, gd: PreferenceConstraintGoalDefinitions) -> Self {
        Self::new(vec![PreferenceConstraintGoalDefinition::r#forall(
            variables, gd,
        )])
    }

    pub fn new_forall(variables: TypedVariables, gd: PreferenceConstraintGoalDefinitions) -> Self {
        Self::r#forall(variables, gd)
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
        PreferenceConstraintGoalDefinitions::goal(value)
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
    #[doc(alias = "new_goal")]
    pub const fn goal(gd: ConstraintGoalDefinition) -> Self {
        Self::Goal(gd)
    }

    pub fn new_goal(gd: ConstraintGoalDefinition) -> Self {
        Self::goal(gd)
    }

    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    #[doc(alias = "new_forall")]
    pub fn r#forall(variables: TypedVariables, gd: PreferenceConstraintGoalDefinitions) -> Self {
        Self::Forall(variables, gd)
    }

    pub fn new_forall(variables: TypedVariables, gd: PreferenceConstraintGoalDefinitions) -> Self {
        Self::r#forall(variables, gd)
    }

    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    #[doc(alias = "new_preference")]
    pub const fn preference(name: Option<PreferenceName>, gd: ConstraintGoalDefinition) -> Self {
        Self::Preference(name, gd)
    }

    pub fn new_preference(name: Option<PreferenceName>, gd: ConstraintGoalDefinition) -> Self {
        Self::preference(name, gd)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pref_con_gd_is_empty() {
        let gds = PreferenceConstraintGoalDefinitions::default();
        assert!(gds.is_empty());

        let inner = ConstraintGoalDefinition::default();
        let goal = PreferenceConstraintGoalDefinition::goal(inner);
        let list = PreferenceConstraintGoalDefinitions::new(vec![goal]);
        assert!(!list.is_empty());
    }

    #[test]
    fn pref_con_gd_try_get_single() {
        let inner = ConstraintGoalDefinition::default();
        let goal = PreferenceConstraintGoalDefinition::goal(inner);
        let list = PreferenceConstraintGoalDefinitions::new(vec![goal.clone()]);
        assert_eq!(list.try_get_single(), Some(goal));

        let empty = PreferenceConstraintGoalDefinitions::default();
        assert!(empty.try_get_single().is_none());
    }

    #[test]
    fn pref_con_gd_is_empty_variants() {
        let inner = ConstraintGoalDefinition::default();
        let goal = PreferenceConstraintGoalDefinition::goal(inner.clone());
        assert!(goal.is_empty());

        let pref = PreferenceConstraintGoalDefinition::preference(None, inner.clone());
        assert!(pref.is_empty());

        let forall = PreferenceConstraintGoalDefinition::r#forall(
            crate::types::TypedVariables::default(),
            PreferenceConstraintGoalDefinitions::default(),
        );
        assert!(forall.is_empty());
    }

    #[test]
    fn pref_con_gd_from_variants() {
        let inner = ConstraintGoalDefinition::default();
        let from_inner: PreferenceConstraintGoalDefinitions = inner.into();
        assert_eq!(from_inner.len(), 1);

        let single = PreferenceConstraintGoalDefinition::Goal(ConstraintGoalDefinition::default());
        let from_single: PreferenceConstraintGoalDefinitions = single.into();
        assert_eq!(from_single.len(), 1);

        let vec = vec![PreferenceConstraintGoalDefinition::goal(
            ConstraintGoalDefinition::default(),
        )];
        let from_vec: PreferenceConstraintGoalDefinitions = vec.into();
        assert_eq!(from_vec.len(), 1);

        let from_some: PreferenceConstraintGoalDefinitions = Some(
            PreferenceConstraintGoalDefinition::goal(ConstraintGoalDefinition::default()),
        )
        .into();
        assert_eq!(from_some.len(), 1);

        let from_none: PreferenceConstraintGoalDefinitions =
            Option::<PreferenceConstraintGoalDefinition>::None.into();
        assert!(from_none.is_empty());

        let from_some_list: PreferenceConstraintGoalDefinitions = Some(
            PreferenceConstraintGoalDefinitions::goal(ConstraintGoalDefinition::default()),
        )
        .into();
        assert_eq!(from_some_list.len(), 1);

        let from_none_list: PreferenceConstraintGoalDefinitions =
            Option::<PreferenceConstraintGoalDefinitions>::None.into();
        assert!(from_none_list.is_empty());
    }

    #[test]
    fn pref_con_gd_try_into() {
        let goal = PreferenceConstraintGoalDefinition::goal(ConstraintGoalDefinition::default());
        let list = PreferenceConstraintGoalDefinitions::new(vec![goal.clone()]);
        let result: Result<PreferenceConstraintGoalDefinition, ()> = list.try_into();
        assert!(result.is_ok());
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_aliases_exist() {
        fn _assert_alias(_: PrefConGD) {}
        fn _assert_aliases(_: PrefConGDs) {}
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_new_constructors() {
        let inner = ConstraintGoalDefinition::default();
        let _ = PreferenceConstraintGoalDefinitions::new_goal(inner.clone());
        let _ = PreferenceConstraintGoalDefinitions::new_preference(None, inner.clone());
        let _ = PreferenceConstraintGoalDefinitions::new_forall(
            crate::types::TypedVariables::default(),
            PreferenceConstraintGoalDefinitions::default(),
        );

        // PreferenceConstraintGoalDefinition
        let _ = PreferenceConstraintGoalDefinition::new_goal(inner.clone());
        let _ = PreferenceConstraintGoalDefinition::new_preference(None, inner.clone());
        let _ = PreferenceConstraintGoalDefinition::new_forall(
            crate::types::TypedVariables::default(),
            PreferenceConstraintGoalDefinitions::default(),
        );
    }

    #[test]
    fn pre_con_gds_deref() {
        let inner = ConstraintGoalDefinition::default();
        let goal = PreferenceConstraintGoalDefinition::goal(inner);
        let list = PreferenceConstraintGoalDefinitions::new(vec![goal.clone()]);
        let slice: &[PreferenceConstraintGoalDefinition] = &list;
        assert_eq!(slice.len(), 1);
        assert_eq!(&slice[0], &goal);
    }

    #[test]
    fn pre_con_gds_as_ref() {
        let inner = ConstraintGoalDefinition::default();
        let goal = PreferenceConstraintGoalDefinition::goal(inner);
        let list = PreferenceConstraintGoalDefinitions::new(vec![goal]);
        let slice: &[PreferenceConstraintGoalDefinition] = list.as_ref();
        assert_eq!(slice.len(), 1);
    }

    #[test]
    fn pre_con_gds_into_iter() {
        let inner = ConstraintGoalDefinition::default();
        let goal = PreferenceConstraintGoalDefinition::goal(inner.clone());
        let list = PreferenceConstraintGoalDefinitions::new(vec![goal.clone()]);
        let collected: Vec<_> = list.into_iter().collect();
        assert_eq!(collected, vec![goal]);
    }

    #[test]
    fn pre_con_gds_into_vec() {
        let inner = ConstraintGoalDefinition::default();
        let goal = PreferenceConstraintGoalDefinition::goal(inner.clone());
        let list = PreferenceConstraintGoalDefinitions::new(vec![goal.clone()]);
        let vec: Vec<PreferenceConstraintGoalDefinition> = list.into();
        assert_eq!(vec, vec![goal]);
    }

    #[test]
    fn pre_con_gds_from_iterator() {
        let inner = ConstraintGoalDefinition::default();
        let goal = PreferenceConstraintGoalDefinition::goal(inner.clone());
        let from_iter: PreferenceConstraintGoalDefinitions =
            vec![goal.clone()].into_iter().collect();
        assert_eq!(from_iter.len(), 1);
        assert_eq!(from_iter.try_get_single(), Some(goal));
    }

    #[test]
    fn pre_con_gds_len() {
        let list = PreferenceConstraintGoalDefinitions::default();
        assert_eq!(list.len(), 0);

        let inner = ConstraintGoalDefinition::default();
        let goal = PreferenceConstraintGoalDefinition::goal(inner);
        let list = PreferenceConstraintGoalDefinitions::new(vec![goal]);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn pre_con_gds_iter() {
        let inner1 = ConstraintGoalDefinition::default();
        let inner2 = ConstraintGoalDefinition::And(vec![]);
        let goal1 = PreferenceConstraintGoalDefinition::goal(inner1);
        let goal2 = PreferenceConstraintGoalDefinition::goal(inner2);
        let list = PreferenceConstraintGoalDefinitions::new(vec![goal1.clone(), goal2.clone()]);
        let collected: Vec<_> = list.iter().cloned().collect();
        assert_eq!(collected, vec![goal1, goal2]);
    }
}
