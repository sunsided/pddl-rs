//! Contains atomic formula skeletons via the [`AtomicFormulaSkeleton`] type.

use crate::types::Predicate;
use crate::types::{Name, TypedVariables};

/// An atomic formula skeleton.
///
/// ## Usage
/// Used by [`PredicateDefinitions`](crate::PredicateDefinitions) and [`DerivedPredicate`](crate::DerivedPredicate).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AtomicFormulaSkeleton {
    predicate: Predicate,
    variables: TypedVariables,
}

impl AtomicFormulaSkeleton {
    pub const fn new(predicate: Predicate, formula: TypedVariables) -> Self {
        Self {
            predicate,
            variables: formula,
        }
    }

    pub fn name(&self) -> &Name {
        self.predicate.as_ref()
    }

    /// Gets a reference to the predicate.
    pub const fn predicate(&self) -> &Predicate {
        &self.predicate
    }

    /// Gets a reference to the variables.
    pub fn variables(&self) -> &TypedVariables {
        &self.variables
    }
}

impl From<(Predicate, TypedVariables)> for AtomicFormulaSkeleton {
    fn from(value: (Predicate, TypedVariables)) -> Self {
        AtomicFormulaSkeleton::new(value.0, value.1)
    }
}
