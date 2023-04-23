//! Contains the [`MetricFExp`] type.

use crate::types::{BinaryOp, FunctionSymbol, MultiOp, Name, Number, PreferenceName};

/// A metric function expression.
#[derive(Debug, Clone, PartialEq)]
pub enum MetricFExp<'a> {
    BinaryOp(BinaryOp, Box<Self>, Box<Self>),
    MultiOp(MultiOp, Box<Self>, Vec<Self>),
    Negative(Box<Self>),
    Number(Number),
    Function(FunctionSymbol<'a>, Vec<Name<'a>>),
    TotalTime,
    /// Requires [Preferences](crate::types::Requirement::Preferences).
    IsViolated(PreferenceName<'a>),
}

impl<'a> MetricFExp<'a> {
    pub fn new_binary_op(op: BinaryOp, lhs: Self, rhs: Self) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_multi_op<I: IntoIterator<Item = Self>>(op: MultiOp, lhs: Self, rhs: I) -> Self {
        let vec: Vec<_> = rhs.into_iter().collect();
        debug_assert!(
            !vec.is_empty(),
            "Right-hand side requires at least one operand"
        );
        Self::MultiOp(op, Box::new(lhs), vec)
    }

    pub fn new_negative(exp: Self) -> Self {
        Self::Negative(Box::new(exp))
    }

    pub fn new_number<N: Into<Number>>(number: N) -> Self {
        Self::Number(number.into())
    }

    pub fn new_function<I: IntoIterator<Item = Name<'a>>>(
        symbol: FunctionSymbol<'a>,
        names: I,
    ) -> Self {
        Self::Function(symbol, names.into_iter().collect())
    }

    pub const fn new_total_time() -> Self {
        Self::TotalTime
    }

    pub const fn new_is_violated(pref: PreferenceName<'a>) -> Self {
        Self::IsViolated(pref)
    }
}
