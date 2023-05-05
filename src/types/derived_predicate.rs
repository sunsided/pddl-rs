//! Contains derived predicates via the [`DerivedPredicate`] type.

use crate::types::{AtomicFormulaSkeleton, GoalDefinition};

/// A derived predicate.
///
/// ## Usage
/// Used by [`StructureDef`](crate::StructureDef).
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedPredicate(AtomicFormulaSkeleton, GoalDefinition);

impl DerivedPredicate {
    pub const fn new(formula: AtomicFormulaSkeleton, gd: GoalDefinition) -> Self {
        Self(formula, gd)
    }

    pub const fn predicate(&self) -> &AtomicFormulaSkeleton {
        &self.0
    }

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
