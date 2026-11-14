//! Contains the [`MetricSpec`] type.

use crate::types::{MetricFluentExpression, Optimization};

/// A metric specification.
///
/// ## Requirements
/// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
#[derive(Debug, Clone, PartialEq)]
pub struct MetricSpec {
    /// The optimization direction, `minimize` or `maximize`.
    optimization: Optimization,
    /// The metric expression to optimize.
    exp: MetricFluentExpression,
}

impl MetricSpec {
    /// Creates a new metric specification, `(:metric <optimization> <metric-f-exp>)`.
    #[must_use]
    pub const fn new(optimization: Optimization, exp: MetricFluentExpression) -> Self {
        Self { optimization, exp }
    }

    /// Gets the optimization instruction.
    #[must_use]
    pub const fn optimization(&self) -> Optimization {
        self.optimization
    }

    /// Gets the expression to optimize.
    #[must_use]
    pub const fn expression(&self) -> &MetricFluentExpression {
        &self.exp
    }
}
