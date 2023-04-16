//! Contains atomic formula skeletons.

use crate::types::{Predicate, Typed, Variable};

/// An atomic formula skeleton.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AtomicFormulaSkeleton<'a> {
    predicate: Predicate<'a>,
    variables: Vec<Typed<'a, Variable<'a>>>,
}

impl<'a> AtomicFormulaSkeleton<'a> {
    pub const fn new(predicate: Predicate<'a>, formula: Vec<Typed<'a, Variable<'a>>>) -> Self {
        Self {
            predicate,
            variables: formula,
        }
    }

    /// Gets a reference to the predicate.
    pub const fn predicate_ref(&self) -> &Predicate<'a> {
        &self.predicate
    }

    /// Gets a reference to the variables.
    pub fn variables_ref(&self) -> &[Typed<'a, Variable<'a>>] {
        self.variables.as_slice()
    }
}

impl<'a> From<(Predicate<'a>, Vec<Typed<'a, Variable<'a>>>)> for AtomicFormulaSkeleton<'a> {
    fn from(value: (Predicate<'a>, Vec<Typed<'a, Variable<'a>>>)) -> Self {
        AtomicFormulaSkeleton::new(value.0, value.1)
    }
}
