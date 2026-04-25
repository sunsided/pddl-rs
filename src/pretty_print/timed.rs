use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    DurationConstraint, DurationValue, DurativeActionEffect, DurativeActionGoalDefinition,
    PreferenceTimedGoalDefinition, SimpleDurationConstraint, TimedEffect, TimedGoalDefinition,
};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for DurationValue {}
impl sealed::Sealed for SimpleDurationConstraint {}
impl sealed::Sealed for DurationConstraint {}
impl sealed::Sealed for TimedGoalDefinition {}
impl sealed::Sealed for PreferenceTimedGoalDefinition {}
impl sealed::Sealed for TimedEffect {}
impl sealed::Sealed for DurativeActionEffect {}
impl sealed::Sealed for DurativeActionGoalDefinition {}

impl Visitor<DurationValue, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurationValue) -> RcDoc<'static> {
        match value {
            DurationValue::Number(n) => n.accept(self),
            DurationValue::FluentExpression(e) => e.accept(self),
        }
    }
}

impl Visitor<SimpleDurationConstraint, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &SimpleDurationConstraint) -> RcDoc<'static> {
        match value {
            SimpleDurationConstraint::Op(op, dv) => RcDoc::text("(")
                .append(op.accept(self))
                .append(" ?duration ")
                .append(dv.accept(self))
                .append(")"),
            SimpleDurationConstraint::At(ts, inner) => {
                let inner_doc = self.visit(inner.as_ref());
                RcDoc::text("(at ")
                    .append(ts.accept(self))
                    .append(" ")
                    .append(inner_doc)
                    .append(")")
            }
        }
    }
}

impl Visitor<DurationConstraint, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurationConstraint) -> RcDoc<'static> {
        match value {
            DurationConstraint::Single(sdc) => sdc.accept(self),
            DurationConstraint::All(sdcs) => RcDoc::text("(and")
                .append(RcDoc::softline())
                .append(RcDoc::intersperse(
                    sdcs.iter().map(|sdc| sdc.accept(self)),
                    RcDoc::softline(),
                ))
                .nest(4)
                .group()
                .append(")"),
        }
    }
}

impl Visitor<TimedGoalDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &TimedGoalDefinition) -> RcDoc<'static> {
        match value {
            TimedGoalDefinition::At(ts, gd) => RcDoc::text("(at ")
                .append(ts.accept(self))
                .append(" ")
                .append(gd.accept(self))
                .append(")"),
            TimedGoalDefinition::Over(interval, gd) => RcDoc::text("(over ")
                .append(interval.accept(self))
                .append(" ")
                .append(gd.accept(self))
                .append(")"),
        }
    }
}

impl Visitor<PreferenceTimedGoalDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreferenceTimedGoalDefinition) -> RcDoc<'static> {
        match value {
            PreferenceTimedGoalDefinition::Required(tgd) => tgd.accept(self),
            PreferenceTimedGoalDefinition::Preference(name, tgd) => {
                let children: Vec<RcDoc<'static>> = match name {
                    Some(n) => vec![n.accept(self), self.visit(tgd)],
                    None => vec![self.visit(tgd)],
                };
                self.sexpr("preference", children)
            }
        }
    }
}

impl Visitor<TimedEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &TimedEffect) -> RcDoc<'static> {
        match value {
            TimedEffect::Conditional(ts, ce) => RcDoc::text("(at ")
                .append(ts.accept(self))
                .append(" ")
                .append(ce.accept(self))
                .append(")"),
            TimedEffect::NumericFluent(ts, fassign) => RcDoc::text("(at ")
                .append(ts.accept(self))
                .append(" ")
                .append(fassign.accept(self))
                .append(")"),
            TimedEffect::ContinuousEffect(op, head, exp) => RcDoc::text("(")
                .append(op.accept(self))
                .append(" ")
                .append(head.accept(self))
                .append(" ")
                .append(exp.accept(self))
                .append(")"),
        }
    }
}

impl Visitor<DurativeActionGoalDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurativeActionGoalDefinition) -> RcDoc<'static> {
        match value {
            DurativeActionGoalDefinition::Timed(ptgd) => ptgd.accept(self),
            DurativeActionGoalDefinition::And(items) => {
                if items.is_empty() {
                    RcDoc::text("(and)")
                } else {
                    self.sexpr("and", items.iter().map(|dagd| self.visit(dagd)))
                }
            }
            DurativeActionGoalDefinition::Forall(vars, body) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    self.visit(body.as_ref()),
                ],
            ),
        }
    }
}

impl Visitor<DurativeActionEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurativeActionEffect) -> RcDoc<'static> {
        match value {
            DurativeActionEffect::Timed(te) => te.accept(self),
            DurativeActionEffect::All(effects) => RcDoc::text("(and")
                .append(RcDoc::softline())
                .append(RcDoc::intersperse(
                    effects.iter().map(|e| self.visit(e)),
                    RcDoc::softline(),
                ))
                .nest(4)
                .group()
                .append(")"),
            DurativeActionEffect::Forall(vars, body) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    self.visit(body.as_ref()),
                ],
            ),
            DurativeActionEffect::When(condition, effect) => RcDoc::text("(at ")
                .append(self.visit(condition))
                .append(" ")
                .append(self.visit(effect))
                .append(")"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;
    use crate::{DurationOperator, PreferenceTimedGoalDefinition};

    #[test]
    fn duration_value_number() {
        let dv = DurationValue::new_number(10);
        assert_eq!(prettify!(dv, 20), "10");
    }

    #[test]
    fn simple_duration_constraint_op() {
        let sdc =
            SimpleDurationConstraint::new_op(DurationOperator::Equal, DurationValue::new_number(10));
        assert_eq!(prettify!(sdc, 30), "(= ?duration 10)");
    }

    #[test]
    fn duration_value_fexp() {
        let exp = crate::FluentExpression::new_number(crate::Number::from(5));
        let dv = DurationValue::new_f_exp(exp);
        assert_eq!(prettify!(dv, 20), "5");
    }

    #[test]
    fn simple_duration_constraint_at() {
        let inner =
            SimpleDurationConstraint::new_op(DurationOperator::Equal, DurationValue::new_number(10));
        let sdc = SimpleDurationConstraint::new_at(crate::TimeSpecifier::Start, inner);
        assert_eq!(prettify!(sdc, 30), "(at start (= ?duration 10))");
    }

    #[test]
    fn duration_constraint_single() {
        let sdc =
            SimpleDurationConstraint::new_op(DurationOperator::Equal, DurationValue::new_number(10));
        let dc = DurationConstraint::new(sdc);
        assert_eq!(prettify!(dc, 30), "(= ?duration 10)");
    }

    #[test]
    fn duration_constraint_all() {
        let sdc1 =
            SimpleDurationConstraint::new_op(DurationOperator::Equal, DurationValue::new_number(10));
        let sdc2 = SimpleDurationConstraint::new_op(
            DurationOperator::GreaterOrEqual,
            DurationValue::new_number(5),
        );
        let dc = DurationConstraint::new_all([sdc1, sdc2]);
        let out = prettify!(dc, 30);
        assert!(out.starts_with("(and"));
        assert!(out.contains("(= ?duration 10)"));
        assert!(out.contains("(>= ?duration 5)"));
    }

    #[test]
    fn timed_gd_at() {
        let gd = crate::GoalDefinition::new_and(Vec::<crate::GoalDefinition>::new());
        let tgd = TimedGoalDefinition::new_at(crate::TimeSpecifier::Start, gd);
        assert_eq!(prettify!(tgd, 30), "(at start (and))");
    }

    #[test]
    fn timed_gd_over() {
        use crate::Interval;
        let gd = crate::GoalDefinition::new_and(Vec::<crate::GoalDefinition>::new());
        let tgd = TimedGoalDefinition::new_over(Interval::All, gd);
        assert_eq!(prettify!(tgd, 30), "(over all (and))");
    }

    #[test]
    fn pref_timed_gd_required() {
        let gd = crate::GoalDefinition::new_and(Vec::<crate::GoalDefinition>::new());
        let tgd = TimedGoalDefinition::new_at(crate::TimeSpecifier::Start, gd);
        let ptgd = PreferenceTimedGoalDefinition::new_required(tgd);
        assert_eq!(prettify!(ptgd, 30), "(at start (and))");
    }

    #[test]
    fn pref_timed_gd_preference_with_name() {
        use crate::PreferenceName;
        let gd = crate::GoalDefinition::new_and(Vec::<crate::GoalDefinition>::new());
        let tgd = TimedGoalDefinition::new_at(crate::TimeSpecifier::Start, gd);
        let ptgd = PreferenceTimedGoalDefinition::new_preference(Some(PreferenceName::new_string("p1")), tgd);
        assert_eq!(prettify!(ptgd, 40), "(preference p1 (at start (and)))");
    }

    #[test]
    fn pref_timed_gd_preference_no_name() {
        let gd = crate::GoalDefinition::new_and(Vec::<crate::GoalDefinition>::new());
        let tgd = TimedGoalDefinition::new_at(crate::TimeSpecifier::Start, gd);
        let ptgd = PreferenceTimedGoalDefinition::new_preference(None, tgd);
        assert_eq!(prettify!(ptgd, 40), "(preference (at start (and)))");
    }

    #[test]
    fn timed_effect_conditional() {
        use crate::EffectCondition;
        let ce = EffectCondition::new_and(Vec::new());
        let te = TimedEffect::new_conditional(crate::TimeSpecifier::Start, ce);
        assert_eq!(prettify!(te, 30), "(at start (and))");
    }

    #[test]
    fn timed_effect_numeric_fluent() {
        use crate::{AssignOp, FluentExpression, DurativeActionFluentExpression, FunctionHead, FunctionSymbol, Number, DurativeActionFunctionAssignment};
        let head = FunctionHead::Simple(FunctionSymbol::from("x"));
        let expr = DurativeActionFluentExpression::new_f_exp(FluentExpression::new_number(Number::from(5)));
        let fassign = DurativeActionFunctionAssignment::new(AssignOp::Increase, head, expr);
        let te = TimedEffect::new_fluent(crate::TimeSpecifier::End, fassign);
        assert_eq!(prettify!(te, 30), "(at end (increase x 5))");
    }

    #[test]
    fn timed_effect_continuous() {
        use crate::{TimedAssignOperator, FluentExpression, FunctionHead, FunctionSymbol, Number, TimedFluentExpression};
        let head = FunctionHead::Simple(FunctionSymbol::from("x"));
        let exp = FluentExpression::new_number(Number::from(1));
        let te = TimedEffect::new_continuous(TimedAssignOperator::Increase, head, TimedFluentExpression::Scaled(exp));
        assert_eq!(prettify!(te, 30), "(increase x 1)");
    }

    #[test]
    fn durative_action_goal_timed() {
        let gd = crate::GoalDefinition::new_and(Vec::<crate::GoalDefinition>::new());
        let tgd = TimedGoalDefinition::new_at(crate::TimeSpecifier::Start, gd);
        let ptgd = PreferenceTimedGoalDefinition::new_required(tgd);
        let dagd = DurativeActionGoalDefinition::new_timed(ptgd);
        assert_eq!(prettify!(dagd, 30), "(at start (and))");
    }

    #[test]
    fn durative_action_goal_and_empty() {
        let dagd = DurativeActionGoalDefinition::new_and(Vec::new());
        assert_eq!(prettify!(dagd, 30), "(and)");
    }

    #[test]
    fn durative_action_goal_and_non_empty() {
        let gd = crate::GoalDefinition::new_and(Vec::<crate::GoalDefinition>::new());
        let tgd = TimedGoalDefinition::new_at(crate::TimeSpecifier::Start, gd);
        let ptgd = PreferenceTimedGoalDefinition::new_required(tgd);
        let dagd =
            DurativeActionGoalDefinition::new_and([DurativeActionGoalDefinition::new_timed(ptgd)]);
        assert_eq!(prettify!(dagd, 40), "(and (at start (and)))");
    }

    #[test]
    fn durative_action_goal_forall() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::new_string("x").to_typed(Type::OBJECT)].into();
        let inner = DurativeActionGoalDefinition::new_and(Vec::new());
        let dagd = DurativeActionGoalDefinition::new_forall(vars, inner);
        assert_eq!(prettify!(dagd, 40), "(forall (?x) (and))");
    }

    #[test]
    fn durative_action_effect_timed() {
        use crate::EffectCondition;
        let ce = EffectCondition::new_and(Vec::new());
        let te = TimedEffect::new_conditional(crate::TimeSpecifier::Start, ce);
        let dae = DurativeActionEffect::new_timed(te);
        assert_eq!(prettify!(dae, 30), "(at start (and))");
    }

    #[test]
    fn durative_action_effect_all() {
        use crate::EffectCondition;
        let ce = EffectCondition::new_and(Vec::new());
        let te = TimedEffect::new_conditional(crate::TimeSpecifier::Start, ce);
        let dae = DurativeActionEffect::new_and([DurativeActionEffect::new_timed(te)]);
        assert_eq!(prettify!(dae, 40), "(and (at start (and)))");
    }

    #[test]
    fn durative_action_effect_forall() {
        use crate::{EffectCondition, ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::new_string("x").to_typed(Type::OBJECT)].into();
        let ce = EffectCondition::new_and(Vec::new());
        let inner = DurativeActionEffect::new_timed(TimedEffect::new_conditional(
            crate::TimeSpecifier::Start,
            ce,
        ));
        let dae = DurativeActionEffect::new_forall(vars, inner);
        assert_eq!(prettify!(dae, 40), "(forall (?x) (at start (and)))");
    }

    #[test]
    fn durative_action_effect_when() {
        use crate::{EffectCondition, GoalDefinition};
        let gd = GoalDefinition::new_and(Vec::<GoalDefinition>::new());
        let ce = EffectCondition::new_and(Vec::new());
        let te = TimedEffect::new_conditional(crate::TimeSpecifier::Start, ce);
        let dae = DurativeActionEffect::new_when(
            DurativeActionGoalDefinition::new_timed(PreferenceTimedGoalDefinition::new_required(TimedGoalDefinition::new_at(
                crate::TimeSpecifier::Start,
                gd,
            ))),
            te,
        );
        assert_eq!(prettify!(dae, 40), "(at (at start (and)) (at start (and)))");
    }
}
