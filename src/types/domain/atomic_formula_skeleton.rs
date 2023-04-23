//! Contains atomic formula skeletons.

use crate::types::domain::{Name, Predicate, TypedList, Variable};

/// An atomic formula skeleton.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AtomicFormulaSkeleton<'a> {
    predicate: Predicate<'a>,
    variables: TypedList<'a, Variable<'a>>,
}

impl<'a> AtomicFormulaSkeleton<'a> {
    pub const fn new(predicate: Predicate<'a>, formula: TypedList<'a, Variable<'a>>) -> Self {
        Self {
            predicate,
            variables: formula,
        }
    }

    pub fn name(&self) -> &Name<'a> {
        self.predicate.as_ref()
    }

    /// Gets a reference to the predicate.
    pub const fn predicate_ref(&self) -> &Predicate<'a> {
        &self.predicate
    }

    /// Gets a reference to the variables.
    pub fn variables_ref(&self) -> &TypedList<'a, Variable<'a>> {
        &self.variables
    }
}

impl<'a> From<(Predicate<'a>, TypedList<'a, Variable<'a>>)> for AtomicFormulaSkeleton<'a> {
    fn from(value: (Predicate<'a>, TypedList<'a, Variable<'a>>)) -> Self {
        AtomicFormulaSkeleton::new(value.0, value.1)
    }
}
