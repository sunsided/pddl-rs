//! Contains the type [`PrefConGD`].

use crate::types::{ConGD, PreferenceName, TypedVariables};

#[derive(Debug, Clone, PartialEq)]
pub enum PrefConGD<'a> {
    And(Vec<PrefConGD<'a>>),
    /// Requires [UniversalPreconditions](crate::types::Requirement::UniversalPreconditions).
    Forall(TypedVariables<'a>, Box<PrefConGD<'a>>),
    /// Requires [Preferences](crate::types::Requirement::Preferences).
    Preference(Option<PreferenceName<'a>>, ConGD<'a>),
    Goal(ConGD<'a>),
}

impl<'a> PrefConGD<'a> {
    pub fn new_and<I: IntoIterator<Item = PrefConGD<'a>>>(iter: I) -> Self {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        Self::And(iter.into_iter().collect())
    }

    pub fn new_forall(variables: TypedVariables<'a>, gd: PrefConGD<'a>) -> Self {
        Self::Forall(variables, Box::new(gd))
    }

    pub const fn new_preference(name: Option<PreferenceName<'a>>, gd: ConGD<'a>) -> Self {
        Self::Preference(name, gd)
    }

    pub const fn new_goal(gd: ConGD<'a>) -> Self {
        Self::Goal(gd)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            PrefConGD::And(x) => x.iter().all(|y| y.is_empty()),
            PrefConGD::Forall(_, x) => x.is_empty(),
            PrefConGD::Preference(_, x) => x.is_empty(),
            PrefConGD::Goal(x) => x.is_empty(),
        }
    }
}
