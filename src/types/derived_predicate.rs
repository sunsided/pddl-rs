//! Contains types for derived predicates.

use crate::types::{AtomicFormulaSkeleton, GD};

/// A derived predicate.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedPredicate<'a>(AtomicFormulaSkeleton<'a>, GD<'a>);

impl<'a> DerivedPredicate<'a> {
    pub const fn new(formula: AtomicFormulaSkeleton<'a>, gd: GD<'a>) -> Self {
        Self(formula, gd)
    }

    pub const fn predicate(&self) -> &AtomicFormulaSkeleton<'a> {
        &self.0
    }

    pub const fn expression(&self) -> &GD<'a> {
        &self.1
    }
}

impl<'a> From<(AtomicFormulaSkeleton<'a>, GD<'a>)> for DerivedPredicate<'a> {
    fn from(value: (AtomicFormulaSkeleton<'a>, GD<'a>)) -> Self {
        DerivedPredicate::new(value.0, value.1)
    }
}

impl<'a> AsRef<AtomicFormulaSkeleton<'a>> for DerivedPredicate<'a> {
    fn as_ref(&self) -> &AtomicFormulaSkeleton<'a> {
        self.predicate()
    }
}

impl<'a> AsRef<GD<'a>> for DerivedPredicate<'a> {
    fn as_ref(&self) -> &GD<'a> {
        self.expression()
    }
}
