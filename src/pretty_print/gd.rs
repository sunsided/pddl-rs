use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    ConstraintGoalDefinition, ConstraintGoalDefinitionInner, GoalDefinition,
    PreconditionGoalDefinition, PreconditionGoalDefinitions, Preference,
    PreferenceConstraintGoalDefinition, PreferenceConstraintGoalDefinitions,
    PreferenceGoalDefinition,
};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for GoalDefinition {}
impl sealed::Sealed for PreconditionGoalDefinition {}
impl sealed::Sealed for PreconditionGoalDefinitions {}
impl sealed::Sealed for PreferenceGoalDefinition {}
impl sealed::Sealed for Preference {}
impl sealed::Sealed for PreferenceConstraintGoalDefinition {}
impl sealed::Sealed for PreferenceConstraintGoalDefinitions {}
impl sealed::Sealed for ConstraintGoalDefinition {}
impl sealed::Sealed for ConstraintGoalDefinitionInner {}

impl Visitor<GoalDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &GoalDefinition) -> RcDoc<'static> {
        match value {
            GoalDefinition::AtomicFormula(af) => af.accept(self),
            GoalDefinition::Literal(lit) => lit.accept(self),
            GoalDefinition::And(items) => {
                if items.is_empty() {
                    RcDoc::text("(and)")
                } else {
                    self.sexpr("and", items.iter().map(|gd| self.visit(gd)))
                }
            }
            GoalDefinition::Or(items) => {
                if items.is_empty() {
                    RcDoc::text("(or)")
                } else {
                    self.sexpr("or", items.iter().map(|gd| self.visit(gd)))
                }
            }
            GoalDefinition::Not(gd) => self.sexpr("not", [self.visit(&**gd)]),
            GoalDefinition::Imply(a, b) => {
                self.sexpr("imply", [self.visit(&**a), self.visit(&**b)])
            }
            GoalDefinition::Exists(vars, body) => self.sexpr(
                "exists",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    self.visit(&**body),
                ],
            ),
            GoalDefinition::ForAll(vars, body) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    self.visit(&**body),
                ],
            ),
            GoalDefinition::FluentComparison(fc) => fc.accept(self),
        }
    }
}

impl Visitor<PreconditionGoalDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreconditionGoalDefinition) -> RcDoc<'static> {
        match value {
            PreconditionGoalDefinition::Preference(pref_gd) => self.visit(pref_gd),
            PreconditionGoalDefinition::Forall(vars, gds) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    RcDoc::intersperse(gds.iter().map(|gd| self.visit(gd)), RcDoc::softline()),
                ],
            ),
        }
    }
}

impl Visitor<PreconditionGoalDefinitions, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreconditionGoalDefinitions) -> RcDoc<'static> {
        if value.is_empty() {
            RcDoc::text("(and)")
        } else if value.len() == 1 {
            value[0].accept(self)
        } else {
            self.sexpr("and", value.iter().map(|gd| self.visit(gd)))
        }
    }
}

impl Visitor<PreferenceGoalDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreferenceGoalDefinition) -> RcDoc<'static> {
        match value {
            PreferenceGoalDefinition::Goal(gd) => self.visit(gd),
            PreferenceGoalDefinition::Preference(pref) => self.visit(pref),
        }
    }
}

impl Visitor<Preference, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Preference) -> RcDoc<'static> {
        let children: Vec<RcDoc<'static>> = match value.name() {
            Some(name) => {
                vec![name.accept(self), self.visit(value.goal())]
            }
            None => {
                vec![self.visit(value.goal())]
            }
        };
        self.sexpr("preference", children)
    }
}

impl Visitor<PreferenceConstraintGoalDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreferenceConstraintGoalDefinition) -> RcDoc<'static> {
        match value {
            PreferenceConstraintGoalDefinition::Goal(gd) => self.visit(gd),
            PreferenceConstraintGoalDefinition::Forall(vars, gds) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    RcDoc::intersperse(gds.iter().map(|gd| self.visit(gd)), RcDoc::softline()),
                ],
            ),
            PreferenceConstraintGoalDefinition::Preference(name, gd) => {
                let children: Vec<RcDoc<'static>> = match name {
                    Some(n) => vec![n.accept(self), self.visit(gd)],
                    None => vec![self.visit(gd)],
                };
                self.sexpr("preference", children)
            }
        }
    }
}

impl Visitor<PreferenceConstraintGoalDefinitions, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreferenceConstraintGoalDefinitions) -> RcDoc<'static> {
        RcDoc::intersperse(value.iter().map(|gd| self.visit(gd)), RcDoc::softline())
    }
}

impl Visitor<ConstraintGoalDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ConstraintGoalDefinition) -> RcDoc<'static> {
        match value {
            ConstraintGoalDefinition::And(items) => {
                if items.is_empty() {
                    RcDoc::text("(and)")
                } else {
                    self.sexpr("and", items.iter().map(|gd| self.visit(gd)))
                }
            }
            ConstraintGoalDefinition::Forall(vars, body) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    self.visit(&**body),
                ],
            ),
            ConstraintGoalDefinition::AtEnd(gd) => {
                self.sexpr("at", [RcDoc::text("end"), self.visit(gd)])
            }
            ConstraintGoalDefinition::Always(gd) => self.sexpr("always", [self.visit(gd)]),
            ConstraintGoalDefinition::Sometime(gd) => self.sexpr("sometime", [self.visit(gd)]),
            ConstraintGoalDefinition::Within(n, gd) => {
                self.sexpr("within", [n.accept(self), self.visit(gd)])
            }
            ConstraintGoalDefinition::AtMostOnce(gd) => {
                self.sexpr("at-most-once", [self.visit(gd)])
            }
            ConstraintGoalDefinition::SometimeAfter(a, b) => {
                self.sexpr("sometime-after", [self.visit(a), self.visit(b)])
            }
            ConstraintGoalDefinition::SometimeBefore(a, b) => {
                self.sexpr("sometime-before", [self.visit(a), self.visit(b)])
            }
            ConstraintGoalDefinition::AlwaysWithin(n, a, b) => self.sexpr(
                "always-within",
                [n.accept(self), self.visit(a), self.visit(b)],
            ),
            ConstraintGoalDefinition::HoldDuring(s, e, gd) => self.sexpr(
                "hold-during",
                [s.accept(self), e.accept(self), self.visit(gd)],
            ),
            ConstraintGoalDefinition::HoldAfter(n, gd) => {
                self.sexpr("hold-after", [n.accept(self), self.visit(gd)])
            }
        }
    }
}

impl Visitor<ConstraintGoalDefinitionInner, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ConstraintGoalDefinitionInner) -> RcDoc<'static> {
        match value {
            ConstraintGoalDefinitionInner::Goal(gd) => self.visit(gd),
            ConstraintGoalDefinitionInner::Nested(gd) => self.visit(&**gd),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;
    use crate::{
        AtomicFormula, BinaryComparison, FluentComparison, FluentExpression, Literal, Name, Number,
        Predicate, Term, Variable,
    };

    fn make_simple_gd() -> GoalDefinition {
        let af: AtomicFormula<Term> = AtomicFormula::predicate(
            Predicate::string("at"),
            vec![
                Term::new_variable(Variable::from("r")),
                Term::new_name(Name::new("loc1")),
            ],
        );
        GoalDefinition::atomic_formula(af)
    }

    #[test]
    fn goal_atomic_formula_works() {
        let gd = make_simple_gd();
        assert_eq!(prettify!(gd, 40), "(at ?r loc1)");
    }

    #[test]
    fn goal_and_works() {
        let af: AtomicFormula<Term> = AtomicFormula::predicate(
            Predicate::string("at"),
            vec![
                Term::new_variable(Variable::from("r")),
                Term::new_name(Name::new("loc1")),
            ],
        );
        let gd = GoalDefinition::and([
            GoalDefinition::atomic_formula(af.clone()),
            GoalDefinition::atomic_formula(af),
        ]);
        assert_eq!(prettify!(gd, 40), "(and (at ?r loc1) (at ?r loc1))");
    }

    #[test]
    fn goal_and_empty_works() {
        let gd = GoalDefinition::and(Vec::<GoalDefinition>::new());
        assert_eq!(prettify!(gd, 40), "(and)");
    }

    #[test]
    fn goal_or_works() {
        let af: AtomicFormula<Term> = AtomicFormula::predicate(
            Predicate::string("at"),
            vec![
                Term::new_variable(Variable::from("r")),
                Term::new_name(Name::new("loc1")),
            ],
        );
        let gd = GoalDefinition::or([
            GoalDefinition::atomic_formula(af.clone()),
            GoalDefinition::atomic_formula(af),
        ]);
        assert_eq!(prettify!(gd, 40), "(or (at ?r loc1) (at ?r loc1))");
    }

    #[test]
    fn goal_not_works() {
        let af: AtomicFormula<Term> = AtomicFormula::predicate(
            Predicate::string("at"),
            vec![
                Term::new_variable(Variable::from("r")),
                Term::new_name(Name::new("loc1")),
            ],
        );
        let gd = GoalDefinition::not(GoalDefinition::atomic_formula(af));
        assert_eq!(prettify!(gd, 40), "(not (at ?r loc1))");
    }

    #[test]
    fn goal_imply_works() {
        let af: AtomicFormula<Term> = AtomicFormula::predicate(
            Predicate::string("at"),
            vec![
                Term::new_variable(Variable::from("r")),
                Term::new_name(Name::new("loc1")),
            ],
        );
        let gd = GoalDefinition::imply(
            GoalDefinition::atomic_formula(af.clone()),
            GoalDefinition::atomic_formula(af),
        );
        assert_eq!(prettify!(gd, 40), "(imply (at ?r loc1) (at ?r loc1))");
    }

    #[test]
    fn goal_fcomp_works() {
        let a = FluentExpression::number(crate::Number::from(3));
        let b = FluentExpression::number(crate::Number::from(4));
        let fc = FluentComparison::new(BinaryComparison::GreaterThan, a, b);
        let gd = GoalDefinition::fluent_comparison(fc);
        assert_eq!(prettify!(gd, 10), "(> 3 4)");
    }

    #[test]
    fn preference_gd_goal_works() {
        let gd = make_simple_gd();
        let pgd = PreferenceGoalDefinition::from_gd(gd);
        assert_eq!(prettify!(pgd, 40), "(at ?r loc1)");
    }

    #[test]
    fn preference_gd_preference_works() {
        use crate::PreferenceName;
        let gd = make_simple_gd();
        let pref = Preference::new(Some(PreferenceName::string("p1")), gd);
        let pgd = PreferenceGoalDefinition::from_preference(pref);
        assert_eq!(prettify!(pgd, 40), "(preference p1 (at ?r loc1))");
    }

    #[test]
    fn preference_no_name_works() {
        let gd = make_simple_gd();
        let pref = Preference::new(None, gd);
        assert_eq!(prettify!(pref, 40), "(preference (at ?r loc1))");
    }

    #[test]
    fn con_gd_and_works() {
        let gd = make_simple_gd();
        let con = ConstraintGoalDefinition::and([ConstraintGoalDefinition::at_end(gd)]);
        assert_eq!(prettify!(con, 40), "(and (at end (at ?r loc1)))");
    }

    #[test]
    fn con_gd_at_end_works() {
        let gd = make_simple_gd();
        let con = ConstraintGoalDefinition::at_end(gd);
        assert_eq!(prettify!(con, 40), "(at end (at ?r loc1))");
    }

    #[test]
    fn con_gd_always_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDefinitionInner::goal(gd);
        let con = ConstraintGoalDefinition::always(con2);
        assert_eq!(prettify!(con, 40), "(always (at ?r loc1))");
    }

    #[test]
    fn con_gd_within_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDefinitionInner::goal(gd);
        let con = ConstraintGoalDefinition::within(Number::from(10), con2);
        assert_eq!(prettify!(con, 40), "(within 10 (at ?r loc1))");
    }

    #[test]
    fn con_gd_sometime_after_works() {
        let gd1 = make_simple_gd();
        let gd2 = make_simple_gd();
        let con = ConstraintGoalDefinition::sometime_after(
            ConstraintGoalDefinitionInner::goal(gd1),
            ConstraintGoalDefinitionInner::goal(gd2),
        );
        assert_eq!(
            prettify!(con, 40),
            "(sometime-after (at ?r loc1) (at ?r\n        loc1))"
        );
    }

    #[test]
    fn con_gd_hold_during_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDefinitionInner::goal(gd);
        let con = ConstraintGoalDefinition::hold_during(Number::from(5), Number::from(15), con2);
        assert_eq!(prettify!(con, 40), "(hold-during 5 15 (at ?r loc1))");
    }

    #[test]
    fn goal_literal_works() {
        let af: AtomicFormula<Term> = AtomicFormula::predicate(
            Predicate::string("at"),
            vec![Term::new_name(Name::new("x"))],
        );
        let lit = Literal::new(af);
        let gd = GoalDefinition::literal(lit);
        assert_eq!(prettify!(gd, 20), "(at x)");
    }

    #[test]
    fn goal_or_empty_works() {
        let gd = GoalDefinition::or(Vec::<GoalDefinition>::new());
        assert_eq!(prettify!(gd, 20), "(or)");
    }

    #[test]
    fn goal_exists_works() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::string("x").to_typed(Type::OBJECT)].into();
        let inner = make_simple_gd();
        let gd = GoalDefinition::exists(vars, inner);
        assert_eq!(prettify!(gd, 40), "(exists (?x) (at ?r loc1))");
    }

    #[test]
    fn goal_forall_works() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::string("x").to_typed(Type::OBJECT)].into();
        let inner = make_simple_gd();
        let gd = GoalDefinition::forall(vars, inner);
        assert_eq!(prettify!(gd, 40), "(forall (?x) (at ?r loc1))");
    }

    #[test]
    fn precondition_goal_definition_preference_works() {
        use crate::PreferenceGoalDefinition;
        let gd = make_simple_gd();
        let pref_gd = PreferenceGoalDefinition::from_gd(gd);
        let pre_gd = PreconditionGoalDefinition::preference(pref_gd);
        assert_eq!(prettify!(pre_gd, 40), "(at ?r loc1)");
    }

    #[test]
    fn precondition_goal_definition_forall_works() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::string("x").to_typed(Type::OBJECT)].into();
        let inner = PreconditionGoalDefinitions::default();
        let pre_gd = PreconditionGoalDefinition::forall(vars, inner);
        // The visitor renders as "forall (?x)" with the body being empty PreconditionGoalDefinitions rendering as (and)
        let out = prettify!(pre_gd, 40);
        assert!(out.contains("forall"));
        assert!(out.contains("?x"));
    }

    #[test]
    fn precondition_goal_definitions_empty_works() {
        let pgds = PreconditionGoalDefinitions::default();
        assert_eq!(prettify!(pgds, 20), "(and)");
    }

    #[test]
    fn precondition_goal_definitions_single_works() {
        let gd = make_simple_gd();
        let pre_gd = PreconditionGoalDefinition::preference(PreferenceGoalDefinition::from_gd(gd));
        let pgds = PreconditionGoalDefinitions::new(vec![pre_gd]);
        assert_eq!(prettify!(pgds, 40), "(at ?r loc1)");
    }

    #[test]
    fn precondition_goal_definitions_multi_works() {
        let gd1 = make_simple_gd();
        let gd2 = make_simple_gd();
        let pgds = PreconditionGoalDefinitions::new(vec![
            PreconditionGoalDefinition::preference(PreferenceGoalDefinition::from_gd(gd1)),
            PreconditionGoalDefinition::preference(PreferenceGoalDefinition::from_gd(gd2)),
        ]);
        assert_eq!(prettify!(pgds, 40), "(and (at ?r loc1) (at ?r loc1))");
    }

    #[test]
    fn pref_con_gd_forall_works() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::string("x").to_typed(Type::OBJECT)].into();
        let _inner = ConstraintGoalDefinition::and(Vec::new());
        let pgds = PreferenceConstraintGoalDefinitions::new(vec![
            PreferenceConstraintGoalDefinition::goal(ConstraintGoalDefinition::always(
                ConstraintGoalDefinitionInner::goal(make_simple_gd()),
            )),
        ]);
        let pgd = PreferenceConstraintGoalDefinition::forall(vars, pgds);
        assert_eq!(prettify!(pgd, 40), "(forall (?x) (always (at ?r loc1)))");
    }

    #[test]
    fn pref_con_gd_preference_with_name_works() {
        use crate::PreferenceName;
        let gd = make_simple_gd();
        let con = ConstraintGoalDefinition::at_end(gd);
        let pcgd =
            PreferenceConstraintGoalDefinition::preference(Some(PreferenceName::string("p1")), con);
        assert_eq!(prettify!(pcgd, 40), "(preference p1 (at end (at ?r loc1)))");
    }

    #[test]
    fn pref_con_gd_preference_no_name_works() {
        let gd = make_simple_gd();
        let con = ConstraintGoalDefinition::at_end(gd);
        let pcgd = PreferenceConstraintGoalDefinition::preference(None, con);
        assert_eq!(prettify!(pcgd, 40), "(preference (at end (at ?r loc1)))");
    }

    #[test]
    fn pref_con_gds_multi_works() {
        let gd1 = make_simple_gd();
        let gd2 = make_simple_gd();
        let pgds = PreferenceConstraintGoalDefinitions::new(vec![
            PreferenceConstraintGoalDefinition::goal(ConstraintGoalDefinition::at_end(gd1)),
            PreferenceConstraintGoalDefinition::goal(ConstraintGoalDefinition::at_end(gd2)),
        ]);
        let out = prettify!(pgds, 40);
        assert!(out.contains("(at end"));
    }

    #[test]
    fn con_gd_sometime_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDefinitionInner::goal(gd);
        let con = ConstraintGoalDefinition::sometime(con2);
        assert_eq!(prettify!(con, 40), "(sometime (at ?r loc1))");
    }

    #[test]
    fn con_gd_at_most_once_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDefinitionInner::goal(gd);
        let con = ConstraintGoalDefinition::at_most_once(con2);
        assert_eq!(prettify!(con, 40), "(at-most-once (at ?r loc1))");
    }

    #[test]
    fn con_gd_sometime_before_works() {
        let gd1 = make_simple_gd();
        let gd2 = make_simple_gd();
        let con = ConstraintGoalDefinition::sometime_before(
            ConstraintGoalDefinitionInner::goal(gd1),
            ConstraintGoalDefinitionInner::goal(gd2),
        );
        assert_eq!(
            prettify!(con, 40),
            "(sometime-before (at ?r loc1) (at ?r\n        loc1))"
        );
    }

    #[test]
    fn con_gd_always_within_works() {
        let gd1 = make_simple_gd();
        let gd2 = make_simple_gd();
        let con = ConstraintGoalDefinition::always_within(
            Number::from(10),
            ConstraintGoalDefinitionInner::goal(gd1),
            ConstraintGoalDefinitionInner::goal(gd2),
        );
        assert_eq!(
            prettify!(con, 40),
            "(always-within 10 (at ?r loc1) (at ?r\n        loc1))"
        );
    }

    #[test]
    fn con_gd_hold_after_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDefinitionInner::goal(gd);
        let con = ConstraintGoalDefinition::hold_after(Number::from(15), con2);
        assert_eq!(prettify!(con, 40), "(hold-after 15 (at ?r loc1))");
    }

    #[test]
    fn con_gd_forall_works() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::string("x").to_typed(Type::OBJECT)].into();
        let inner = ConstraintGoalDefinition::at_end(make_simple_gd());
        let con = ConstraintGoalDefinition::forall(vars, inner);
        assert_eq!(prettify!(con, 40), "(forall (?x) (at end (at ?r loc1)))");
    }

    #[test]
    fn con2_gd_nested_works() {
        let gd = make_simple_gd();
        let con = ConstraintGoalDefinition::at_end(gd);
        let con2 = ConstraintGoalDefinitionInner::nested(con);
        assert_eq!(prettify!(con2, 40), "(at end (at ?r loc1))");
    }
}
