//! Contains the type [`PrefConGD`].

use crate::types::{ConGD, PreferenceName, TypedVariables};
use std::ops::Deref;

/// A list of [`PrefConGD`] values. This represents the `(and ...)` variant
/// of the PDDL definition, modeling cases of zero, one or many values.
///
/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
///
/// ## Usage
/// Used by [`ProblemConstraintsDef`](crate::ProblemConstraintsDef).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PrefConGDs<'a>(Vec<PrefConGD<'a>>);

impl<'a> PrefConGDs<'a> {
    /// Constructs a new instance from the provided vector of values.
    pub const fn new(gds: Vec<PrefConGD<'a>>) -> Self {
        Self(gds)
    }

    /// Constructs a list containing a single [`PrefConGD::Preference`] variant.
    pub fn new_preference(name: Option<PreferenceName<'a>>, gd: ConGD<'a>) -> Self {
        Self::new(vec![PrefConGD::new_preference(name, gd)])
    }

    /// Constructs a list containing a single [`PrefConGD::Forall`] variant.
    pub fn new_forall(variables: TypedVariables<'a>, gd: PrefConGDs<'a>) -> Self {
        Self::new(vec![PrefConGD::new_forall(variables, gd)])
    }

    /// Constructs a list containing a single [`PrefConGD::Goal`] variant.
    pub fn new_goal(gd: ConGD<'a>) -> Self {
        Self::new(vec![PrefConGD::new_goal(gd)])
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
    pub fn iter(&'a self) -> std::slice::Iter<'a, PrefConGD<'a>> {
        self.0.iter()
    }

    /// Get the only element of this list if the list has
    /// exactly one element. Returns [`None`] in all other cases.
    pub fn try_get_single(self) -> Option<PrefConGD<'a>> {
        if self.len() == 1 {
            self.into_iter().next()
        } else {
            None
        }
    }
}

impl<'a> Deref for PrefConGDs<'a> {
    type Target = [PrefConGD<'a>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a> IntoIterator for PrefConGDs<'a> {
    type Item = PrefConGD<'a>;
    type IntoIter = std::vec::IntoIter<PrefConGD<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> From<PrefConGDs<'a>> for Vec<PrefConGD<'a>> {
    fn from(value: PrefConGDs<'a>) -> Self {
        value.0
    }
}

impl<'a> From<PrefConGD<'a>> for PrefConGDs<'a> {
    fn from(value: PrefConGD<'a>) -> Self {
        PrefConGDs::new(vec![value])
    }
}

impl<'a> From<Vec<PrefConGD<'a>>> for PrefConGDs<'a> {
    fn from(value: Vec<PrefConGD<'a>>) -> Self {
        PrefConGDs::new(value)
    }
}

impl<'a> FromIterator<PrefConGD<'a>> for PrefConGDs<'a> {
    fn from_iter<T: IntoIterator<Item = PrefConGD<'a>>>(iter: T) -> Self {
        PrefConGDs::new(iter.into_iter().collect())
    }
}

impl<'a> From<Option<PrefConGD<'a>>> for PrefConGDs<'a> {
    fn from(value: Option<PrefConGD<'a>>) -> Self {
        match value {
            None => PrefConGDs::default(),
            Some(value) => value.into(),
        }
    }
}

impl<'a> From<Option<PrefConGDs<'a>>> for PrefConGDs<'a> {
    fn from(value: Option<PrefConGDs<'a>>) -> Self {
        match value {
            None => PrefConGDs::default(),
            Some(values) => values,
        }
    }
}

impl<'a> From<ConGD<'a>> for PrefConGDs<'a> {
    fn from(value: ConGD<'a>) -> Self {
        PrefConGDs::new_goal(value)
    }
}

impl<'a> TryInto<PrefConGD<'a>> for PrefConGDs<'a> {
    type Error = ();

    fn try_into(self) -> Result<PrefConGD<'a>, Self::Error> {
        self.try_get_single().ok_or(())
    }
}

/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
///
/// ## Usage
/// Used by [`PrefConGD`] itself, as well as [`ProblemConstraintsDef`](crate::ProblemConstraintsDef).
#[derive(Debug, Clone, PartialEq)]
pub enum PrefConGD<'a> {
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables<'a>, PrefConGDs<'a>),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Option<PreferenceName<'a>>, ConGD<'a>),
    Goal(ConGD<'a>),
}

impl<'a> PrefConGD<'a> {
    pub fn new_forall(variables: TypedVariables<'a>, gd: PrefConGDs<'a>) -> Self {
        Self::Forall(variables, gd)
    }

    pub const fn new_preference(name: Option<PreferenceName<'a>>, gd: ConGD<'a>) -> Self {
        Self::Preference(name, gd)
    }

    pub const fn new_goal(gd: ConGD<'a>) -> Self {
        Self::Goal(gd)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            PrefConGD::Forall(_, x) => x.is_empty(),
            PrefConGD::Preference(_, x) => x.is_empty(),
            PrefConGD::Goal(x) => x.is_empty(),
        }
    }
}
