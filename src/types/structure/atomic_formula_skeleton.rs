//! Contains atomic formula skeletons via the [`AtomicFormulaSkeleton`] type.

use crate::types::Predicate;
use crate::types::{Name, TypedVariables};

/// An atomic formula skeleton.
///
/// ## Usage
/// Used by [`PredicateDefinitions`](crate::PredicateDefinitions) and [`DerivedPredicate`](crate::DerivedPredicate).
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AtomicFormulaSkeleton {
    /// The predicate being declared.
    predicate: Predicate,
    /// The typed parameters of the predicate.
    variables: TypedVariables,
}

impl AtomicFormulaSkeleton {
    /// Creates a skeleton `(<predicate> <typed list(variable)>)` from a predicate
    /// and its typed parameters.
    #[must_use]
    pub const fn new(predicate: Predicate, formula: TypedVariables) -> Self {
        Self {
            predicate,
            variables: formula,
        }
    }

    /// Gets the name of the predicate.
    #[must_use]
    pub fn name(&self) -> &Name {
        self.predicate.as_ref()
    }

    /// Gets a reference to the predicate.
    #[must_use]
    pub const fn predicate(&self) -> &Predicate {
        &self.predicate
    }

    /// Gets a reference to the variables.
    #[must_use]
    pub fn variables(&self) -> &TypedVariables {
        &self.variables
    }
}

impl From<(Predicate, TypedVariables)> for AtomicFormulaSkeleton {
    fn from(value: (Predicate, TypedVariables)) -> Self {
        AtomicFormulaSkeleton::new(value.0, value.1)
    }
}
