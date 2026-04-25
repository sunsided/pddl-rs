//! Contains precondition goal definitions.

use crate::types::TypedVariables;
use crate::types::{Preference, PreferenceGoalDefinition};
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
    #[doc(alias = "new_preference")]
    pub fn preference(pref: PreferenceGoalDefinition) -> Self {
        PreconditionGoalDefinition::preference(pref).into()
    }

    pub fn new_preference(pref: PreferenceGoalDefinition) -> Self {
        Self::preference(pref)
    }

    /// Constructs a list containing a single [`PreconditionGoalDefinition::Forall`] variant.
    #[doc(alias = "new_forall")]
    pub fn r#forall(variables: TypedVariables, gd: PreconditionGoalDefinitions) -> Self {
        PreconditionGoalDefinition::r#forall(variables, gd).into()
    }

    pub fn new_forall(variables: TypedVariables, gd: PreconditionGoalDefinitions) -> Self {
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
    pub fn iter(&self) -> std::slice::Iter<'_, PreconditionGoalDefinition> {
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
        value.unwrap_or_default()
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
    /// None per se: this branch may expand into [`PreferenceGoalDefinition::Goal`](PreferenceGoalDefinition::Goal),
    /// which has no requirements.
    Preference(PreferenceGoalDefinition),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables, PreconditionGoalDefinitions),
}

impl PreconditionGoalDefinition {
    #[doc(alias = "new_and")]
    pub fn and<I: IntoIterator<Item = PreconditionGoalDefinition>>(
        iter: I,
    ) -> PreconditionGoalDefinitions {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        PreconditionGoalDefinitions::from_iter(iter)
    }

    pub fn new_and<I: IntoIterator<Item = PreconditionGoalDefinition>>(
        iter: I,
    ) -> PreconditionGoalDefinitions {
        Self::and(iter)
    }

    /// Constructs a new [`Preference`](Self::Preference) variant.
    #[doc(alias = "new_preference")]
    pub const fn preference(pref: PreferenceGoalDefinition) -> Self {
        Self::Preference(pref)
    }

    pub fn new_preference(pref: PreferenceGoalDefinition) -> Self {
        Self::preference(pref)
    }

    /// Constructs a new [`Forall`](Self::Forall) variant.
    #[doc(alias = "new_forall")]
    pub const fn r#forall(variables: TypedVariables, gd: PreconditionGoalDefinitions) -> Self {
        Self::Forall(variables, gd)
    }

    pub fn new_forall(variables: TypedVariables, gd: PreconditionGoalDefinitions) -> Self {
        Self::r#forall(variables, gd)
    }
}

impl From<PreferenceGoalDefinition> for PreconditionGoalDefinition {
    fn from(value: PreferenceGoalDefinition) -> Self {
        PreconditionGoalDefinition::preference(value)
    }
}

impl From<Preference> for PreconditionGoalDefinition {
    fn from(value: Preference) -> Self {
        PreconditionGoalDefinition::preference(PreferenceGoalDefinition::from_preference(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{GoalDefinition, PreferenceGoalDefinition};

    fn make_preference_goal() -> PreferenceGoalDefinition {
        let gd = GoalDefinition::and(Vec::new());
        PreferenceGoalDefinition::from_gd(gd)
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_new_constructors() {
        let pref = make_preference_goal();
        let _ = PreconditionGoalDefinitions::new_preference(pref.clone());
        let _ = PreconditionGoalDefinitions::new_forall(
            TypedVariables::default(),
            PreconditionGoalDefinitions::default(),
        );
    }

    #[test]
    fn pre_gd_from_preference_goal() {
        let pref = make_preference_goal();
        let from_pref: PreconditionGoalDefinition = pref.clone().into();
        assert_eq!(from_pref, PreconditionGoalDefinition::preference(pref));
    }

    #[test]
    fn pre_gd_from_preference() {
        let gd = GoalDefinition::and(Vec::new());
        let pref = crate::Preference::new(None, gd);
        let from_pref: PreconditionGoalDefinition = pref.into();
        assert!(matches!(
            from_pref,
            PreconditionGoalDefinition::Preference(_)
        ));
    }

    #[test]
    fn pre_gds_from_option_some() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDefinition::preference(pref);
        let from_some: PreconditionGoalDefinitions = Some(single.clone()).into();
        assert_eq!(from_some.len(), 1);
        assert_eq!(from_some.try_get_single(), Some(single));
    }

    #[test]
    fn pre_gds_from_option_none() {
        let from_none: PreconditionGoalDefinitions =
            Option::<PreconditionGoalDefinition>::None.into();
        assert!(from_none.is_empty());
    }

    #[test]
    fn pre_gds_from_option_list_some() {
        let list = PreconditionGoalDefinitions::default();
        let from_some: PreconditionGoalDefinitions = Some(list).into();
        assert!(from_some.is_empty());
    }

    #[test]
    fn pre_gds_from_option_list_none() {
        let from_none: PreconditionGoalDefinitions =
            Option::<PreconditionGoalDefinitions>::None.into();
        assert!(from_none.is_empty());
    }

    #[test]
    fn pre_gds_deref() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDefinition::preference(pref);
        let list = PreconditionGoalDefinitions::new(vec![single.clone()]);
        let slice: &[PreconditionGoalDefinition] = &list;
        assert_eq!(slice.len(), 1);
        assert_eq!(&slice[0], &single);
    }

    #[test]
    fn pre_gds_as_ref() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDefinition::preference(pref);
        let list = PreconditionGoalDefinitions::new(vec![single]);
        let slice: &[PreconditionGoalDefinition] = list.as_ref();
        assert_eq!(slice.len(), 1);
    }

    #[test]
    fn pre_gds_into_iter() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDefinition::preference(pref.clone());
        let list = PreconditionGoalDefinitions::new(vec![single.clone()]);
        let collected: Vec<_> = list.into_iter().collect();
        assert_eq!(collected, vec![single]);
    }

    #[test]
    fn pre_gds_into_vec() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDefinition::preference(pref.clone());
        let list = PreconditionGoalDefinitions::new(vec![single.clone()]);
        let vec: Vec<PreconditionGoalDefinition> = list.into();
        assert_eq!(vec, vec![single]);
    }

    #[test]
    fn pre_gds_try_get_single() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDefinition::preference(pref.clone());
        let list = PreconditionGoalDefinitions::new(vec![single.clone()]);
        assert_eq!(list.try_get_single(), Some(single.clone()));

        let empty = PreconditionGoalDefinitions::default();
        assert!(empty.try_get_single().is_none());

        let multi = PreconditionGoalDefinitions::new(vec![single.clone(), single]);
        assert!(multi.try_get_single().is_none());
    }

    #[test]
    fn pre_gds_try_into() {
        let pref = make_preference_goal();
        let single = PreconditionGoalDefinition::preference(pref.clone());
        let list = PreconditionGoalDefinitions::new(vec![single.clone()]);
        let result: Result<PreconditionGoalDefinition, ()> = list.try_into();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), single);
    }

    #[test]
    fn pre_gd_and() {
        let pref = make_preference_goal();
        let items = vec![
            PreconditionGoalDefinition::preference(pref.clone()),
            PreconditionGoalDefinition::preference(pref),
        ];
        let result: PreconditionGoalDefinitions = PreconditionGoalDefinition::and(items);
        assert_eq!(result.len(), 2);
    }

    #[test]
    #[allow(deprecated)]
    fn pre_gd_and_deprecated() {
        let pref = make_preference_goal();
        let _ =
            PreconditionGoalDefinition::new_and(vec![PreconditionGoalDefinition::preference(pref)]);
    }

    #[test]
    fn pre_gd_forall() {
        let vars = TypedVariables::default();
        let gds = PreconditionGoalDefinitions::default();
        let forall = PreconditionGoalDefinition::r#forall(vars.clone(), gds.clone());
        assert!(matches!(forall, PreconditionGoalDefinition::Forall(_, _)));
    }
}
