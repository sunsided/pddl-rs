//! Contains derived predicates via the [`DerivedPredicate`] type.

use crate::types::{AtomicFormulaSkeleton, GoalDescription};

/// A derived predicate.
///
/// ## Usage
/// Used by [`StructureDef`](crate::StructureDef).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DerivedPredicate(AtomicFormulaSkeleton, GoalDescription);

impl DerivedPredicate {
    /// Creates a new derived predicate, i.e. `(:derived <atomic formula skeleton> <GD>)`,
    /// from the predicate and the goal description that derives it.
    #[must_use]
    pub const fn new(formula: AtomicFormulaSkeleton, gd: GoalDescription) -> Self {
        Self(formula, gd)
    }

    /// Gets the derived predicate's atomic formula skeleton, i.e. its name and parameters.
    #[must_use]
    pub const fn predicate(&self) -> &AtomicFormulaSkeleton {
        &self.0
    }

    /// Gets the goal description under which the predicate holds.
    #[must_use]
    pub const fn expression(&self) -> &GoalDescription {
        &self.1
    }
}

impl From<(AtomicFormulaSkeleton, GoalDescription)> for DerivedPredicate {
    fn from(value: (AtomicFormulaSkeleton, GoalDescription)) -> Self {
        DerivedPredicate::new(value.0, value.1)
    }
}

impl AsRef<AtomicFormulaSkeleton> for DerivedPredicate {
    fn as_ref(&self) -> &AtomicFormulaSkeleton {
        self.predicate()
    }
}

impl AsRef<GoalDescription> for DerivedPredicate {
    fn as_ref(&self) -> &GoalDescription {
        self.expression()
    }
}
