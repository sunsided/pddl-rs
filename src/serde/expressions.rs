//! Fluent expressions, comparisons and the operators in them.

use crate::serde::{deserialize_from_pddl, formula, serialize_display, Head, Keyed};
use crate::{
    AssignOp, BinaryComparison, BinaryOp, FluentComparison, FluentExpression, FunctionHead,
    MetricFluentExpression, MultiOp, Optimization, Term,
};
use ::serde::{Serialize, Serializer};
use std::fmt::Display;

serialize_display!(AssignOp, BinaryComparison, BinaryOp, MultiOp, Optimization);

/// Serializes `{"op": operands}`, keyed by the operator's PDDL spelling.
pub(crate) fn operation<S: Serializer, T: Serialize + ?Sized>(
    serializer: S,
    op: &impl Display,
    operands: &T,
) -> Result<S::Ok, S::Error> {
    Keyed(&op.to_string(), operands).serialize(serializer)
}

/// The operands of a multi-operand expression as one list.
pub(crate) fn operands<'a, E>(first: &'a E, rest: &'a [E]) -> Vec<&'a E> {
    std::iter::once(first).chain(rest).collect()
}

impl Serialize for FluentComparison {
    /// `{">=": [a, b]}`.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        operation(
            serializer,
            self.comparison(),
            &(self.first(), self.second()),
        )
    }
}

impl Serialize for FunctionHead {
    /// `["symbol", terms...]`, also when there are no terms.
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            FunctionHead::Simple(symbol) => Head(symbol, &[] as &[Term]).serialize(serializer),
            FunctionHead::WithTerms(symbol, terms) => Head(symbol, terms).serialize(serializer),
        }
    }
}

impl Serialize for FluentExpression {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            FluentExpression::Number(number) => number.serialize(serializer),
            FluentExpression::Function(head) => head.serialize(serializer),
            FluentExpression::Negative(value) => Keyed("-", value).serialize(serializer),
            FluentExpression::BinaryOp(op, a, b) => operation(serializer, op, &(a, b)),
            FluentExpression::MultiOp(op, first, rest) => {
                operation(serializer, op, &operands(&**first, rest))
            }
        }
    }
}

impl Serialize for MetricFluentExpression {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        type M = MetricFluentExpression;
        match self {
            M::BinaryOp(op, a, b) => operation(serializer, op, &(a, b)),
            M::MultiOp(op, first, rest) => operation(serializer, op, &operands(&**first, rest)),
            M::Negative(value) => Keyed("-", value).serialize(serializer),
            M::Number(number) => number.serialize(serializer),
            M::Function(symbol, names) => Head(symbol, names).serialize(serializer),
            M::TotalTime => serializer.serialize_str("total-time"),
            M::IsViolated(name) => Keyed("is-violated", name).serialize(serializer),
        }
    }
}

deserialize_from_pddl!(
    AssignOp => formula,
    BinaryComparison => formula,
    BinaryOp => formula,
    MultiOp => formula,
    Optimization => formula,
    FluentComparison => formula,
    FunctionHead => formula,
    FluentExpression => formula,
    MetricFluentExpression => formula,
);
