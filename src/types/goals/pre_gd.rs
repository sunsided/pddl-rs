//! Contains precondition goal descriptions.

use crate::types::TypedVariables;
use crate::types::{Preference, PreferenceGoalDescription};
use std::ops::Deref;

/// Zero, one or many precondition goal descriptions.
///
/// ## Usage
/// Used by [`GoalDef`](crate::GoalDef), as well as [`ActionDefinition`](crate::ActionDefinition).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PreconditionGoalDescriptions(Vec<PreconditionGoalDescription>);

impl PreconditionGoalDescriptions {
    /// Constructs a new instance from the provided vector of values.
    #[must_use]
    pub const fn new(values: Vec<PreconditionGoalDescription>) -> Self {
        Self(values)
    }

    /// Constructs a list containing a single [`PreconditionGoalDescription::Preference`] variant.
    #[doc(alias = "new_preference")]
    #[must_use]
    pub fn preference(pref: PreferenceGoalDescription) -> Self {
        PreconditionGoalDescription::preference(pref).into()
    }

    /// Alias for [`Self::preference`].
    #[must_use]
    pub fn new_preference(pref: PreferenceGoalDescription) -> Self {
        Self::preference(pref)
    }

    /// Constructs a list containing a single [`PreconditionGoalDescription::Forall`] variant.
    #[doc(alias = "new_forall")]
    #[must_use]
    pub fn r#forall(variables: TypedVariables, gd: PreconditionGoalDescriptions) -> Self {
        PreconditionGoalDescription::r#forall(variables, gd).into()
    }

    /// Alias for [`Self::forall`].
    #[must_use]
    pub fn new_forall(variables: TypedVariables, gd: PreconditionGoalDescriptions) -> Self {
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
    pub fn iter(&self) -> std::slice::Iter<'_, PreconditionGoalDescription> {
        self.0.iter()
    }

    /// Get the only element of this list if the list has
    /// exactly one element. Returns [`None`] in all other cases.
    #[must_use]
    pub fn try_get_single(self) -> Option<PreconditionGoalDescription> {
        if self.len() == 1 {
            self.into_iter().next()
        } else {
            None
        }
    }
}

impl FromIterator<PreconditionGoalDescription> for PreconditionGoalDescriptions {
    fn from_iter<T: IntoIterator<Item = PreconditionGoalDescription>>(iter: T) -> Self {
        PreconditionGoalDescriptions::new(iter.into_iter().collect())
    }
}

impl Deref for PreconditionGoalDescriptions {
    type Target = [PreconditionGoalDescription];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl AsRef<[PreconditionGoalDescription]> for PreconditionGoalDescriptions {
    fn as_ref(&self) -> &[PreconditionGoalDescription] {
        self.0.as_slice()
    }
}

impl IntoIterator for PreconditionGoalDescriptions {
    type Item = PreconditionGoalDescription;
    type IntoIter = std::vec::IntoIter<PreconditionGoalDescription>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<PreconditionGoalDescription> for PreconditionGoalDescriptions {
    fn from(value: PreconditionGoalDescription) -> Self {
        PreconditionGoalDescriptions::new(vec![value])
    }
}

impl From<Option<PreconditionGoalDescription>> for PreconditionGoalDescriptions {
    fn from(value: Option<PreconditionGoalDescription>) -> Self {
        match value {
            None => PreconditionGoalDescriptions::default(),
            Some(value) => value.into(),
        }
    }
}

impl From<Option<PreconditionGoalDescriptions>> for PreconditionGoalDescriptions {
    fn from(value: Option<PreconditionGoalDescriptions>) -> Self {
        value.unwrap_or_default()
    }
}

impl From<PreconditionGoalDescriptions> for Vec<PreconditionGoalDescription> {
    fn from(value: PreconditionGoalDescriptions) -> Self {
        value.0
    }
}

impl TryInto<PreconditionGoalDescription> for PreconditionGoalDescriptions {
    type Error = ();

    fn try_into(self) -> Result<PreconditionGoalDescription, Self::Error> {
        self.try_get_single().ok_or(())
    }
}

/// A precondition goal description.
///
/// ## Usage
/// Used by [`PreconditionGoalDescriptions`].
#[derive(Debug, Clone, PartialEq)]
pub enum PreconditionGoalDescription {
    /// ## Requirements
    /// None per se: this branch may expand into [`PreferenceGoalDescription::Goal`],
    /// which has no requirements.
    Preference(PreferenceGoalDescription),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables, PreconditionGoalDescriptions),
}

impl PreconditionGoalDescription {
    /// Creates a conjunction `(and <pre-GD>*)`, represented as a list of
    /// precondition goal descriptions.
    #[doc(alias = "new_and")]
    pub fn and<I: IntoIterator<Item = PreconditionGoalDescription>>(
        iter: I,
    ) -> PreconditionGoalDescriptions {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        PreconditionGoalDescriptions::from_iter(iter)
    }

    /// Alias for [`Self::and`].
    pub fn new_and<I: IntoIterator<Item = PreconditionGoalDescription>>(
        iter: I,
    ) -> PreconditionGoalDescriptions {
        Self::and(iter)
    }

    /// Constructs a new [`Preference`](Self::Preference) variant.
    #[doc(alias = "new_preference")]
    #[must_use]
    pub const fn preference(pref: PreferenceGoalDescription) -> Self {
        Self::Preference(pref)
    }

    /// Alias for [`Self::preference`].
    #[must_use]
    pub fn new_preference(pref: PreferenceGoalDescription) -> Self {
        Self::preference(pref)
    }

    /// Constructs a new [`Forall`](Self::Forall) variant.
    #[doc(alias = "new_forall")]
    #[must_use]
    pub const fn r#forall(variables: TypedVariables, gd: PreconditionGoalDescriptions) -> Self {
        Self::Forall(variables, gd)
    }

    /// Alias for [`Self::forall`].
    #[must_use]
    pub fn new_forall(variables: TypedVariables, gd: PreconditionGoalDescriptions) -> Self {
        Self::r#forall(variables, gd)
    }
}

impl From<PreferenceGoalDescription> for PreconditionGoalDescription {
    fn from(value: PreferenceGoalDescription) -> Self {
        PreconditionGoalDescription::preference(value)
    }
}

impl From<Preference> for PreconditionGoalDescription {
    fn from(value: Preference) -> Self {
        PreconditionGoalDescription::preference(PreferenceGoalDescription::from_preference(value))
    }
}

impl<'a> IntoIterator for &'a PreconditionGoalDescriptions {
    type Item = &'a PreconditionGoalDescription;
    type IntoIter = std::slice::Iter<'a, PreconditionGoalDescription>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{GoalDescription, PreferenceGoalDescription};
    use assert2::check;

    fn make_preference_goal() -> PreferenceGoalDescription {
        let gd = GoalDescription::and(Vec::new());
        PreferenceGoalDescription::from_gd(gd)
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_new_constructors() {
        let pref = make_preference_goal();
        let _ = PreconditionGoalDescriptions::new_preference(pref.clone());
        let _ = PreconditionGoalDescriptions::new_forall(
            TypedVariables::default(),
            PreconditionGoalDescriptions::default(),
        );
    }

    #[test]
    fn pre_gd_from_preference_goal() {
        let pref = make_preference_goal();
        let from_pref: PreconditionGoalDescription = pref.clone().into();
        check!(from_pref == PreconditionGoalDescription::preference(pref));
    }

    #[test]
    fn pre_gd_from_preference() {
        let gd = GoalDescription::and(Vec::new());
        let pref = crate::Preference::new(None, gd);
        let from_pref: PreconditionGoalDescription = pref.into();
        check!(let PreconditionGoalDescription::Preference(_) = from_pref);
    }

    #[test]
    fn pre_gds_from_option_some() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDescription::preference(pref);
        let from_some: PreconditionGoalDescriptions = Some(single.clone()).into();
        check!(from_some.len() == 1);
        check!(from_some.try_get_single() == Some(single));
    }

    #[test]
    fn pre_gds_from_option_none() {
        let from_none: PreconditionGoalDescriptions =
            Option::<PreconditionGoalDescription>::None.into();
        check!(from_none.is_empty());
    }

    #[test]
    fn pre_gds_from_option_list_some() {
        let list = PreconditionGoalDescriptions::default();
        let from_some: PreconditionGoalDescriptions = Some(list).into();
        check!(from_some.is_empty());
    }

    #[test]
    fn pre_gds_from_option_list_none() {
        let from_none: PreconditionGoalDescriptions =
            Option::<PreconditionGoalDescriptions>::None.into();
        check!(from_none.is_empty());
    }

    #[test]
    fn pre_gds_deref() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDescription::preference(pref);
        let list = PreconditionGoalDescriptions::new(vec![single.clone()]);
        let slice: &[PreconditionGoalDescription] = &list;
        check!(slice.len() == 1);
        check!(&slice[0] == &single);
    }

    #[test]
    fn pre_gds_as_ref() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDescription::preference(pref);
        let list = PreconditionGoalDescriptions::new(vec![single]);
        let slice: &[PreconditionGoalDescription] = list.as_ref();
        check!(slice.len() == 1);
    }

    #[test]
    fn pre_gds_into_iter() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDescription::preference(pref.clone());
        let list = PreconditionGoalDescriptions::new(vec![single.clone()]);
        let collected: Vec<_> = list.into_iter().collect();
        check!(collected == vec![single]);
    }

    #[test]
    fn pre_gds_into_vec() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDescription::preference(pref.clone());
        let list = PreconditionGoalDescriptions::new(vec![single.clone()]);
        let vec: Vec<PreconditionGoalDescription> = list.into();
        check!(vec == vec![single]);
    }

    #[test]
    fn pre_gds_try_get_single() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDescription::preference(pref.clone());
        let list = PreconditionGoalDescriptions::new(vec![single.clone()]);
        check!(list.try_get_single() == Some(single.clone()));

        let empty = PreconditionGoalDescriptions::default();
        check!(empty.try_get_single().is_none());

        let multi = PreconditionGoalDescriptions::new(vec![single.clone(), single]);
        check!(multi.try_get_single().is_none());
    }

    #[test]
    fn pre_gds_try_into() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDescription::preference(pref.clone());
        let list = PreconditionGoalDescriptions::new(vec![single.clone()]);
        let result: Result<PreconditionGoalDescription, ()> = list.try_into();
        check!(result.is_ok());
        check!(result.unwrap() == single);
    }

    #[test]
    fn pre_gd_and() {
        let pref = make_preference_goal();
        let items = vec![
            PreconditionGoalDescription::preference(pref.clone()),
            PreconditionGoalDescription::preference(pref),
        ];
        let result: PreconditionGoalDescriptions = PreconditionGoalDescription::and(items);
        check!(result.len() == 2);
    }

    #[test]
    #[allow(deprecated)]
    fn pre_gd_and_deprecated() {
        let pref = make_preference_goal();
        let _ =
            PreconditionGoalDescription::new_and(vec![PreconditionGoalDescription::preference(
                pref,
            )]);
    }

    #[test]
    fn pre_gd_forall() {
        let vars = TypedVariables::default();
        let gds = PreconditionGoalDescriptions::default();
        let forall = PreconditionGoalDescription::r#forall(vars.clone(), gds.clone());
        check!(let PreconditionGoalDescription::Forall(_, _) = forall);
    }
}
