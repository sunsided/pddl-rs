//! Contains the [`MetricSpec`] type.

use crate::types::{MetricFExp, Optimization};

/// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
#[derive(Debug, Clone, PartialEq)]
pub struct MetricSpec<'a> {
    optimization: Optimization,
    exp: MetricFExp<'a>,
}

impl<'a> MetricSpec<'a> {
    pub const fn new(optimization: Optimization, exp: MetricFExp<'a>) -> Self {
        Self { optimization, exp }
    }

    /// Gets the optimization instruction.
    pub const fn optimization(&self) -> Optimization {
        self.optimization
    }

    /// Gets the expression to optimize.
    pub const fn expression(&self) -> &MetricFExp<'a> {
        &self.exp
    }
}
