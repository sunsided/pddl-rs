//! The static check of durative actions.

use pddl::{
    DurationConstraint, DurationValue, DurativeActionDefinition, DurativeActionEffect,
    DurativeActionFluentExpression, DurativeActionGoalDescription, EffectCondition,
    PreferenceTimedGoalDescription, SimpleDurationConstraint, TimeSpecifier, TimedEffect,
    TimedFluentExpression, TimedGoalDescription,
};

use super::{Checker, Issue};
use crate::report::Construct;

impl Checker<'_, '_> {
    /// Checks a durative action: parameters, duration constraint, conditions, effects.
    pub(super) fn durative(&mut self, action: &DurativeActionDefinition) -> Result<(), Issue> {
        self.bind(action.parameters())?;
        match action.duration() {
            Some(DurationConstraint::Single(constraint)) => self.duration(constraint)?,
            Some(DurationConstraint::All(constraints)) => {
                constraints.iter().try_for_each(|c| self.duration(c))?;
            }
            None => {}
        }

        if let Some(condition) = action.condition() {
            self.timed_condition(condition)?;
        }

        if let Some(effect) = action.effect() {
            self.timed_effects(effect)?;
        }

        Ok(())
    }

    /// Checks the effect of a PDDL+ process: continuous effects only, possibly under
    /// `and` and `forall`.
    pub(super) fn process_effect(&mut self, effect: &DurativeActionEffect) -> Result<(), Issue> {
        match effect {
            DurativeActionEffect::Timed(timed @ TimedEffect::ContinuousEffect(..)) => {
                self.timed_effect(timed)
            }
            DurativeActionEffect::All(items) => {
                items.iter().try_for_each(|item| self.process_effect(item))
            }
            DurativeActionEffect::Forall(variables, inner) => {
                let count = self.bind(variables)?;
                self.process_effect(inner)?;
                self.unbind(count);
                Ok(())
            }
            DurativeActionEffect::Timed(_) | DurativeActionEffect::When(..) => {
                Err(self.unsupported(Construct::DiscreteProcessEffects))
            }
        }
    }

    /// Checks one duration constraint.
    fn duration(&self, constraint: &SimpleDurationConstraint) -> Result<(), Issue> {
        match constraint {
            SimpleDurationConstraint::Op(_, DurationValue::Number(_)) => Ok(()),
            SimpleDurationConstraint::Op(_, DurationValue::FluentExpression(exp)) => {
                self.expression(exp)
            }
            SimpleDurationConstraint::At(_, inner) => self.duration(inner),
        }
    }

    /// Checks a timed condition.
    fn timed_condition(&mut self, condition: &DurativeActionGoalDescription) -> Result<(), Issue> {
        match condition {
            DurativeActionGoalDescription::Timed(PreferenceTimedGoalDescription::Required(
                TimedGoalDescription::At(_, gd) | TimedGoalDescription::Over(_, gd),
            )) => self.goal(gd),
            DurativeActionGoalDescription::Timed(PreferenceTimedGoalDescription::Preference(
                ..,
            )) => Err(self.unsupported(Construct::Preferences)),
            DurativeActionGoalDescription::And(items) => {
                items.iter().try_for_each(|item| self.timed_condition(item))
            }
            DurativeActionGoalDescription::Forall(variables, inner) => {
                let count = self.bind(variables)?;
                self.timed_condition(inner)?;
                self.unbind(count);
                Ok(())
            }
        }
    }

    /// Checks the effects of a durative action.
    fn timed_effects(&mut self, effect: &DurativeActionEffect) -> Result<(), Issue> {
        match effect {
            DurativeActionEffect::Timed(timed) => self.timed_effect(timed),
            DurativeActionEffect::All(items) => {
                items.iter().try_for_each(|item| self.timed_effects(item))
            }
            DurativeActionEffect::Forall(variables, inner) => {
                let count = self.bind(variables)?;
                self.timed_effects(inner)?;
                self.unbind(count);
                Ok(())
            }
            DurativeActionEffect::When(condition, timed) => {
                if !supported_effect_condition(condition, effect_time(timed)) {
                    return Err(self.unsupported(Construct::TimedEffectConditions));
                }

                self.timed_condition(condition)?;
                self.timed_effect(timed)
            }
        }
    }

    /// Checks one timed effect.
    fn timed_effect(&self, effect: &TimedEffect) -> Result<(), Issue> {
        match effect {
            TimedEffect::Conditional(_, EffectCondition::Single(primitive)) => {
                self.primitive(primitive)
            }
            TimedEffect::Conditional(_, EffectCondition::All(primitives)) => {
                primitives.iter().try_for_each(|p| self.primitive(p))
            }
            TimedEffect::NumericFluent(_, assignment) => {
                self.function(assignment.function())?;
                self.duration_expression(assignment.function_expr())
            }
            TimedEffect::ContinuousEffect(_, head, rate) => {
                self.function(head)?;
                match rate {
                    TimedFluentExpression::Now => Ok(()),
                    TimedFluentExpression::Scaled(exp) => self.expression(exp),
                }
            }
        }
    }

    /// Checks a numeric expression that may contain `?duration`.
    fn duration_expression(&self, exp: &DurativeActionFluentExpression) -> Result<(), Issue> {
        match exp {
            DurativeActionFluentExpression::Assign(..) => {
                Err(self.unsupported(Construct::NestedAssignments))
            }
            DurativeActionFluentExpression::BinaryOp(_, left, right) => {
                self.duration_expression(left)?;
                self.duration_expression(right)
            }
            DurativeActionFluentExpression::MultiOp(_, first, rest) => {
                self.duration_expression(first)?;
                rest.iter()
                    .try_for_each(|item| self.duration_expression(item))
            }
            DurativeActionFluentExpression::Negative(inner) => self.duration_expression(inner),
            DurativeActionFluentExpression::Duration => Ok(()),
            DurativeActionFluentExpression::FluentExpression(exp) => self.expression(exp),
        }
    }
}

/// Returns when a timed effect happens; `None` for continuous change.
pub fn effect_time(effect: &TimedEffect) -> Option<TimeSpecifier> {
    match effect {
        TimedEffect::Conditional(time, _) | TimedEffect::NumericFluent(time, _) => Some(*time),
        TimedEffect::ContinuousEffect(..) => None,
    }
}

/// Returns `true` if the validator interprets a `when` condition on an effect at `time`:
/// no `over all` part, no condition on continuous change, and no `at end` part on an
/// `at start` effect, which would need the future.
fn supported_effect_condition(
    condition: &DurativeActionGoalDescription,
    time: Option<TimeSpecifier>,
) -> bool {
    let Some(time) = time else {
        return false;
    };

    match condition {
        DurativeActionGoalDescription::Timed(PreferenceTimedGoalDescription::Required(
            TimedGoalDescription::At(at, _),
        )) => !(time == TimeSpecifier::Start && *at == TimeSpecifier::End),
        DurativeActionGoalDescription::Timed(_) => false,
        DurativeActionGoalDescription::And(items) => items
            .iter()
            .all(|item| supported_effect_condition(item, Some(time))),
        DurativeActionGoalDescription::Forall(_, inner) => {
            supported_effect_condition(inner, Some(time))
        }
    }
}
