//! Contains durative action effects via the [`DurativeActionEffect`] type.

use crate::types::TypedVariables;
use crate::types::{DurativeActionGoalDefinition, TimedEffect};

/// A durative action effect used in [`DurativeActionDefinition`](crate::types::DurativeActionDefinition).
///
/// ## Usage
/// Used by [`DurativeActionDefinition`](crate::DurativeActionDefinition).
#[derive(Debug, Clone, PartialEq)]
pub enum DurativeActionEffect {
    Timed(TimedEffect),
    /// Conjunction: All effects apply (i.e. a and b and c ..).
    All(Vec<DurativeActionEffect>),
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    Forall(TypedVariables, Box<DurativeActionEffect>),
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    When(DurativeActionGoalDefinition, TimedEffect),
}

impl DurativeActionEffect {
    #[doc(alias = "new_timed")]
    pub const fn timed(effect: TimedEffect) -> Self {
        Self::Timed(effect)
    }

    pub fn new_timed(effect: TimedEffect) -> Self {
        Self::timed(effect)
    }

    #[doc(alias = "new_and")]
    pub fn and<E: IntoIterator<Item = DurativeActionEffect>>(effect: E) -> Self {
        Self::All(effect.into_iter().collect())
    }

    pub fn new_and<E: IntoIterator<Item = DurativeActionEffect>>(effect: E) -> Self {
        Self::and(effect)
    }

    #[doc(alias = "new_forall")]
    pub fn r#forall(variables: TypedVariables, effect: DurativeActionEffect) -> Self {
        Self::Forall(variables, Box::new(effect))
    }

    pub fn new_forall(variables: TypedVariables, effect: DurativeActionEffect) -> Self {
        Self::r#forall(variables, effect)
    }

    #[doc(alias = "new_when")]
    pub const fn when(gd: DurativeActionGoalDefinition, effect: TimedEffect) -> Self {
        Self::When(gd, effect)
    }

    pub fn new_when(gd: DurativeActionGoalDefinition, effect: TimedEffect) -> Self {
        Self::when(gd, effect)
    }
}

impl From<TimedEffect> for DurativeActionEffect {
    fn from(value: TimedEffect) -> Self {
        Self::timed(value)
    }
}

impl FromIterator<DurativeActionEffect> for DurativeActionEffect {
    fn from_iter<T: IntoIterator<Item = DurativeActionEffect>>(iter: T) -> Self {
        Self::and(iter)
    }
}

impl From<(TypedVariables, DurativeActionEffect)> for DurativeActionEffect {
    fn from(value: (TypedVariables, DurativeActionEffect)) -> Self {
        Self::r#forall(value.0, value.1)
    }
}

impl From<(DurativeActionGoalDefinition, TimedEffect)> for DurativeActionEffect {
    fn from(value: (DurativeActionGoalDefinition, TimedEffect)) -> Self {
        Self::when(value.0, value.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AtomicFormula, EffectCondition, GoalDefinition, Name, Predicate,
        PreferenceTimedGoalDefinition, PrimitiveEffect, Term, TimeSpecifier, TimedGoalDefinition,
        TypedVariables,
    };

    fn make_timed_effect() -> TimedEffect {
        let af = AtomicFormula::<Term>::predicate(
            Predicate::string("on"),
            vec![Term::new_name(Name::new("a"))],
        );
        let ec = EffectCondition::new(PrimitiveEffect::atomic_formula(af));
        TimedEffect::conditional(TimeSpecifier::Start, ec)
    }

    fn make_da_effect() -> DurativeActionEffect {
        DurativeActionEffect::timed(make_timed_effect())
    }

    fn make_da_gd() -> DurativeActionGoalDefinition {
        let af = AtomicFormula::<Term>::predicate(
            Predicate::string("clear"),
            vec![Term::new_name(Name::new("b"))],
        );
        let timed_gd =
            TimedGoalDefinition::at(TimeSpecifier::Start, GoalDefinition::AtomicFormula(af));
        let pref_timed = PreferenceTimedGoalDefinition::required(timed_gd);
        DurativeActionGoalDefinition::timed(pref_timed)
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_new_constructors() {
        let effect = make_timed_effect();
        let _ = DurativeActionEffect::new_timed(effect.clone());
        let _ = DurativeActionEffect::new_and(vec![DurativeActionEffect::timed(effect.clone())]);
        let _ = DurativeActionEffect::new_forall(
            TypedVariables::default(),
            DurativeActionEffect::timed(effect.clone()),
        );
        let da_gd = make_da_gd();
        let _ = DurativeActionEffect::new_when(da_gd, effect);
    }

    #[test]
    fn from_timed_effect() {
        let effect = make_timed_effect();
        let from_effect: DurativeActionEffect = effect.clone().into();
        assert_eq!(from_effect, DurativeActionEffect::timed(effect));
    }

    #[test]
    fn from_iterator() {
        let effect = make_da_effect();
        let from_iter: DurativeActionEffect =
            vec![effect.clone(), effect.clone()].into_iter().collect();
        assert_eq!(
            from_iter,
            DurativeActionEffect::and(vec![effect, make_da_effect()])
        );
    }

    #[test]
    fn from_tuple_forall() {
        let effect = make_da_effect();
        let vars = TypedVariables::default();
        let from_tuple: DurativeActionEffect = (vars.clone(), effect.clone()).into();
        assert_eq!(from_tuple, DurativeActionEffect::r#forall(vars, effect));
    }

    #[test]
    fn from_tuple_when() {
        let effect = make_timed_effect();
        let da_gd = make_da_gd();
        let from_tuple: DurativeActionEffect = (da_gd.clone(), effect.clone()).into();
        assert_eq!(from_tuple, DurativeActionEffect::when(da_gd, effect));
    }
}
