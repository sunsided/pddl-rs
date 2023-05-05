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
pub struct PrefConGDs(Vec<PrefConGD>);

impl PrefConGDs {
    /// Constructs a new instance from the provided vector of values.
    pub const fn new(gds: Vec<PrefConGD>) -> Self {
        Self(gds)
    }

    /// Constructs a list containing a single [`PrefConGD::Goal`] variant.
    pub fn new_goal(gd: ConGD) -> Self {
        Self::new(vec![PrefConGD::new_goal(gd)])
    }

    /// Constructs a list containing a single [`PrefConGD::Preference`] variant.
    ///
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    pub fn new_preference(name: Option<PreferenceName>, gd: ConGD) -> Self {
        Self::new(vec![PrefConGD::new_preference(name, gd)])
    }

    /// Constructs a list containing a single [`PrefConGD::Forall`] variant.
    ///
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    pub fn new_forall(variables: TypedVariables, gd: PrefConGDs) -> Self {
        Self::new(vec![PrefConGD::new_forall(variables, gd)])
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
    pub fn iter(&self) -> std::slice::Iter<PrefConGD> {
        self.0.iter()
    }

    /// Get the only element of this list if the list has
    /// exactly one element. Returns [`None`] in all other cases.
    pub fn try_get_single(self) -> Option<PrefConGD> {
        if self.len() == 1 {
            self.into_iter().next()
        } else {
            None
        }
    }
}

impl Deref for PrefConGDs {
    type Target = [PrefConGD];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl AsRef<[PrefConGD]> for PrefConGDs {
    fn as_ref(&self) -> &[PrefConGD] {
        self.0.as_slice()
    }
}

impl IntoIterator for PrefConGDs {
    type Item = PrefConGD;
    type IntoIter = std::vec::IntoIter<PrefConGD>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<PrefConGDs> for Vec<PrefConGD> {
    fn from(value: PrefConGDs) -> Self {
        value.0
    }
}

impl From<PrefConGD> for PrefConGDs {
    fn from(value: PrefConGD) -> Self {
        PrefConGDs::new(vec![value])
    }
}

impl From<Vec<PrefConGD>> for PrefConGDs {
    fn from(value: Vec<PrefConGD>) -> Self {
        PrefConGDs::new(value)
    }
}

impl FromIterator<PrefConGD> for PrefConGDs {
    fn from_iter<T: IntoIterator<Item = PrefConGD>>(iter: T) -> Self {
        PrefConGDs::new(iter.into_iter().collect())
    }
}

impl From<Option<PrefConGD>> for PrefConGDs {
    fn from(value: Option<PrefConGD>) -> Self {
        match value {
            None => PrefConGDs::default(),
            Some(value) => value.into(),
        }
    }
}

impl From<Option<PrefConGDs>> for PrefConGDs {
    fn from(value: Option<PrefConGDs>) -> Self {
        match value {
            None => PrefConGDs::default(),
            Some(values) => values,
        }
    }
}

impl From<ConGD> for PrefConGDs {
    fn from(value: ConGD) -> Self {
        PrefConGDs::new_goal(value)
    }
}

impl TryInto<PrefConGD> for PrefConGDs {
    type Error = ();

    fn try_into(self) -> Result<PrefConGD, Self::Error> {
        self.try_get_single().ok_or(())
    }
}

/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
///
/// ## Usage
/// Used by [`PrefConGD`] itself, as well as [`ProblemConstraintsDef`](crate::ProblemConstraintsDef).
#[derive(Debug, Clone, PartialEq)]
pub enum PrefConGD {
    Goal(ConGD),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables, PrefConGDs),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Option<PreferenceName>, ConGD),
}

impl PrefConGD {
    pub const fn new_goal(gd: ConGD) -> Self {
        Self::Goal(gd)
    }

    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    pub fn new_forall(variables: TypedVariables, gd: PrefConGDs) -> Self {
        Self::Forall(variables, gd)
    }

    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    pub const fn new_preference(name: Option<PreferenceName>, gd: ConGD) -> Self {
        Self::Preference(name, gd)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            PrefConGD::Forall(_, x) => x.is_empty(),
            PrefConGD::Preference(_, x) => x.is_empty(),
            PrefConGD::Goal(x) => x.is_empty(),
        }
    }
}
