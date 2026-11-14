//! Contains derived predicates via the [`DerivedPredicate`] type.

use crate::types::{AtomicFormulaSkeleton, GoalDefinition};

/// A derived predicate.
///
/// ## Usage
/// Used by [`StructureDef`](crate::StructureDef).
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedPredicate(AtomicFormulaSkeleton, GoalDefinition);

impl DerivedPredicate {
    /// Creates a new derived predicate, i.e. `(:derived <atomic formula skeleton> <GD>)`,
    /// from the predicate and the goal definition that derives it.
    pub const fn new(formula: AtomicFormulaSkeleton, gd: GoalDefinition) -> Self {
        Self(formula, gd)
    }

    /// Gets the derived predicate's atomic formula skeleton, i.e. its name and parameters.
    pub const fn predicate(&self) -> &AtomicFormulaSkeleton {
        &self.0
    }

    /// Gets the goal definition under which the predicate holds.
    pub const fn expression(&self) -> &GoalDefinition {
        &self.1
    }
}

impl From<(AtomicFormulaSkeleton, GoalDefinition)> for DerivedPredicate {
    fn from(value: (AtomicFormulaSkeleton, GoalDefinition)) -> Self {
        DerivedPredicate::new(value.0, value.1)
    }
}

impl AsRef<AtomicFormulaSkeleton> for DerivedPredicate {
    fn as_ref(&self) -> &AtomicFormulaSkeleton {
        self.predicate()
    }
}

impl AsRef<GoalDefinition> for DerivedPredicate {
    fn as_ref(&self) -> &GoalDefinition {
        self.expression()
    }
}
