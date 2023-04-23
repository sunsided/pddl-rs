//! Contains types for derived predicates.

use crate::types::domain::{AtomicFormulaSkeleton, GoalDefinition};

/// A derived predicate.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedPredicate<'a>(AtomicFormulaSkeleton<'a>, GoalDefinition<'a>);

impl<'a> DerivedPredicate<'a> {
    pub const fn new(formula: AtomicFormulaSkeleton<'a>, gd: GoalDefinition<'a>) -> Self {
        Self(formula, gd)
    }

    pub const fn predicate(&self) -> &AtomicFormulaSkeleton<'a> {
        &self.0
    }

    pub const fn expression(&self) -> &GoalDefinition<'a> {
        &self.1
    }
}

impl<'a> From<(AtomicFormulaSkeleton<'a>, GoalDefinition<'a>)> for DerivedPredicate<'a> {
    fn from(value: (AtomicFormulaSkeleton<'a>, GoalDefinition<'a>)) -> Self {
        DerivedPredicate::new(value.0, value.1)
    }
}

impl<'a> AsRef<AtomicFormulaSkeleton<'a>> for DerivedPredicate<'a> {
    fn as_ref(&self) -> &AtomicFormulaSkeleton<'a> {
        self.predicate()
    }
}

impl<'a> AsRef<GoalDefinition<'a>> for DerivedPredicate<'a> {
    fn as_ref(&self) -> &GoalDefinition<'a> {
        self.expression()
    }
}
