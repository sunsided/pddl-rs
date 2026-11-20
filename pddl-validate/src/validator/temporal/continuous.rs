//! Continuous change between happenings, and `over all` conditions under it.
//!
//! Between two happenings, a running action's `(increase (f) (* r #t))` changes `f` at
//! the constant rate `r`. A rate must not read a fluent that changes continuously
//! itself, since `f` would then grow exponentially or worse: such dynamics are
//! [`Construct::NonlinearDynamics`].
//!
//! With linear change, every comparison of an `over all` condition whose sides are
//! linear in the changing fluents is linear in time, so its truth changes at most at one
//! root per interval. The condition is checked at every root and at the midpoints
//! between consecutive roots and the interval's ends, which decides it on the whole
//! open interval, up to the comparison tolerance. A comparison that is not linear (a
//! product of two changing fluents, a division by one) is unsupported.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::format;
use alloc::vec::Vec;
use core::cell::RefCell;

use pddl::{
    DurativeActionEffect, DurativeActionGoalDescription, FluentExpression, TimedAssignOperator,
    TimedEffect, TimedFluentExpression,
};

use super::rounding_slack;
use super::timed::{condition, Moment};
use crate::atom::Atom;
use crate::eval::numeric::{degree, finite, fluent_atom};
use crate::eval::{EvalError, Evaluator, Observer, Scope};
use crate::model::Model;
use crate::report::{Construct, Unmet};
use crate::state::State;

/// A continuous effect of a running action, ground at its start.
#[derive(Debug, Clone)]
pub struct Flow<'d> {
    /// The fluent that changes.
    fluent: Atom,
    /// `1.0` for `increase`, `-1.0` for `decrease`.
    sign: f64,
    /// The rate per time unit; `None` for `#t` alone, a rate of 1.
    rate: Option<&'d FluentExpression>,
    /// The bindings the rate is evaluated with.
    scope: Scope,
}

/// Collects the continuous effects of `effect` with `scope` bound.
pub fn flows<'d>(
    evaluator: &Evaluator<'_, '_>,
    effect: &'d DurativeActionEffect,
    scope: &mut Scope,
    out: &mut Vec<Flow<'d>>,
) -> Result<(), EvalError> {
    match effect {
        DurativeActionEffect::Timed(TimedEffect::ContinuousEffect(op, head, rate)) => {
            out.push(Flow {
                fluent: fluent_atom(head, scope),
                sign: match op {
                    TimedAssignOperator::Increase => 1.0,
                    TimedAssignOperator::Decrease => -1.0,
                },
                rate: match rate {
                    TimedFluentExpression::Now => None,
                    TimedFluentExpression::Scaled(exp) => Some(exp),
                },
                scope: scope.clone(),
            });
            Ok(())
        }
        DurativeActionEffect::All(items) => items
            .iter()
            .try_for_each(|item| flows(evaluator, item, scope, out)),
        DurativeActionEffect::Forall(variables, inner) => {
            evaluator.each_binding(variables, scope, |scope| {
                flows(evaluator, inner, scope, out)
            })
        }
        // The static check rejects `when` on continuous effects.
        DurativeActionEffect::Timed(_) | DurativeActionEffect::When(..) => Ok(()),
    }
}

/// The rates of change of the continuously changing fluents, per time unit.
pub type Rates = BTreeMap<Atom, f64>;

/// Evaluates the rates of `flows` in the state at the start of an interval.
///
/// # Errors
///
/// [`Construct::NonlinearDynamics`] if a rate reads a changing fluent,
/// [`EvalError::Undefined`] if a changing fluent or a rate has no value.
pub fn rates(evaluator: &Evaluator<'_, '_>, flows: &[&Flow<'_>]) -> Result<Rates, EvalError> {
    let moving: BTreeSet<Atom> = flows.iter().map(|flow| flow.fluent.clone()).collect();
    let mut rates = Rates::new();
    for flow in flows {
        evaluator.fluent(&flow.fluent)?;
        let rate = match flow.rate {
            None => 1.0,
            Some(exp) if degree(exp, &flow.scope, &moving) > 0 => {
                return Err(EvalError::Unsupported(Construct::NonlinearDynamics));
            }
            Some(exp) => evaluator.value(exp, &flow.scope)?,
        };
        *rates.entry(flow.fluent.clone()).or_insert(0.0) += flow.sign * rate;
    }

    Ok(rates)
}

/// Returns `state` advanced by `elapsed` time units at `rates`.
///
/// # Errors
///
/// [`EvalError::NonFinite`] naming the first fluent that leaves the finite range.
pub fn advance(state: &State, rates: &Rates, elapsed: f64) -> Result<State, EvalError> {
    let mut next = state.clone();
    for (fluent, rate) in rates {
        if let Some(value) = next.fluents.get_mut(fluent) {
            *value = finite(*value + rate * elapsed, || format!("{fluent}"))?;
        }
    }

    Ok(next)
}

/// An open interval of time with the state at its start and the rates within it.
pub struct Interval<'s> {
    /// The start of the interval, the time of the happening before it.
    pub start: f64,
    /// The end of the interval, the time of the happening after it.
    pub end: f64,
    /// The state right after the happening at `start`.
    pub state: &'s State,
    /// The rates of change within the interval.
    pub rates: &'s Rates,
}

impl Interval<'_> {
    /// Checks that the `over all` parts of `invariant` hold throughout the interval.
    /// Returns a time at which they fail, with what fails.
    ///
    /// # Errors
    ///
    /// [`Construct::NonlinearDynamics`] for a comparison that is not linear in time, and
    /// the evaluation errors of the condition.
    pub fn check(
        &self,
        model: &Model<'_>,
        tolerance: f64,
        invariant: &DurativeActionGoalDescription,
        scope: &mut Scope,
    ) -> Result<Option<(f64, Unmet)>, EvalError> {
        let at = |time: f64, observer: Option<&RefCell<Observer>>, scope: &mut Scope| {
            let state = advance(self.state, self.rates, time - self.start)?;
            let evaluator = Evaluator {
                model,
                state: &state,
                tolerance,
                observer,
            };
            condition(&evaluator, invariant, scope, Moment::Over)
        };

        let middle = self.start.midpoint(self.end);
        if self.rates.is_empty() {
            return Ok(at(middle, None, scope)?.map(|unmet| (middle, unmet)));
        }

        for time in self.samples(|time, observer| at(time, Some(observer), scope))? {
            if let Some(unmet) = at(time, None, scope)? {
                return Ok(Some((time, unmet)));
            }
        }

        Ok(None)
    }

    /// Returns the times to check: every root of a comparison inside the interval and
    /// the midpoints between them and the ends. `probe` evaluates the condition at a
    /// time with an observer.
    fn samples(
        &self,
        mut probe: impl FnMut(f64, &RefCell<Observer>) -> Result<Option<Unmet>, EvalError>,
    ) -> Result<Vec<f64>, EvalError> {
        let moving: BTreeSet<Atom> = self.rates.keys().cloned().collect();
        let mut differences = |time: f64| {
            let observer = RefCell::new(Observer {
                moving: moving.clone(),
                ..Observer::default()
            });
            probe(time, &observer)?;
            let observer = observer.into_inner();
            if observer.nonlinear {
                return Err(EvalError::Unsupported(Construct::NonlinearDynamics));
            }

            Ok(observer.differences)
        };
        let first = differences(self.start)?;
        let last = differences(self.end)?;
        if first.len() != last.len() {
            return Err(EvalError::Unsupported(Construct::NonlinearDynamics));
        }

        let length = self.end - self.start;
        let mut points: Vec<f64> = first
            .iter()
            .zip(&last)
            .filter(|(a, b)| *a - *b != 0.0)
            .map(|(a, b)| self.start + length * a / (a - b))
            .filter(|time| self.is_inside(*time))
            .collect();
        points.push(self.start);
        points.push(self.end);
        points.sort_by(f64::total_cmp);
        points.dedup();

        let mut samples: Vec<f64> = points
            .iter()
            .zip(points.iter().skip(1))
            .map(|(a, b)| a.midpoint(*b))
            .collect();
        samples.extend(
            points
                .iter()
                .copied()
                .filter(|t| *t > self.start && *t < self.end),
        );
        samples.sort_by(f64::total_cmp);
        Ok(samples)
    }

    /// Returns `true` if `time` lies inside the interval by more than the rounding slack
    /// at either end. A root closer to an end than that is rounding noise of a crossing
    /// at the end itself, where the comparison is not checked, so it is not sampled.
    fn is_inside(&self, time: f64) -> bool {
        time > self.start + rounding_slack(self.start) && time < self.end - rounding_slack(self.end)
    }
}
