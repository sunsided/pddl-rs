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
    /// A binary operation, `(<binary-op> <metric-f-exp> <metric-f-exp>)`.
    BinaryOp(BinaryOp, Box<Self>, Box<Self>),
    /// An operation over two or more operands, `(<multi-op> <metric-f-exp> <metric-f-exp>+)`.
    MultiOp(MultiOp, Box<Self>, Vec<Self>),
    /// A negation, `(- <metric-f-exp>)`.
    Negative(Box<Self>),
    /// A numeric literal.
    Number(Number),
    /// A ground function, `(<function-symbol> <name>*)` or `<function-symbol>`.
    Function(FunctionSymbol, Vec<Name>),
    /// The total time of the plan, `total-time`.
    TotalTime,
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    IsViolated(PreferenceName),
}

impl MetricFluentExpression {
    /// Creates a binary operation, `(<binary-op> <metric-f-exp> <metric-f-exp>)`.
    #[doc(alias = "new_binary_op")]
    pub fn binary_op(op: BinaryOp, lhs: Self, rhs: Self) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    /// Creates a binary operation, `(<binary-op> <metric-f-exp> <metric-f-exp>)`.
    pub fn new_binary_op(op: BinaryOp, lhs: Self, rhs: Self) -> Self {
        Self::binary_op(op, lhs, rhs)
    }

    /// Creates an operation over two or more operands, `(<multi-op> <metric-f-exp> <metric-f-exp>+)`.
    ///
    /// `rhs` must contain at least one operand; this is checked in debug builds.
    #[doc(alias = "new_multi_op")]
    pub fn multi_op<I: IntoIterator<Item = Self>>(op: MultiOp, lhs: Self, rhs: I) -> Self {
        let vec: Vec<_> = rhs.into_iter().collect();
        debug_assert!(
            !vec.is_empty(),
            "Right-hand side requires at least one operand"
        );
        Self::MultiOp(op, Box::new(lhs), vec)
    }

    /// Creates an operation over two or more operands, `(<multi-op> <metric-f-exp> <metric-f-exp>+)`.
    pub fn new_multi_op<I: IntoIterator<Item = Self>>(op: MultiOp, lhs: Self, rhs: I) -> Self {
        Self::multi_op(op, lhs, rhs)
    }

    /// Creates a negation, `(- <metric-f-exp>)`.
    #[doc(alias = "new_negative")]
    pub fn negative(exp: Self) -> Self {
        Self::Negative(Box::new(exp))
    }

    /// Creates a negation, `(- <metric-f-exp>)`.
    pub fn new_negative(exp: Self) -> Self {
        Self::negative(exp)
    }

    /// Creates a numeric literal.
    #[doc(alias = "new_number")]
    pub fn number<N: Into<Number>>(number: N) -> Self {
        Self::Number(number.into())
    }

    /// Creates a numeric literal.
    pub fn new_number<N: Into<Number>>(number: N) -> Self {
        Self::number(number)
    }

    /// Creates a ground function, `(<function-symbol> <name>*)`.
    #[doc(alias = "new_function")]
    pub fn function<I: IntoIterator<Item = Name>>(symbol: FunctionSymbol, names: I) -> Self {
        Self::Function(symbol, names.into_iter().collect())
    }

    /// Creates a ground function, `(<function-symbol> <name>*)`.
    pub fn new_function<I: IntoIterator<Item = Name>>(symbol: FunctionSymbol, names: I) -> Self {
        Self::function(symbol, names)
    }

    /// Creates the `total-time` expression.
    #[doc(alias = "new_total_time")]
    pub const fn total_time() -> Self {
        Self::TotalTime
    }

    /// Creates the `total-time` expression.
    pub fn new_total_time() -> Self {
        Self::total_time()
    }

    /// Creates an `(is-violated <pref-name>)` expression.
    #[doc(alias = "new_is_violated")]
    pub const fn is_violated(pref: PreferenceName) -> Self {
        Self::IsViolated(pref)
    }

    /// Creates an `(is-violated <pref-name>)` expression.
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
    use assert2::check;

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
        check!(let MetricFluentExpression::BinaryOp(_, _, _) = expr);
    }

    #[test]
    fn multi_op_constructor() {
        let lhs = MetricFluentExpression::number(1);
        let rhs = vec![
            MetricFluentExpression::number(2),
            MetricFluentExpression::number(3),
        ];
        let expr = MetricFluentExpression::multi_op(MultiOp::Addition, lhs, rhs);
        check!(let MetricFluentExpression::MultiOp(_, _, _) = expr);
    }

    #[test]
    fn negative_constructor() {
        let inner = MetricFluentExpression::number(5);
        let expr = MetricFluentExpression::negative(inner);
        check!(let MetricFluentExpression::Negative(_) = expr);
    }

    #[test]
    fn number_constructor() {
        let expr = MetricFluentExpression::number(42);
        check!(let MetricFluentExpression::Number(_) = expr);
    }

    #[test]
    fn function_constructor() {
        let sym = FunctionSymbol::new(Name::new("cost"));
        let expr = MetricFluentExpression::function(sym, vec![Name::new("x")]);
        check!(let MetricFluentExpression::Function(_, _) = expr);
    }

    #[test]
    fn total_time_constructor() {
        let expr = MetricFluentExpression::total_time();
        check!(let MetricFluentExpression::TotalTime = expr);
    }

    #[test]
    fn is_violated_constructor() {
        let pref = PreferenceName::new("p");
        let expr = MetricFluentExpression::is_violated(pref);
        check!(let MetricFluentExpression::IsViolated(_) = expr);
    }
}
