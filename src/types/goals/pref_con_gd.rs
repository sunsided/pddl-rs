//! Contains the types [`PreferenceConstraintGoalDescription`] and [`PreferenceConstraintGoalDescriptions`].

use crate::types::{ConstraintGoalDescription, PreferenceName, TypedVariables};
use std::ops::Deref;

/// A list of [`PreferenceConstraintGoalDescription`] values. This represents the `(and ...)` variant
/// of the PDDL definition, modeling cases of zero, one or many values.
///
/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
///
/// ## Usage
/// Used by [`ProblemConstraintsDef`](crate::ProblemConstraintsDef).
#[derive(Debug, Clone, PartialEq, Default)]
#[doc(alias = "pref-con-GD")]
pub struct PreferenceConstraintGoalDescriptions(Vec<PreferenceConstraintGoalDescription>);

impl PreferenceConstraintGoalDescriptions {
    /// Constructs a new instance from the provided vector of values.
    #[must_use]
    pub const fn new(gds: Vec<PreferenceConstraintGoalDescription>) -> Self {
        Self(gds)
    }

    /// Constructs a list containing a single [`PreferenceConstraintGoalDescription::Goal`] variant.
    #[doc(alias = "new_goal")]
    #[must_use]
    pub fn goal(gd: ConstraintGoalDescription) -> Self {
        Self::new(vec![PreferenceConstraintGoalDescription::goal(gd)])
    }

    /// Alias for [`Self::goal`].
    #[must_use]
    pub fn new_goal(gd: ConstraintGoalDescription) -> Self {
        Self::goal(gd)
    }

    /// Constructs a list containing a single [`PreferenceConstraintGoalDescription::Preference`] variant.
    ///
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    #[doc(alias = "new_preference")]
    #[must_use]
    pub fn preference(name: Option<PreferenceName>, gd: ConstraintGoalDescription) -> Self {
        Self::new(vec![PreferenceConstraintGoalDescription::preference(
            name, gd,
        )])
    }

    /// Alias for [`Self::preference`].
    #[must_use]
    pub fn new_preference(name: Option<PreferenceName>, gd: ConstraintGoalDescription) -> Self {
        Self::preference(name, gd)
    }

    /// Constructs a list containing a single [`PreferenceConstraintGoalDescription::Forall`] variant.
    ///
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    #[doc(alias = "new_forall")]
    #[must_use]
    pub fn r#forall(variables: TypedVariables, gd: PreferenceConstraintGoalDescriptions) -> Self {
        Self::new(vec![PreferenceConstraintGoalDescription::r#forall(
            variables, gd,
        )])
    }

    /// Alias for [`Self::forall`].
    #[must_use]
    pub fn new_forall(variables: TypedVariables, gd: PreferenceConstraintGoalDescriptions) -> Self {
        Self::r#forall(variables, gd)
    }

    /// Returns `true` if the list contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of elements in the list, also referred to
    /// as its 'length'.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns an iterator over the list.
    ///
    /// The iterator yields all items from start to end.
    pub fn iter(&self) -> std::slice::Iter<'_, PreferenceConstraintGoalDescription> {
        self.0.iter()
    }

    /// Get the only element of this list if the list has
    /// exactly one element. Returns [`None`] in all other cases.
    #[must_use]
    pub fn try_get_single(self) -> Option<PreferenceConstraintGoalDescription> {
        if self.len() == 1 {
            self.into_iter().next()
        } else {
            None
        }
    }
}

impl Deref for PreferenceConstraintGoalDescriptions {
    type Target = [PreferenceConstraintGoalDescription];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl AsRef<[PreferenceConstraintGoalDescription]> for PreferenceConstraintGoalDescriptions {
    fn as_ref(&self) -> &[PreferenceConstraintGoalDescription] {
        self.0.as_slice()
    }
}

impl IntoIterator for PreferenceConstraintGoalDescriptions {
    type Item = PreferenceConstraintGoalDescription;
    type IntoIter = std::vec::IntoIter<PreferenceConstraintGoalDescription>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<PreferenceConstraintGoalDescriptions> for Vec<PreferenceConstraintGoalDescription> {
    fn from(value: PreferenceConstraintGoalDescriptions) -> Self {
        value.0
    }
}

impl From<PreferenceConstraintGoalDescription> for PreferenceConstraintGoalDescriptions {
    fn from(value: PreferenceConstraintGoalDescription) -> Self {
        PreferenceConstraintGoalDescriptions::new(vec![value])
    }
}

impl From<Vec<PreferenceConstraintGoalDescription>> for PreferenceConstraintGoalDescriptions {
    fn from(value: Vec<PreferenceConstraintGoalDescription>) -> Self {
        PreferenceConstraintGoalDescriptions::new(value)
    }
}

impl FromIterator<PreferenceConstraintGoalDescription> for PreferenceConstraintGoalDescriptions {
    fn from_iter<T: IntoIterator<Item = PreferenceConstraintGoalDescription>>(iter: T) -> Self {
        PreferenceConstraintGoalDescriptions::new(iter.into_iter().collect())
    }
}

impl From<Option<PreferenceConstraintGoalDescription>> for PreferenceConstraintGoalDescriptions {
    fn from(value: Option<PreferenceConstraintGoalDescription>) -> Self {
        match value {
            None => PreferenceConstraintGoalDescriptions::default(),
            Some(value) => value.into(),
        }
    }
}

impl From<Option<PreferenceConstraintGoalDescriptions>> for PreferenceConstraintGoalDescriptions {
    fn from(value: Option<PreferenceConstraintGoalDescriptions>) -> Self {
        value.unwrap_or_default()
    }
}

impl From<ConstraintGoalDescription> for PreferenceConstraintGoalDescriptions {
    fn from(value: ConstraintGoalDescription) -> Self {
        PreferenceConstraintGoalDescriptions::goal(value)
    }
}

impl TryInto<PreferenceConstraintGoalDescription> for PreferenceConstraintGoalDescriptions {
    type Error = ();

    fn try_into(self) -> Result<PreferenceConstraintGoalDescription, Self::Error> {
        self.try_get_single().ok_or(())
    }
}

/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
///
/// ## Usage
/// Used by [`PreferenceConstraintGoalDescriptions`] itself, as well as [`ProblemConstraintsDef`](crate::ProblemConstraintsDef).
#[derive(Debug, Clone, PartialEq)]
#[doc(alias = "pref-con-GD")]
pub enum PreferenceConstraintGoalDescription {
    /// A required constraint goal, `<con-GD>`.
    Goal(ConstraintGoalDescription),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables, PreferenceConstraintGoalDescriptions),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Option<PreferenceName>, ConstraintGoalDescription),
}

impl PreferenceConstraintGoalDescription {
    /// Creates a required constraint goal.
    #[doc(alias = "new_goal")]
    #[must_use]
    pub const fn goal(gd: ConstraintGoalDescription) -> Self {
        Self::Goal(gd)
    }

    /// Alias for [`Self::goal`].
    #[must_use]
    pub fn new_goal(gd: ConstraintGoalDescription) -> Self {
        Self::goal(gd)
    }

    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    #[doc(alias = "new_forall")]
    #[must_use]
    pub fn r#forall(variables: TypedVariables, gd: PreferenceConstraintGoalDescriptions) -> Self {
        Self::Forall(variables, gd)
    }

    /// Alias for [`Self::forall`].
    #[must_use]
    pub fn new_forall(variables: TypedVariables, gd: PreferenceConstraintGoalDescriptions) -> Self {
        Self::r#forall(variables, gd)
    }

    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    #[doc(alias = "new_preference")]
    #[must_use]
    pub const fn preference(name: Option<PreferenceName>, gd: ConstraintGoalDescription) -> Self {
        Self::Preference(name, gd)
    }

    /// Alias for [`Self::preference`].
    #[must_use]
    pub fn new_preference(name: Option<PreferenceName>, gd: ConstraintGoalDescription) -> Self {
        Self::preference(name, gd)
    }

    /// Returns `true` if the contained constraint goal description is empty.
    #[allow(
        clippy::match_same_arms,
        reason = "the `x` bindings have different types"
    )]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            PreferenceConstraintGoalDescription::Forall(_, x) => x.is_empty(),
            PreferenceConstraintGoalDescription::Preference(_, x) => x.is_empty(),
            PreferenceConstraintGoalDescription::Goal(x) => x.is_empty(),
        }
    }
}

/// Alias for [`PreferenceConstraintGoalDescription`]; matches BNF `<pref-con-GD>`.
#[deprecated(
    since = "0.2.0",
    note = "Use `PreferenceConstraintGoalDescription` instead"
)]
pub type PrefConGD = PreferenceConstraintGoalDescription;

/// Alias for [`PreferenceConstraintGoalDescriptions`]; matches BNF `<pref-con-GD>` (list form).
#[deprecated(
    since = "0.2.0",
    note = "Use `PreferenceConstraintGoalDescriptions` instead"
)]
pub type PrefConGDs = PreferenceConstraintGoalDescriptions;

impl<'a> IntoIterator for &'a PreferenceConstraintGoalDescriptions {
    type Item = &'a PreferenceConstraintGoalDescription;
    type IntoIter = std::slice::Iter<'a, PreferenceConstraintGoalDescription>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn pref_con_gd_is_empty() {
        let gds = PreferenceConstraintGoalDescriptions::default();
        check!(gds.is_empty());

        let inner = ConstraintGoalDescription::default();
        let goal = PreferenceConstraintGoalDescription::goal(inner);
        let list = PreferenceConstraintGoalDescriptions::new(vec![goal]);
        check!(!list.is_empty());
    }

    #[test]
    fn pref_con_gd_try_get_single() {
        let inner = ConstraintGoalDescription::default();
        let goal = PreferenceConstraintGoalDescription::goal(inner);
        let list = PreferenceConstraintGoalDescriptions::new(vec![goal.clone()]);
        check!(list.try_get_single() == Some(goal));

        let empty = PreferenceConstraintGoalDescriptions::default();
        check!(empty.try_get_single().is_none());
    }

    #[test]
    fn pref_con_gd_is_empty_variants() {
        let inner = ConstraintGoalDescription::default();
        let goal = PreferenceConstraintGoalDescription::goal(inner.clone());
        check!(goal.is_empty());

        let pref = PreferenceConstraintGoalDescription::preference(None, inner.clone());
        check!(pref.is_empty());

        let forall = PreferenceConstraintGoalDescription::r#forall(
            crate::types::TypedVariables::default(),
            PreferenceConstraintGoalDescriptions::default(),
        );
        check!(forall.is_empty());
    }

    #[test]
    fn pref_con_gd_from_variants() {
        let inner = ConstraintGoalDescription::default();
        let from_inner: PreferenceConstraintGoalDescriptions = inner.into();
        check!(from_inner.len() == 1);

        let single =
            PreferenceConstraintGoalDescription::Goal(ConstraintGoalDescription::default());
        let from_single: PreferenceConstraintGoalDescriptions = single.into();
        check!(from_single.len() == 1);

        let vec = vec![PreferenceConstraintGoalDescription::goal(
            ConstraintGoalDescription::default(),
        )];
        let from_vec: PreferenceConstraintGoalDescriptions = vec.into();
        check!(from_vec.len() == 1);

        let from_some: PreferenceConstraintGoalDescriptions = Some(
            PreferenceConstraintGoalDescription::goal(ConstraintGoalDescription::default()),
        )
        .into();
        check!(from_some.len() == 1);

        let from_none: PreferenceConstraintGoalDescriptions =
            Option::<PreferenceConstraintGoalDescription>::None.into();
        check!(from_none.is_empty());

        let from_some_list: PreferenceConstraintGoalDescriptions = Some(
            PreferenceConstraintGoalDescriptions::goal(ConstraintGoalDescription::default()),
        )
        .into();
        check!(from_some_list.len() == 1);

        let from_none_list: PreferenceConstraintGoalDescriptions =
            Option::<PreferenceConstraintGoalDescriptions>::None.into();
        check!(from_none_list.is_empty());
    }

    #[test]
    fn pref_con_gd_try_into() {
        let goal = PreferenceConstraintGoalDescription::goal(ConstraintGoalDescription::default());
        let list = PreferenceConstraintGoalDescriptions::new(vec![goal.clone()]);
        let result: Result<PreferenceConstraintGoalDescription, ()> = list.try_into();
        check!(result.is_ok());
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
        let inner = ConstraintGoalDescription::default();
        let _ = PreferenceConstraintGoalDescriptions::new_goal(inner.clone());
        let _ = PreferenceConstraintGoalDescriptions::new_preference(None, inner.clone());
        let _ = PreferenceConstraintGoalDescriptions::new_forall(
            crate::types::TypedVariables::default(),
            PreferenceConstraintGoalDescriptions::default(),
        );

        // PreferenceConstraintGoalDescription
        let _ = PreferenceConstraintGoalDescription::new_goal(inner.clone());
        let _ = PreferenceConstraintGoalDescription::new_preference(None, inner.clone());
        let _ = PreferenceConstraintGoalDescription::new_forall(
            crate::types::TypedVariables::default(),
            PreferenceConstraintGoalDescriptions::default(),
        );
    }

    #[test]
    fn pre_con_gds_deref() {
        let inner = ConstraintGoalDescription::default();
        let goal = PreferenceConstraintGoalDescription::goal(inner);
        let list = PreferenceConstraintGoalDescriptions::new(vec![goal.clone()]);
        let slice: &[PreferenceConstraintGoalDescription] = &list;
        check!(slice.len() == 1);
        check!(&slice[0] == &goal);
    }

    #[test]
    fn pre_con_gds_as_ref() {
        let inner = ConstraintGoalDescription::default();
        let goal = PreferenceConstraintGoalDescription::goal(inner);
        let list = PreferenceConstraintGoalDescriptions::new(vec![goal]);
        let slice: &[PreferenceConstraintGoalDescription] = list.as_ref();
        check!(slice.len() == 1);
    }

    #[test]
    fn pre_con_gds_into_iter() {
        let inner = ConstraintGoalDescription::default();
        let goal = PreferenceConstraintGoalDescription::goal(inner.clone());
        let list = PreferenceConstraintGoalDescriptions::new(vec![goal.clone()]);
        let collected: Vec<_> = list.into_iter().collect();
        check!(collected == vec![goal]);
    }

    #[test]
    fn pre_con_gds_into_vec() {
        let inner = ConstraintGoalDescription::default();
        let goal = PreferenceConstraintGoalDescription::goal(inner.clone());
        let list = PreferenceConstraintGoalDescriptions::new(vec![goal.clone()]);
        let vec: Vec<PreferenceConstraintGoalDescription> = list.into();
        check!(vec == vec![goal]);
    }

    #[test]
    fn pre_con_gds_from_iterator() {
        let inner = ConstraintGoalDescription::default();
        let goal = PreferenceConstraintGoalDescription::goal(inner.clone());
        let from_iter: PreferenceConstraintGoalDescriptions =
            vec![goal.clone()].into_iter().collect();
        check!(from_iter.len() == 1);
        check!(from_iter.try_get_single() == Some(goal));
    }

    #[test]
    fn pre_con_gds_len() {
        let list = PreferenceConstraintGoalDescriptions::default();
        check!(list.len() == 0);

        let inner = ConstraintGoalDescription::default();
        let goal = PreferenceConstraintGoalDescription::goal(inner);
        let list = PreferenceConstraintGoalDescriptions::new(vec![goal]);
        check!(list.len() == 1);
    }

    #[test]
    fn pre_con_gds_iter() {
        let inner1 = ConstraintGoalDescription::default();
        let inner2 = ConstraintGoalDescription::And(vec![]);
        let goal1 = PreferenceConstraintGoalDescription::goal(inner1);
        let goal2 = PreferenceConstraintGoalDescription::goal(inner2);
        let list = PreferenceConstraintGoalDescriptions::new(vec![goal1.clone(), goal2.clone()]);
        let collected: Vec<_> = list.iter().cloned().collect();
        check!(collected == vec![goal1, goal2]);
    }
}
