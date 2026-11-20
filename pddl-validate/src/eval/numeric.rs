//! Numeric expressions and comparisons.
//!
//! Values are `f64`, as the `pddl` parser stores numbers. A fluent without a value is
//! undefined: reading it is an [`EvalError::Undefined`], whatever the surrounding
//! condition, so `(or (p) (> (f) 0))` and `(or (> (f) 0) (p))` fail alike. So does
//! arithmetic that divides by zero or leaves the finite range.
//!
//! Comparisons use a relative tolerance `t` (see [`Validator::with_tolerance`]): `a` and
//! `b` are equal if `|a - b| <= t * max(1, |a|, |b|)`, and otherwise ordered as numbers.
//! Exactly one of `a < b`, `a = b` and `a > b` holds.
//!
//! [`Validator::with_tolerance`]: crate::Validator::with_tolerance

use alloc::string::String;
use core::cmp::Ordering;

use pddl::{
    BinaryComparison, BinaryOp, FluentComparison, FluentExpression, FunctionHead, FunctionTerm,
    GoalDescription, MetricFluentExpression, MultiOp, PrimitiveEffect, Term,
};

use super::{EvalError, Evaluator, Scope, Verdict};
use crate::atom::{lower, Atom};
use crate::render;
use crate::report::{Construct, Unmet};

impl Evaluator<'_, '_> {
    /// Checks a numeric comparison against `want`.
    pub(super) fn comparison(
        &self,
        gd: &GoalDescription,
        comparison: &FluentComparison,
        scope: &Scope,
        want: bool,
    ) -> Verdict {
        let left = self.value(comparison.first(), scope)?;
        let right = self.value(comparison.second(), scope)?;
        let order = compare(left, right, self.tolerance);
        let holds = match comparison.comparison() {
            BinaryComparison::GreaterThan => order == Ordering::Greater,
            BinaryComparison::LessThan => order == Ordering::Less,
            BinaryComparison::Equal => order == Ordering::Equal,
            BinaryComparison::GreaterOrEqual => order != Ordering::Less,
            BinaryComparison::LessThanOrEqual => order != Ordering::Greater,
        };

        Ok((holds != want).then(|| Unmet::Comparison {
            condition: render::goal(gd, scope),
            left,
            right,
            expected: want,
        }))
    }

    /// Evaluates a numeric expression.
    ///
    /// # Errors
    ///
    /// [`EvalError::Undefined`] for a fluent without a value, [`EvalError::DivisionByZero`]
    /// and [`EvalError::NonFinite`] for arithmetic without a finite result.
    pub fn value(&self, exp: &FluentExpression, scope: &Scope) -> Result<f64, EvalError> {
        let value = match exp {
            FluentExpression::Number(number) => **number,
            FluentExpression::Function(head) => return self.fluent(&fluent_atom(head, scope)),
            FluentExpression::Negative(inner) => -self.value(inner, scope)?,
            FluentExpression::BinaryOp(op, left, right) => {
                let left = self.value(left, scope)?;
                let right = self.value(right, scope)?;
                binary(*op, left, right).ok_or_else(|| division(exp, scope))?
            }
            FluentExpression::MultiOp(op, first, rest) => {
                let mut total = self.value(first, scope)?;
                for item in rest {
                    total = multi(*op, total, self.value(item, scope)?);
                }

                total
            }
        };

        finite(value, || render::expression(exp, scope))
    }

    /// Returns the value of a ground fluent.
    ///
    /// # Errors
    ///
    /// [`EvalError::Undefined`] if it has none.
    pub fn fluent(&self, atom: &Atom) -> Result<f64, EvalError> {
        self.state
            .fluents
            .get(atom)
            .copied()
            .ok_or_else(|| EvalError::Undefined(atom.clone()))
    }

    /// Evaluates a metric expression in the final state of a plan of `steps` steps.
    /// `total-time` is the number of steps, the makespan of a sequential plan whose
    /// steps take one time unit each.
    ///
    /// # Errors
    ///
    /// As [`Self::value`], and [`EvalError::Unsupported`] for `is-violated`.
    pub fn metric(&self, exp: &MetricFluentExpression, steps: usize) -> Result<f64, EvalError> {
        let value = match exp {
            MetricFluentExpression::Number(number) => **number,
            MetricFluentExpression::Function(symbol, names)
                if self.model.is_total_time(symbol, names.len()) =>
            {
                total_time(steps)
            }
            MetricFluentExpression::Function(symbol, names) => {
                let arguments = names.iter().map(|name| lower(name)).collect();
                return self.fluent(&Atom::lowered(lower(symbol), arguments));
            }
            MetricFluentExpression::Negative(inner) => -self.metric(inner, steps)?,
            MetricFluentExpression::BinaryOp(op, left, right) => {
                let left = self.metric(left, steps)?;
                let right = self.metric(right, steps)?;
                binary(*op, left, right)
                    .ok_or_else(|| EvalError::DivisionByZero(String::from("metric")))?
            }
            MetricFluentExpression::MultiOp(op, first, rest) => {
                let mut total = self.metric(first, steps)?;
                for item in rest {
                    total = multi(*op, total, self.metric(item, steps)?);
                }

                total
            }
            MetricFluentExpression::TotalTime => total_time(steps),
            MetricFluentExpression::IsViolated(_) => {
                return Err(EvalError::Unsupported(Construct::Preferences));
            }
        };

        finite(value, || String::from("metric"))
    }
}

/// Returns the `total-time` of a sequential plan of `steps` steps.
#[expect(
    clippy::cast_precision_loss,
    reason = "plans of more than 2^53 steps do not fit in memory"
)]
const fn total_time(steps: usize) -> f64 {
    steps as f64
}

/// Orders `left` and `right`, treating them as equal within the relative `tolerance`.
///
/// Both values are finite: evaluation rejects every other value.
pub fn compare(left: f64, right: f64, tolerance: f64) -> Ordering {
    let scale = 1.0_f64.max(abs(left)).max(abs(right));
    if abs(left - right) <= tolerance * scale {
        Ordering::Equal
    } else if left < right {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

/// Returns `|value|`. `f64::abs` needs `std` on this crate's MSRV.
fn abs(value: f64) -> f64 {
    if value < 0.0 {
        -value
    } else {
        value
    }
}

/// Applies a binary operator; `None` for a division by zero.
fn binary(op: BinaryOp, left: f64, right: f64) -> Option<f64> {
    match op {
        BinaryOp::Multiplication => Some(left * right),
        BinaryOp::Addition => Some(left + right),
        BinaryOp::Subtraction => Some(left - right),
        BinaryOp::Division if right == 0.0 => None,
        BinaryOp::Division => Some(left / right),
    }
}

/// Applies an n-ary operator to an accumulated value and the next operand.
fn multi(op: MultiOp, total: f64, next: f64) -> f64 {
    match op {
        MultiOp::Addition => total + next,
        MultiOp::Multiplication => total * next,
    }
}

/// Returns `value` if it is finite, and else an [`EvalError::NonFinite`] naming the
/// expression that `text` renders.
fn finite(value: f64, text: impl FnOnce() -> String) -> Result<f64, EvalError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(EvalError::NonFinite(text()))
    }
}

/// Builds the error for a division by zero in `exp`.
fn division(exp: &FluentExpression, scope: &Scope) -> EvalError {
    EvalError::DivisionByZero(render::expression(exp, scope))
}

/// Reads `(assign (f ...) (g ...))` as the numeric assignment it is.
///
/// The parser cannot tell `(g ...)` from an object-valued function term without the
/// domain's declarations, so it produces [`PrimitiveEffect::AssignObjectFluent`]. The
/// validator accepts only number-valued functions, so such an effect always assigns
/// the number `(g ...)` to `(f ...)`. Returns the target and value, or `None` for any
/// other effect, such as `(assign (f) undefined)` or an object on the right.
pub fn numeric_assignment(effect: &PrimitiveEffect) -> Option<(FunctionHead, FluentExpression)> {
    let PrimitiveEffect::AssignObjectFluent(target, Some(Term::Function(source))) = effect else {
        return None;
    };

    let head =
        |term: &FunctionTerm| FunctionHead::WithTerms(term.symbol().clone(), term.terms().to_vec());

    Some((head(target), FluentExpression::Function(head(source))))
}

/// Grounds a function application in `scope`, such as `(fuel ?t)` to `(fuel t1)`.
pub fn fluent_atom(head: &FunctionHead, scope: &Scope) -> Atom {
    match head {
        FunctionHead::Simple(symbol) => Atom::lowered(lower(symbol), alloc::vec::Vec::new()),
        FunctionHead::WithTerms(symbol, terms) => {
            let arguments = terms.iter().map(|t| super::resolve(t, scope)).collect();
            Atom::lowered(lower(symbol), arguments)
        }
    }
}
