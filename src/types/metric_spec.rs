//! Contains the [`MetricSpec`] type.

use crate::types::{MetricFExp, Optimization};

/// A metric specification.
///
/// ## Requirements
/// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
#[derive(Debug, Clone, PartialEq)]
pub struct MetricSpec {
    optimization: Optimization,
    exp: MetricFExp,
}

impl MetricSpec {
    pub const fn new(optimization: Optimization, exp: MetricFExp) -> Self {
        Self { optimization, exp }
    }

    /// Gets the optimization instruction.
    pub const fn optimization(&self) -> Optimization {
        self.optimization
    }

    /// Gets the expression to optimize.
    pub const fn expression(&self) -> &MetricFExp {
        &self.exp
    }
}
