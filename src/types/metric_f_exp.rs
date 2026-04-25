//! Contains the [`MetricFluentExpression`] type.

use crate::types::{BinaryOp, FunctionSymbol, MultiOp, Name, Number, PreferenceName};

/// A metric function expression.
///
/// ## Requirements
/// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
///
/// ## Usage
/// Used by [`MetricSpec`](crate::MetricSpec).
#[derive(Debug, Clone, PartialEq)]
pub enum MetricFluentExpression {
    BinaryOp(BinaryOp, Box<Self>, Box<Self>),
    MultiOp(MultiOp, Box<Self>, Vec<Self>),
    Negative(Box<Self>),
    Number(Number),
    Function(FunctionSymbol, Vec<Name>),
    TotalTime,
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    IsViolated(PreferenceName),
}

impl MetricFluentExpression {
    #[doc(alias = "new_binary_op")]
    pub fn binary_op(op: BinaryOp, lhs: Self, rhs: Self) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_binary_op(op: BinaryOp, lhs: Self, rhs: Self) -> Self {
        Self::binary_op(op, lhs, rhs)
    }

    #[doc(alias = "new_multi_op")]
    pub fn multi_op<I: IntoIterator<Item = Self>>(op: MultiOp, lhs: Self, rhs: I) -> Self {
        let vec: Vec<_> = rhs.into_iter().collect();
        debug_assert!(
            !vec.is_empty(),
            "Right-hand side requires at least one operand"
        );
        Self::MultiOp(op, Box::new(lhs), vec)
    }

    pub fn new_multi_op<I: IntoIterator<Item = Self>>(op: MultiOp, lhs: Self, rhs: I) -> Self {
        Self::multi_op(op, lhs, rhs)
    }

    #[doc(alias = "new_negative")]
    pub fn negative(exp: Self) -> Self {
        Self::Negative(Box::new(exp))
    }

    pub fn new_negative(exp: Self) -> Self {
        Self::negative(exp)
    }

    #[doc(alias = "new_number")]
    pub fn number<N: Into<Number>>(number: N) -> Self {
        Self::Number(number.into())
    }

    pub fn new_number<N: Into<Number>>(number: N) -> Self {
        Self::number(number)
    }

    #[doc(alias = "new_function")]
    pub fn function<I: IntoIterator<Item = Name>>(symbol: FunctionSymbol, names: I) -> Self {
        Self::Function(symbol, names.into_iter().collect())
    }

    pub fn new_function<I: IntoIterator<Item = Name>>(symbol: FunctionSymbol, names: I) -> Self {
        Self::function(symbol, names)
    }

    #[doc(alias = "new_total_time")]
    pub const fn total_time() -> Self {
        Self::TotalTime
    }

    pub fn new_total_time() -> Self {
        Self::total_time()
    }

    #[doc(alias = "new_is_violated")]
    pub const fn is_violated(pref: PreferenceName) -> Self {
        Self::IsViolated(pref)
    }

    pub fn new_is_violated(pref: PreferenceName) -> Self {
        Self::is_violated(pref)
    }
}

/// Alias for [`MetricFluentExpression`]; matches BNF `<metric-f-exp>`.
#[deprecated(since = "0.2.0", note = "Use `MetricFluentExpression` instead")]
#[allow(dead_code)]
pub type MetricFExp = MetricFluentExpression;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn deprecated_new_constructors() {
        let _ = MetricFluentExpression::new_number(42);
        let _ = MetricFluentExpression::new_total_time();

        let pref = PreferenceName::new("p");
        let _ = MetricFluentExpression::new_is_violated(pref);

        let sym = FunctionSymbol::new(Name::new("cost"));
        let _ = MetricFluentExpression::new_function(sym, vec![Name::new("x")]);

        let lhs = MetricFluentExpression::number(1);
        let rhs = MetricFluentExpression::number(2);
        let _ = MetricFluentExpression::new_binary_op(BinaryOp::Addition, lhs.clone(), rhs.clone());
        let _ =
            MetricFluentExpression::new_multi_op(MultiOp::Addition, lhs.clone(), vec![rhs.clone()]);
        let _ = MetricFluentExpression::new_negative(lhs);
    }

    #[test]
    fn binary_op_constructor() {
        let lhs = MetricFluentExpression::number(1);
        let rhs = MetricFluentExpression::number(2);
        let expr = MetricFluentExpression::binary_op(BinaryOp::Addition, lhs, rhs);
        assert!(matches!(expr, MetricFluentExpression::BinaryOp(_, _, _)));
    }

    #[test]
    fn multi_op_constructor() {
        let lhs = MetricFluentExpression::number(1);
        let rhs = vec![
            MetricFluentExpression::number(2),
            MetricFluentExpression::number(3),
        ];
        let expr = MetricFluentExpression::multi_op(MultiOp::Addition, lhs, rhs);
        assert!(matches!(expr, MetricFluentExpression::MultiOp(_, _, _)));
    }

    #[test]
    fn negative_constructor() {
        let inner = MetricFluentExpression::number(5);
        let expr = MetricFluentExpression::negative(inner);
        assert!(matches!(expr, MetricFluentExpression::Negative(_)));
    }

    #[test]
    fn number_constructor() {
        let expr = MetricFluentExpression::number(42);
        assert!(matches!(expr, MetricFluentExpression::Number(_)));
    }

    #[test]
    fn function_constructor() {
        let sym = FunctionSymbol::new(Name::new("cost"));
        let expr = MetricFluentExpression::function(sym, vec![Name::new("x")]);
        assert!(matches!(expr, MetricFluentExpression::Function(_, _)));
    }

    #[test]
    fn total_time_constructor() {
        let expr = MetricFluentExpression::total_time();
        assert!(matches!(expr, MetricFluentExpression::TotalTime));
    }

    #[test]
    fn is_violated_constructor() {
        let pref = PreferenceName::new("p");
        let expr = MetricFluentExpression::is_violated(pref);
        assert!(matches!(expr, MetricFluentExpression::IsViolated(_)));
    }
}
