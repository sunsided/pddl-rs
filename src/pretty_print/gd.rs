use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    Con2GD, ConGD, GoalDefinition, PreconditionGoalDefinition, PreconditionGoalDefinitions,
    PrefConGD, PrefConGDs, Preference, PreferenceGD,
};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for GoalDefinition {}
impl sealed::Sealed for PreconditionGoalDefinition {}
impl sealed::Sealed for PreconditionGoalDefinitions {}
impl sealed::Sealed for PreferenceGD {}
impl sealed::Sealed for Preference {}
impl sealed::Sealed for PrefConGD {}
impl sealed::Sealed for PrefConGDs {}
impl sealed::Sealed for ConGD {}
impl sealed::Sealed for Con2GD {}

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
            GoalDefinition::FComp(fc) => fc.accept(self),
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

impl Visitor<PreferenceGD, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreferenceGD) -> RcDoc<'static> {
        match value {
            PreferenceGD::Goal(gd) => self.visit(gd),
            PreferenceGD::Preference(pref) => self.visit(pref),
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

impl Visitor<PrefConGD, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PrefConGD) -> RcDoc<'static> {
        match value {
            PrefConGD::Goal(gd) => self.visit(gd),
            PrefConGD::Forall(vars, gds) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    RcDoc::intersperse(gds.iter().map(|gd| self.visit(gd)), RcDoc::softline()),
                ],
            ),
            PrefConGD::Preference(name, gd) => {
                let children: Vec<RcDoc<'static>> = match name {
                    Some(n) => vec![n.accept(self), self.visit(gd)],
                    None => vec![self.visit(gd)],
                };
                self.sexpr("preference", children)
            }
        }
    }
}

impl Visitor<PrefConGDs, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PrefConGDs) -> RcDoc<'static> {
        RcDoc::intersperse(value.iter().map(|gd| self.visit(gd)), RcDoc::softline())
    }
}

impl Visitor<ConGD, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ConGD) -> RcDoc<'static> {
        match value {
            ConGD::And(items) => {
                if items.is_empty() {
                    RcDoc::text("(and)")
                } else {
                    self.sexpr("and", items.iter().map(|gd| self.visit(gd)))
                }
            }
            ConGD::Forall(vars, body) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    self.visit(&**body),
                ],
            ),
            ConGD::AtEnd(gd) => self.sexpr("at", [RcDoc::text("end"), self.visit(gd)]),
            ConGD::Always(gd) => self.sexpr("always", [self.visit(gd)]),
            ConGD::Sometime(gd) => self.sexpr("sometime", [self.visit(gd)]),
            ConGD::Within(n, gd) => self.sexpr("within", [n.accept(self), self.visit(gd)]),
            ConGD::AtMostOnce(gd) => self.sexpr("at-most-once", [self.visit(gd)]),
            ConGD::SometimeAfter(a, b) => {
                self.sexpr("sometime-after", [self.visit(a), self.visit(b)])
            }
            ConGD::SometimeBefore(a, b) => {
                self.sexpr("sometime-before", [self.visit(a), self.visit(b)])
            }
            ConGD::AlwaysWithin(n, a, b) => self.sexpr(
                "always-within",
                [n.accept(self), self.visit(a), self.visit(b)],
            ),
            ConGD::HoldDuring(s, e, gd) => self.sexpr(
                "hold-during",
                [s.accept(self), e.accept(self), self.visit(gd)],
            ),
            ConGD::HoldAfter(n, gd) => self.sexpr("hold-after", [n.accept(self), self.visit(gd)]),
        }
    }
}

impl Visitor<Con2GD, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Con2GD) -> RcDoc<'static> {
        match value {
            Con2GD::Goal(gd) => self.visit(gd),
            Con2GD::Nested(gd) => self.visit(&**gd),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;
    use crate::{AtomicFormula, BinaryComp, FComp, FExp, Name, Number, Predicate, Term, Variable};

    fn make_simple_gd() -> GoalDefinition {
        let af: AtomicFormula<Term> = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![
                Term::new_variable(Variable::from("r")),
                Term::new_name(Name::new("loc1")),
            ],
        );
        GoalDefinition::new_atomic_formula(af)
    }

    #[test]
    fn goal_atomic_formula_works() {
        let gd = make_simple_gd();
        assert_eq!(prettify!(gd, 40), "(at ?r loc1)");
    }

    #[test]
    fn goal_and_works() {
        let af: AtomicFormula<Term> = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![
                Term::new_variable(Variable::from("r")),
                Term::new_name(Name::new("loc1")),
            ],
        );
        let gd = GoalDefinition::new_and([
            GoalDefinition::new_atomic_formula(af.clone()),
            GoalDefinition::new_atomic_formula(af),
        ]);
        assert_eq!(prettify!(gd, 40), "(and (at ?r loc1) (at ?r loc1))");
    }

    #[test]
    fn goal_and_empty_works() {
        let gd = GoalDefinition::new_and(Vec::<GoalDefinition>::new());
        assert_eq!(prettify!(gd, 40), "(and)");
    }

    #[test]
    fn goal_or_works() {
        let af: AtomicFormula<Term> = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![
                Term::new_variable(Variable::from("r")),
                Term::new_name(Name::new("loc1")),
            ],
        );
        let gd = GoalDefinition::new_or([
            GoalDefinition::new_atomic_formula(af.clone()),
            GoalDefinition::new_atomic_formula(af),
        ]);
        assert_eq!(prettify!(gd, 40), "(or (at ?r loc1) (at ?r loc1))");
    }

    #[test]
    fn goal_not_works() {
        let af: AtomicFormula<Term> = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![
                Term::new_variable(Variable::from("r")),
                Term::new_name(Name::new("loc1")),
            ],
        );
        let gd = GoalDefinition::new_not(GoalDefinition::new_atomic_formula(af));
        assert_eq!(prettify!(gd, 40), "(not (at ?r loc1))");
    }

    #[test]
    fn goal_imply_works() {
        let af: AtomicFormula<Term> = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![
                Term::new_variable(Variable::from("r")),
                Term::new_name(Name::new("loc1")),
            ],
        );
        let gd = GoalDefinition::new_imply(
            GoalDefinition::new_atomic_formula(af.clone()),
            GoalDefinition::new_atomic_formula(af),
        );
        assert_eq!(prettify!(gd, 40), "(imply (at ?r loc1) (at ?r loc1))");
    }

    #[test]
    fn goal_fcomp_works() {
        let a = FExp::new_number(crate::Number::from(3));
        let b = FExp::new_number(crate::Number::from(4));
        let fc = FComp::new(BinaryComp::GreaterThan, a, b);
        let gd = GoalDefinition::new_f_comp(fc);
        assert_eq!(prettify!(gd, 10), "(> 3 4)");
    }

    #[test]
    fn preference_gd_goal_works() {
        let gd = make_simple_gd();
        let pgd = PreferenceGD::from_gd(gd);
        assert_eq!(prettify!(pgd, 40), "(at ?r loc1)");
    }

    #[test]
    fn preference_gd_preference_works() {
        use crate::PreferenceName;
        let gd = make_simple_gd();
        let pref = Preference::new(Some(PreferenceName::new_string("p1")), gd);
        let pgd = PreferenceGD::from_preference(pref);
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
        let con = ConGD::new_and([ConGD::new_at_end(gd)]);
        assert_eq!(prettify!(con, 40), "(and (at end (at ?r loc1)))");
    }

    #[test]
    fn con_gd_at_end_works() {
        let gd = make_simple_gd();
        let con = ConGD::new_at_end(gd);
        assert_eq!(prettify!(con, 40), "(at end (at ?r loc1))");
    }

    #[test]
    fn con_gd_always_works() {
        let gd = make_simple_gd();
        let con2 = Con2GD::new_goal(gd);
        let con = ConGD::new_always(con2);
        assert_eq!(prettify!(con, 40), "(always (at ?r loc1))");
    }

    #[test]
    fn con_gd_within_works() {
        let gd = make_simple_gd();
        let con2 = Con2GD::new_goal(gd);
        let con = ConGD::new_within(Number::from(10), con2);
        assert_eq!(prettify!(con, 40), "(within 10 (at ?r loc1))");
    }

    #[test]
    fn con_gd_sometime_after_works() {
        let gd1 = make_simple_gd();
        let gd2 = make_simple_gd();
        let con = ConGD::new_sometime_after(Con2GD::new_goal(gd1), Con2GD::new_goal(gd2));
        assert_eq!(
            prettify!(con, 40),
            "(sometime-after (at ?r loc1) (at ?r\n        loc1))"
        );
    }

    #[test]
    fn con_gd_hold_during_works() {
        let gd = make_simple_gd();
        let con2 = Con2GD::new_goal(gd);
        let con = ConGD::new_hold_during(Number::from(5), Number::from(15), con2);
        assert_eq!(prettify!(con, 40), "(hold-during 5 15 (at ?r loc1))");
    }
}
