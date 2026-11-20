//! Timed conditions, duration constraints and discrete timed effects of durative actions.

use alloc::collections::VecDeque;
use alloc::format;
use alloc::string::String;

use pddl::{
    DurationConstraint, DurationOperator, DurationValue, DurativeActionEffect,
    DurativeActionFluentExpression, DurativeActionGoalDescription, EffectCondition,
    PreferenceTimedGoalDescription, SimpleDurationConstraint, TimeSpecifier, TimedEffect,
    TimedGoalDescription,
};

use crate::effects::Changes;
use crate::eval::numeric::{binary, compare, finite, fluent_atom, multi};
use crate::eval::{EvalError, Evaluator, Scope, Verdict};
use crate::render;
use crate::report::{Construct, Unmet};

/// When a part of a durative action applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Moment {
    /// `at start`.
    Start,
    /// `at end`.
    End,
    /// `over all`.
    Over,
}

impl Moment {
    /// Returns the moment of a time specifier.
    const fn of(time: TimeSpecifier) -> Self {
        match time {
            TimeSpecifier::Start => Self::Start,
            TimeSpecifier::End => Self::End,
        }
    }
}

/// Checks the parts of a durative action's condition that apply at `moment`.
pub fn condition(
    evaluator: &Evaluator<'_, '_>,
    condition: &DurativeActionGoalDescription,
    scope: &mut Scope,
    moment: Moment,
) -> Verdict {
    match condition {
        DurativeActionGoalDescription::Timed(PreferenceTimedGoalDescription::Required(timed)) => {
            let (applies, gd) = match timed {
                TimedGoalDescription::At(time, gd) => (Moment::of(*time) == moment, gd),
                TimedGoalDescription::Over(_, gd) => (moment == Moment::Over, gd),
            };
            if applies {
                evaluator.holds(gd, scope, true)
            } else {
                Ok(None)
            }
        }
        DurativeActionGoalDescription::Timed(PreferenceTimedGoalDescription::Preference(..)) => {
            Err(EvalError::Unsupported(Construct::Preferences))
        }
        DurativeActionGoalDescription::And(items) => {
            let mut first = None;
            for item in items {
                let unmet = self::condition(evaluator, item, scope, moment)?;
                first = first.or(unmet);
            }

            Ok(first)
        }
        DurativeActionGoalDescription::Forall(variables, inner) => {
            let mut first = None;
            evaluator.each_binding(variables, scope, |scope| {
                let unmet = self::condition(evaluator, inner, scope, moment)?;
                first = first.take().or(unmet);
                Ok(())
            })?;
            Ok(first)
        }
    }
}

/// Checks the parts of a `:duration` constraint that apply at `moment` (`at start`
/// unless they say otherwise) for the duration `duration`.
pub fn duration(
    evaluator: &Evaluator<'_, '_>,
    constraint: &DurationConstraint,
    scope: &Scope,
    duration: f64,
    moment: Moment,
) -> Verdict {
    let simple = match constraint {
        DurationConstraint::Single(simple) => core::slice::from_ref(simple),
        DurationConstraint::All(all) => all.as_slice(),
    };
    let mut first = None;
    for constraint in simple {
        let unmet = simple_duration(
            evaluator,
            constraint,
            scope,
            duration,
            moment,
            Moment::Start,
        )?;
        first = first.or(unmet);
    }

    Ok(first)
}

/// Checks one duration constraint, which applies at `at` unless it says otherwise.
fn simple_duration(
    evaluator: &Evaluator<'_, '_>,
    constraint: &SimpleDurationConstraint,
    scope: &Scope,
    duration: f64,
    moment: Moment,
    at: Moment,
) -> Verdict {
    match constraint {
        SimpleDurationConstraint::At(time, inner) => {
            simple_duration(evaluator, inner, scope, duration, moment, Moment::of(*time))
        }
        SimpleDurationConstraint::Op(_, _) if at != moment => Ok(None),
        SimpleDurationConstraint::Op(op, value) => {
            let (bound, text) = match value {
                DurationValue::Number(number) => (**number, format!("{}", **number)),
                DurationValue::FluentExpression(exp) => {
                    (evaluator.value(exp, scope)?, render::expression(exp, scope))
                }
            };
            let order = compare(duration, bound, evaluator.tolerance);
            let (holds, keyword) = match op {
                DurationOperator::Equal => (order.is_eq(), "="),
                DurationOperator::GreaterOrEqual => (order.is_ge(), ">="),
                DurationOperator::LessThanOrEqual => (order.is_le(), "<="),
            };

            Ok((!holds).then(|| Unmet::Comparison {
                condition: format!("({keyword} ?duration {text})"),
                left: duration,
                right: bound,
                expected: true,
            }))
        }
    }
}

/// The outcomes of `at start` effect conditions, kept until the action ends, in the
/// order the effects are walked.
pub type Memo = VecDeque<bool>;

/// Collects the discrete effects of a durative action that happen at `moment`
/// (`Start` or `End`) for the duration `duration`.
pub struct Collector<'e, 's, 'a> {
    /// Evaluates conditions and right-hand sides in the state before the happening.
    pub evaluator: &'e Evaluator<'s, 'a>,
    /// `Start` or `End`.
    pub moment: Moment,
    /// The duration, for `?duration`.
    pub duration: f64,
}

impl Collector<'_, '_, '_> {
    /// Collects the effects of `effect` into `changes`. At the start it records the
    /// `at start` conditions of `at end` effects in `memo`; at the end it consumes them.
    pub fn collect(
        &self,
        effect: &DurativeActionEffect,
        scope: &mut Scope,
        memo: &mut Memo,
        changes: &mut Changes,
    ) -> Result<(), EvalError> {
        match effect {
            DurativeActionEffect::Timed(timed) => self.timed(timed, scope, changes),
            DurativeActionEffect::All(items) => items
                .iter()
                .try_for_each(|item| self.collect(item, scope, memo, changes)),
            DurativeActionEffect::Forall(variables, inner) => {
                self.evaluator.each_binding(variables, scope, |scope| {
                    self.collect(inner, scope, memo, changes)
                })
            }
            DurativeActionEffect::When(when, timed) => {
                self.conditional(when, timed, scope, memo, changes)
            }
        }
    }

    /// Collects a conditional effect.
    fn conditional(
        &self,
        when: &DurativeActionGoalDescription,
        timed: &TimedEffect,
        scope: &mut Scope,
        memo: &mut Memo,
        changes: &mut Changes,
    ) -> Result<(), EvalError> {
        let Some(time) = effect_moment(timed) else {
            return Err(EvalError::Unsupported(Construct::TimedEffectConditions));
        };

        let start_holds = |scope: &mut Scope| {
            Ok::<_, EvalError>(condition(self.evaluator, when, scope, Moment::Start)?.is_none())
        };
        match (self.moment, time) {
            (Moment::Start, Moment::Start) => {
                if start_holds(scope)? {
                    self.timed(timed, scope, changes)?;
                }
            }
            (Moment::Start, Moment::End) => memo.push_back(start_holds(scope)?),
            (Moment::End, Moment::End) => {
                let started = memo.pop_front().unwrap_or(false);
                let ends = condition(self.evaluator, when, scope, Moment::End)?.is_none();
                if started && ends {
                    self.timed(timed, scope, changes)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Collects one timed effect if it happens now.
    fn timed(
        &self,
        effect: &TimedEffect,
        scope: &mut Scope,
        changes: &mut Changes,
    ) -> Result<(), EvalError> {
        match effect {
            TimedEffect::Conditional(time, primitives) if Moment::of(*time) == self.moment => {
                let primitives = match primitives {
                    EffectCondition::Single(primitive) => core::slice::from_ref(primitive),
                    EffectCondition::All(primitives) => primitives.as_slice(),
                };
                for primitive in primitives {
                    changes.primitive(self.evaluator, primitive, scope)?;
                }

                Ok(())
            }
            TimedEffect::NumericFluent(time, assignment) if Moment::of(*time) == self.moment => {
                let value = self.value(assignment.function_expr(), scope)?;
                let fluent = fluent_atom(assignment.function(), scope);
                changes.update(fluent, assignment.operation().clone(), value);
                Ok(())
            }
            TimedEffect::Conditional(..)
            | TimedEffect::NumericFluent(..)
            | TimedEffect::ContinuousEffect(..) => Ok(()),
        }
    }

    /// Evaluates a numeric expression that may contain `?duration`.
    fn value(&self, exp: &DurativeActionFluentExpression, scope: &Scope) -> Result<f64, EvalError> {
        let value = match exp {
            DurativeActionFluentExpression::Assign(..) => {
                return Err(EvalError::Unsupported(Construct::NestedAssignments));
            }
            DurativeActionFluentExpression::BinaryOp(op, left, right) => {
                let left = self.value(left, scope)?;
                let right = self.value(right, scope)?;
                binary(*op, left, right).ok_or_else(|| {
                    EvalError::DivisionByZero(String::from("?duration expression"))
                })?
            }
            DurativeActionFluentExpression::MultiOp(op, first, rest) => {
                let mut total = self.value(first, scope)?;
                for item in rest {
                    total = multi(*op, total, self.value(item, scope)?);
                }

                total
            }
            DurativeActionFluentExpression::Negative(inner) => -self.value(inner, scope)?,
            DurativeActionFluentExpression::Duration => self.duration,
            DurativeActionFluentExpression::FluentExpression(exp) => {
                return self.evaluator.value(exp, scope);
            }
        };

        finite(value, || String::from("?duration expression"))
    }
}

/// Returns when a discrete timed effect happens; `None` for continuous change.
pub const fn effect_moment(effect: &TimedEffect) -> Option<Moment> {
    match effect {
        TimedEffect::Conditional(time, _) | TimedEffect::NumericFluent(time, _) => {
            Some(Moment::of(*time))
        }
        TimedEffect::ContinuousEffect(..) => None,
    }
}
