//! Pretty-printing of goal descriptions, preferences and constraints.

use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    ConstraintGoalDescription, ConstraintGoalDescriptionInner, GoalDescription,
    PreconditionGoalDescription, PreconditionGoalDescriptions, Preference,
    PreferenceConstraintGoalDescription, PreferenceConstraintGoalDescriptions,
    PreferenceGoalDescription,
};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for GoalDescription {}
impl sealed::Sealed for PreconditionGoalDescription {}
impl sealed::Sealed for PreconditionGoalDescriptions {}
impl sealed::Sealed for PreferenceGoalDescription {}
impl sealed::Sealed for Preference {}
impl sealed::Sealed for PreferenceConstraintGoalDescription {}
impl sealed::Sealed for PreferenceConstraintGoalDescriptions {}
impl sealed::Sealed for ConstraintGoalDescription {}
impl sealed::Sealed for ConstraintGoalDescriptionInner {}

impl Visitor<GoalDescription, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &GoalDescription) -> RcDoc<'static> {
        match value {
            GoalDescription::AtomicFormula(af) => af.accept(self),
            GoalDescription::Literal(lit) => lit.accept(self),
            GoalDescription::And(items) => {
                if items.is_empty() {
                    RcDoc::text("(and)")
                } else {
                    self.sexpr("and", items.iter().map(|gd| self.visit(gd)))
                }
            }
            GoalDescription::Or(items) => {
                if items.is_empty() {
                    RcDoc::text("(or)")
                } else {
                    self.sexpr("or", items.iter().map(|gd| self.visit(gd)))
                }
            }
            GoalDescription::Not(gd) => self.sexpr("not", [self.visit(&**gd)]),
            GoalDescription::Imply(a, b) => {
                self.sexpr("imply", [self.visit(&**a), self.visit(&**b)])
            }
            GoalDescription::Exists(vars, body) => self.sexpr(
                "exists",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    self.visit(&**body),
                ],
            ),
            GoalDescription::ForAll(vars, body) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    self.visit(&**body),
                ],
            ),
            GoalDescription::FluentComparison(fc) => fc.accept(self),
        }
    }
}

impl Visitor<PreconditionGoalDescription, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreconditionGoalDescription) -> RcDoc<'static> {
        match value {
            PreconditionGoalDescription::Preference(pref_gd) => self.visit(pref_gd),
            PreconditionGoalDescription::Forall(vars, gds) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    RcDoc::intersperse(gds.iter().map(|gd| self.visit(gd)), RcDoc::softline()),
                ],
            ),
        }
    }
}

impl Visitor<PreconditionGoalDescriptions, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreconditionGoalDescriptions) -> RcDoc<'static> {
        if value.is_empty() {
            RcDoc::text("(and)")
        } else if value.len() == 1 {
            value[0].accept(self)
        } else {
            self.sexpr("and", value.iter().map(|gd| self.visit(gd)))
        }
    }
}

impl Visitor<PreferenceGoalDescription, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreferenceGoalDescription) -> RcDoc<'static> {
        match value {
            PreferenceGoalDescription::Goal(gd) => self.visit(gd),
            PreferenceGoalDescription::Preference(pref) => self.visit(pref),
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

impl Visitor<PreferenceConstraintGoalDescription, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreferenceConstraintGoalDescription) -> RcDoc<'static> {
        match value {
            PreferenceConstraintGoalDescription::Goal(gd) => self.visit(gd),
            PreferenceConstraintGoalDescription::Forall(vars, gds) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    RcDoc::intersperse(gds.iter().map(|gd| self.visit(gd)), RcDoc::softline()),
                ],
            ),
            PreferenceConstraintGoalDescription::Preference(name, gd) => {
                let children: Vec<RcDoc<'static>> = match name {
                    Some(n) => vec![n.accept(self), self.visit(gd)],
                    None => vec![self.visit(gd)],
                };
                self.sexpr("preference", children)
            }
        }
    }
}

impl Visitor<PreferenceConstraintGoalDescriptions, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreferenceConstraintGoalDescriptions) -> RcDoc<'static> {
        RcDoc::intersperse(value.iter().map(|gd| self.visit(gd)), RcDoc::softline())
    }
}

impl Visitor<ConstraintGoalDescription, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ConstraintGoalDescription) -> RcDoc<'static> {
        match value {
            ConstraintGoalDescription::And(items) => {
                if items.is_empty() {
                    RcDoc::text("(and)")
                } else {
                    self.sexpr("and", items.iter().map(|gd| self.visit(gd)))
                }
            }
            ConstraintGoalDescription::Forall(vars, body) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    self.visit(&**body),
                ],
            ),
            ConstraintGoalDescription::AtEnd(gd) => {
                self.sexpr("at", [RcDoc::text("end"), self.visit(gd)])
            }
            ConstraintGoalDescription::Always(gd) => self.sexpr("always", [self.visit(gd)]),
            ConstraintGoalDescription::Sometime(gd) => self.sexpr("sometime", [self.visit(gd)]),
            ConstraintGoalDescription::Within(n, gd) => {
                self.sexpr("within", [n.accept(self), self.visit(gd)])
            }
            ConstraintGoalDescription::AtMostOnce(gd) => {
                self.sexpr("at-most-once", [self.visit(gd)])
            }
            ConstraintGoalDescription::SometimeAfter(a, b) => {
                self.sexpr("sometime-after", [self.visit(a), self.visit(b)])
            }
            ConstraintGoalDescription::SometimeBefore(a, b) => {
                self.sexpr("sometime-before", [self.visit(a), self.visit(b)])
            }
            ConstraintGoalDescription::AlwaysWithin(n, a, b) => self.sexpr(
                "always-within",
                [n.accept(self), self.visit(a), self.visit(b)],
            ),
            ConstraintGoalDescription::HoldDuring(s, e, gd) => self.sexpr(
                "hold-during",
                [s.accept(self), e.accept(self), self.visit(gd)],
            ),
            ConstraintGoalDescription::HoldAfter(n, gd) => {
                self.sexpr("hold-after", [n.accept(self), self.visit(gd)])
            }
        }
    }
}

impl Visitor<ConstraintGoalDescriptionInner, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ConstraintGoalDescriptionInner) -> RcDoc<'static> {
        match value {
            ConstraintGoalDescriptionInner::Goal(gd) => self.visit(gd),
            ConstraintGoalDescriptionInner::Nested(gd) => self.visit(&**gd),
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
    use assert2::check;

    fn make_simple_gd() -> GoalDescription {
        let af: AtomicFormula<Term> = AtomicFormula::predicate(
            Predicate::string("at"),
            vec![
                Term::new_variable(Variable::from("r")),
                Term::new_name(Name::new("loc1")),
            ],
        );
        GoalDescription::atomic_formula(af)
    }

    #[test]
    fn goal_atomic_formula_works() {
        let gd = make_simple_gd();
        check!(prettify!(gd, 40) == "(at ?r loc1)");
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
        let gd = GoalDescription::and([
            GoalDescription::atomic_formula(af.clone()),
            GoalDescription::atomic_formula(af),
        ]);
        check!(prettify!(gd, 40) == "(and (at ?r loc1) (at ?r loc1))");
    }

    #[test]
    fn goal_and_empty_works() {
        let gd = GoalDescription::and(Vec::<GoalDescription>::new());
        check!(prettify!(gd, 40) == "(and)");
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
        let gd = GoalDescription::or([
            GoalDescription::atomic_formula(af.clone()),
            GoalDescription::atomic_formula(af),
        ]);
        check!(prettify!(gd, 40) == "(or (at ?r loc1) (at ?r loc1))");
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
        let gd = GoalDescription::not(GoalDescription::atomic_formula(af));
        check!(prettify!(gd, 40) == "(not (at ?r loc1))");
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
        let gd = GoalDescription::imply(
            GoalDescription::atomic_formula(af.clone()),
            GoalDescription::atomic_formula(af),
        );
        check!(prettify!(gd, 40) == "(imply (at ?r loc1) (at ?r loc1))");
    }

    #[test]
    fn goal_fcomp_works() {
        let a = FluentExpression::number(crate::Number::from(3));
        let b = FluentExpression::number(crate::Number::from(4));
        let fc = FluentComparison::new(BinaryComparison::GreaterThan, a, b);
        let gd = GoalDescription::fluent_comparison(fc);
        check!(prettify!(gd, 10) == "(> 3 4)");
    }

    #[test]
    fn preference_gd_goal_works() {
        let gd = make_simple_gd();
        let pgd = PreferenceGoalDescription::from_gd(gd);
        check!(prettify!(pgd, 40) == "(at ?r loc1)");
    }

    #[test]
    fn preference_gd_preference_works() {
        use crate::PreferenceName;
        let gd = make_simple_gd();
        let pref = Preference::new(Some(PreferenceName::string("p1")), gd);
        let pgd = PreferenceGoalDescription::from_preference(pref);
        check!(prettify!(pgd, 40) == "(preference p1 (at ?r loc1))");
    }

    #[test]
    fn preference_no_name_works() {
        let gd = make_simple_gd();
        let pref = Preference::new(None, gd);
        check!(prettify!(pref, 40) == "(preference (at ?r loc1))");
    }

    #[test]
    fn con_gd_and_works() {
        let gd = make_simple_gd();
        let con = ConstraintGoalDescription::and([ConstraintGoalDescription::at_end(gd)]);
        check!(prettify!(con, 40) == "(and (at end (at ?r loc1)))");
    }

    #[test]
    fn con_gd_at_end_works() {
        let gd = make_simple_gd();
        let con = ConstraintGoalDescription::at_end(gd);
        check!(prettify!(con, 40) == "(at end (at ?r loc1))");
    }

    #[test]
    fn con_gd_always_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDescriptionInner::goal(gd);
        let con = ConstraintGoalDescription::always(con2);
        check!(prettify!(con, 40) == "(always (at ?r loc1))");
    }

    #[test]
    fn con_gd_within_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDescriptionInner::goal(gd);
        let con = ConstraintGoalDescription::within(Number::from(10), con2);
        check!(prettify!(con, 40) == "(within 10 (at ?r loc1))");
    }

    #[test]
    fn con_gd_sometime_after_works() {
        let gd1 = make_simple_gd();
        let gd2 = make_simple_gd();
        let con = ConstraintGoalDescription::sometime_after(
            ConstraintGoalDescriptionInner::goal(gd1),
            ConstraintGoalDescriptionInner::goal(gd2),
        );
        check!(prettify!(con, 40) == "(sometime-after (at ?r loc1) (at ?r\n        loc1))");
    }

    #[test]
    fn con_gd_hold_during_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDescriptionInner::goal(gd);
        let con = ConstraintGoalDescription::hold_during(Number::from(5), Number::from(15), con2);
        check!(prettify!(con, 40) == "(hold-during 5 15 (at ?r loc1))");
    }

    #[test]
    fn goal_literal_works() {
        let af: AtomicFormula<Term> = AtomicFormula::predicate(
            Predicate::string("at"),
            vec![Term::new_name(Name::new("x"))],
        );
        let lit = Literal::new(af);
        let gd = GoalDescription::literal(lit);
        check!(prettify!(gd, 20) == "(at x)");
    }

    #[test]
    fn goal_or_empty_works() {
        let gd = GoalDescription::or(Vec::<GoalDescription>::new());
        check!(prettify!(gd, 20) == "(or)");
    }

    #[test]
    fn goal_exists_works() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::string("x").to_typed(Type::OBJECT)].into();
        let inner = make_simple_gd();
        let gd = GoalDescription::exists(vars, inner);
        check!(prettify!(gd, 40) == "(exists (?x) (at ?r loc1))");
    }

    #[test]
    fn goal_forall_works() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::string("x").to_typed(Type::OBJECT)].into();
        let inner = make_simple_gd();
        let gd = GoalDescription::forall(vars, inner);
        check!(prettify!(gd, 40) == "(forall (?x) (at ?r loc1))");
    }

    #[test]
    fn precondition_goal_description_preference_works() {
        use crate::PreferenceGoalDescription;
        let gd = make_simple_gd();
        let preference = PreferenceGoalDescription::from_gd(gd);
        let precondition = PreconditionGoalDescription::preference(preference);
        check!(prettify!(precondition, 40) == "(at ?r loc1)");
    }

    #[test]
    fn precondition_goal_description_forall_works() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::string("x").to_typed(Type::OBJECT)].into();
        let inner = PreconditionGoalDescriptions::default();
        let pre_gd = PreconditionGoalDescription::forall(vars, inner);
        // The visitor renders as "forall (?x)" with the body being empty PreconditionGoalDescriptions rendering as (and)
        let out = prettify!(pre_gd, 40);
        check!(out.contains("forall"));
        check!(out.contains("?x"));
    }

    #[test]
    fn precondition_goal_descriptions_empty_works() {
        let pgds = PreconditionGoalDescriptions::default();
        check!(prettify!(pgds, 20) == "(and)");
    }

    #[test]
    fn precondition_goal_descriptions_single_works() {
        let gd = make_simple_gd();
        let pre_gd =
            PreconditionGoalDescription::preference(PreferenceGoalDescription::from_gd(gd));
        let pgds = PreconditionGoalDescriptions::new(vec![pre_gd]);
        check!(prettify!(pgds, 40) == "(at ?r loc1)");
    }

    #[test]
    fn precondition_goal_descriptions_multi_works() {
        let gd1 = make_simple_gd();
        let gd2 = make_simple_gd();
        let pgds = PreconditionGoalDescriptions::new(vec![
            PreconditionGoalDescription::preference(PreferenceGoalDescription::from_gd(gd1)),
            PreconditionGoalDescription::preference(PreferenceGoalDescription::from_gd(gd2)),
        ]);
        check!(prettify!(pgds, 40) == "(and (at ?r loc1) (at ?r loc1))");
    }

    #[test]
    fn pref_con_gd_forall_works() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::string("x").to_typed(Type::OBJECT)].into();
        let _inner = ConstraintGoalDescription::and(Vec::new());
        let pgds = PreferenceConstraintGoalDescriptions::new(vec![
            PreferenceConstraintGoalDescription::goal(ConstraintGoalDescription::always(
                ConstraintGoalDescriptionInner::goal(make_simple_gd()),
            )),
        ]);
        let pgd = PreferenceConstraintGoalDescription::forall(vars, pgds);
        check!(prettify!(pgd, 40) == "(forall (?x) (always (at ?r loc1)))");
    }

    #[test]
    fn pref_con_gd_preference_with_name_works() {
        use crate::PreferenceName;
        let gd = make_simple_gd();
        let con = ConstraintGoalDescription::at_end(gd);
        let pcgd = PreferenceConstraintGoalDescription::preference(
            Some(PreferenceName::string("p1")),
            con,
        );
        check!(prettify!(pcgd, 40) == "(preference p1 (at end (at ?r loc1)))");
    }

    #[test]
    fn pref_con_gd_preference_no_name_works() {
        let gd = make_simple_gd();
        let con = ConstraintGoalDescription::at_end(gd);
        let pcgd = PreferenceConstraintGoalDescription::preference(None, con);
        check!(prettify!(pcgd, 40) == "(preference (at end (at ?r loc1)))");
    }

    #[test]
    fn pref_con_gds_multi_works() {
        let gd1 = make_simple_gd();
        let gd2 = make_simple_gd();
        let pgds = PreferenceConstraintGoalDescriptions::new(vec![
            PreferenceConstraintGoalDescription::goal(ConstraintGoalDescription::at_end(gd1)),
            PreferenceConstraintGoalDescription::goal(ConstraintGoalDescription::at_end(gd2)),
        ]);
        let out = prettify!(pgds, 40);
        check!(out.contains("(at end"));
    }

    #[test]
    fn con_gd_sometime_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDescriptionInner::goal(gd);
        let con = ConstraintGoalDescription::sometime(con2);
        check!(prettify!(con, 40) == "(sometime (at ?r loc1))");
    }

    #[test]
    fn con_gd_at_most_once_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDescriptionInner::goal(gd);
        let con = ConstraintGoalDescription::at_most_once(con2);
        check!(prettify!(con, 40) == "(at-most-once (at ?r loc1))");
    }

    #[test]
    fn con_gd_sometime_before_works() {
        let gd1 = make_simple_gd();
        let gd2 = make_simple_gd();
        let con = ConstraintGoalDescription::sometime_before(
            ConstraintGoalDescriptionInner::goal(gd1),
            ConstraintGoalDescriptionInner::goal(gd2),
        );
        check!(prettify!(con, 40) == "(sometime-before (at ?r loc1) (at ?r\n        loc1))");
    }

    #[test]
    fn con_gd_always_within_works() {
        let gd1 = make_simple_gd();
        let gd2 = make_simple_gd();
        let con = ConstraintGoalDescription::always_within(
            Number::from(10),
            ConstraintGoalDescriptionInner::goal(gd1),
            ConstraintGoalDescriptionInner::goal(gd2),
        );
        check!(prettify!(con, 40) == "(always-within 10 (at ?r loc1) (at ?r\n        loc1))");
    }

    #[test]
    fn con_gd_hold_after_works() {
        let gd = make_simple_gd();
        let con2 = ConstraintGoalDescriptionInner::goal(gd);
        let con = ConstraintGoalDescription::hold_after(Number::from(15), con2);
        check!(prettify!(con, 40) == "(hold-after 15 (at ?r loc1))");
    }

    #[test]
    fn con_gd_forall_works() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::string("x").to_typed(Type::OBJECT)].into();
        let inner = ConstraintGoalDescription::at_end(make_simple_gd());
        let con = ConstraintGoalDescription::forall(vars, inner);
        check!(prettify!(con, 40) == "(forall (?x) (at end (at ?r loc1)))");
    }

    #[test]
    fn con2_gd_nested_works() {
        let gd = make_simple_gd();
        let con = ConstraintGoalDescription::at_end(gd);
        let con2 = ConstraintGoalDescriptionInner::nested(con);
        check!(prettify!(con2, 40) == "(at end (at ?r loc1))");
    }
}
