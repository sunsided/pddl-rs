//! Contains atomic formula skeletons via the [`AtomicFormulaSkeleton`] type.

use crate::types::Predicate;
use crate::types::{Name, TypedVariables};

/// An atomic formula skeleton.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AtomicFormulaSkeleton<'a> {
    predicate: Predicate<'a>,
    variables: TypedVariables<'a>,
}

impl<'a> AtomicFormulaSkeleton<'a> {
    pub const fn new(predicate: Predicate<'a>, formula: TypedVariables<'a>) -> Self {
        Self {
            predicate,
            variables: formula,
        }
    }

    pub fn name(&self) -> &Name<'a> {
        self.predicate.as_ref()
    }

    /// Gets a reference to the predicate.
    pub const fn predicate(&self) -> &Predicate<'a> {
        &self.predicate
    }

    /// Gets a reference to the variables.
    pub fn variables(&self) -> &TypedVariables<'a> {
        &self.variables
    }
}

impl<'a> From<(Predicate<'a>, TypedVariables<'a>)> for AtomicFormulaSkeleton<'a> {
    fn from(value: (Predicate<'a>, TypedVariables<'a>)) -> Self {
        AtomicFormulaSkeleton::new(value.0, value.1)
    }
}
