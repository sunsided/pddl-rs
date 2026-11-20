//! Numeric expressions and comparisons.
//!
//! Values are `f64`, as the `pddl` parser stores numbers. A fluent without a value is
//! undefined: a comparison that reads it is an [`EvalError::Undefined`], and so is unknown
//! rather than false. So is one with arithmetic that divides by zero or leaves the finite
//! range. The surrounding condition decides whether an unknown part matters: see the
//! [three-valued rule](super).
//!
//! Comparisons use a relative tolerance `t` (see [`Validator::with_tolerance`]): `a` and
//! `b` are equal if `|a - b| <= t * max(1, |a|, |b|)`, and otherwise ordered as numbers.
//! Exactly one of `a < b`, `a = b` and `a > b` holds.
//!
//! [`Validator::with_tolerance`]: crate::Validator::with_tolerance

use alloc::collections::BTreeSet;
use alloc::string::String;
use core::cmp::Ordering;

use pddl::{
    BinaryComparison, BinaryOp, EqualityAtomicFormula, FluentComparison, FluentExpression,
    FunctionHead, FunctionTerm, MetricFluentExpression, MultiOp, PrimitiveEffect, Term,
};

use super::{EvalError, Evaluator, Scope, Verdict};
use crate::atom::{lower, Atom};
use crate::render;
use crate::report::{Construct, Unmet};

impl Evaluator<'_, '_> {
    /// Checks a numeric comparison against `want`.
    pub(super) fn comparison(
        &self,
        comparison: &FluentComparison,
        scope: &Scope,
        want: bool,
    ) -> Verdict {
        let left = self.value(comparison.first(), scope)?;
        let right = self.value(comparison.second(), scope)?;
        if let Some(observer) = self.observer {
            let mut observer = observer.borrow_mut();
            observer.differences.push(left - right);
            let degree = degree(comparison.first(), scope, &observer.moving).max(degree(
                comparison.second(),
                scope,
                &observer.moving,
            ));
            observer.nonlinear |= degree > 1;
        }

        let order = compare(left, right, self.tolerance);
        let holds = match comparison.comparison() {
            BinaryComparison::GreaterThan => order == Ordering::Greater,
            BinaryComparison::LessThan => order == Ordering::Less,
            BinaryComparison::Equal => order == Ordering::Equal,
            BinaryComparison::GreaterOrEqual => order != Ordering::Less,
            BinaryComparison::LessThanOrEqual => order != Ordering::Greater,
        };

        Ok((holds != want).then(|| Unmet::Comparison {
            condition: render::comparison(comparison, scope),
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
        if let Some(observer) = self.observer {
            observer.borrow_mut().fluents.insert(atom.clone());
        }

        self.state
            .fluents
            .get(atom)
            .copied()
            .ok_or_else(|| EvalError::Undefined(atom.clone()))
    }

    /// Evaluates a metric expression in the final state of a plan whose `total-time` is
    /// `total_time`: the makespan of a temporal plan, or the number of steps of a
    /// sequential one, whose steps take one time unit each.
    ///
    /// # Errors
    ///
    /// As [`Self::value`], and [`EvalError::Unsupported`] for `is-violated`.
    pub fn metric(&self, exp: &MetricFluentExpression, total_time: f64) -> Result<f64, EvalError> {
        let value = match exp {
            MetricFluentExpression::Number(number) => **number,
            MetricFluentExpression::Function(symbol, names)
                if self.model.is_total_time(symbol, names.len()) =>
            {
                total_time
            }
            MetricFluentExpression::Function(symbol, names) => {
                let arguments = names.iter().map(|name| lower(name)).collect();
                return self.fluent(&Atom::lowered(lower(symbol), arguments));
            }
            MetricFluentExpression::Negative(inner) => -self.metric(inner, total_time)?,
            MetricFluentExpression::BinaryOp(op, left, right) => {
                let left = self.metric(left, total_time)?;
                let right = self.metric(right, total_time)?;
                binary(*op, left, right)
                    .ok_or_else(|| EvalError::DivisionByZero(String::from("metric")))?
            }
            MetricFluentExpression::MultiOp(op, first, rest) => {
                let mut total = self.metric(first, total_time)?;
                for item in rest {
                    total = multi(*op, total, self.metric(item, total_time)?);
                }

                total
            }
            MetricFluentExpression::TotalTime => total_time,
            MetricFluentExpression::IsViolated(_) => {
                return Err(EvalError::Unsupported(Construct::Preferences));
            }
        };

        finite(value, || String::from("metric"))
    }
}

/// Returns the polynomial degree of `exp` in time when the fluents in `moving` change
/// linearly and all others stay constant: 0 for a constant, 1 for linear, and
/// [`u32::MAX`] for a division by a changing value.
pub fn degree(exp: &FluentExpression, scope: &Scope, moving: &BTreeSet<Atom>) -> u32 {
    match exp {
        FluentExpression::Number(_) => 0,
        FluentExpression::Function(head) => u32::from(moving.contains(&fluent_atom(head, scope))),
        FluentExpression::Negative(inner) => degree(inner, scope, moving),
        FluentExpression::BinaryOp(op, left, right) => {
            let (left, right) = (degree(left, scope, moving), degree(right, scope, moving));
            match op {
                BinaryOp::Addition | BinaryOp::Subtraction => left.max(right),
                BinaryOp::Multiplication => left.saturating_add(right),
                BinaryOp::Division if right > 0 => u32::MAX,
                BinaryOp::Division => left,
            }
        }
        FluentExpression::MultiOp(op, first, rest) => {
            let degrees = core::iter::once(&**first)
                .chain(rest)
                .map(|e| degree(e, scope, moving));
            match op {
                MultiOp::Addition => degrees.max().unwrap_or(0),
                MultiOp::Multiplication => degrees.fold(0, u32::saturating_add),
            }
        }
    }
}

/// Orders `left` and `right`, treating them as equal within the relative `tolerance`.
///
/// Both values are finite: evaluation rejects every other value.
pub fn compare(left: f64, right: f64, tolerance: f64) -> Ordering {
    let scale = 1.0_f64.max(left.abs()).max(right.abs());
    if (left - right).abs() <= tolerance * scale {
        Ordering::Equal
    } else if left < right {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

/// Applies a binary operator; `None` for a division by zero.
pub fn binary(op: BinaryOp, left: f64, right: f64) -> Option<f64> {
    match op {
        BinaryOp::Multiplication => Some(left * right),
        BinaryOp::Addition => Some(left + right),
        BinaryOp::Subtraction => Some(left - right),
        BinaryOp::Division if right == 0.0 => None,
        BinaryOp::Division => Some(left / right),
    }
}

/// Applies an n-ary operator to an accumulated value and the next operand.
pub fn multi(op: MultiOp, total: f64, next: f64) -> f64 {
    match op {
        MultiOp::Addition => total + next,
        MultiOp::Multiplication => total * next,
    }
}

/// Returns `value` if it is finite, and else an [`EvalError::NonFinite`] naming the
/// expression that `text` renders.
pub fn finite(value: f64, text: impl FnOnce() -> String) -> Result<f64, EvalError> {
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

/// Reads `(= (f ...) (g ...))` as the numeric comparison it is.
///
/// As with [`numeric_assignment`], the parser reads an equality of two function terms as
/// an equality of objects. Returns `None` unless both sides are function terms.
pub fn numeric_equality(equality: &EqualityAtomicFormula<Term>) -> Option<FluentComparison> {
    let (Term::Function(left), Term::Function(right)) = (equality.first(), equality.second())
    else {
        return None;
    };

    Some(FluentComparison::new(
        BinaryComparison::Equal,
        FluentExpression::Function(head(left)),
        FluentExpression::Function(head(right)),
    ))
}

/// Returns a function term as the head of a numeric fluent.
fn head(term: &FunctionTerm) -> FunctionHead {
    FunctionHead::WithTerms(term.symbol().clone(), term.terms().to_vec())
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
