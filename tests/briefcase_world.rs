//! Parses the briefcase world domain and problem and checks the resulting AST.

use assert2::check;

use pddl::{
    AtomicFormula, Domain, GoalDescription, Parser, PreconditionGoalDescription,
    PreferenceGoalDescription, Problem, TermLiteral,
};

/// The briefcase world domain from the PDDL 1.2 manual.
pub const BRIEFCASE_WORLD: &str = r"
    (define (domain briefcase-world)
      (:requirements :strips :equality :typing :conditional-effects)
      (:types location physob)
      (:constants
            B ; the briefcase
            P ; the paycheck
            D
            - physob)
      (:predicates (at ?x - physob ?y - location)
                   (in ?x ?y - physob))

      ; Move briefcase from one location to another.
      (:action mov-B
           :parameters (?m ?l - location)
           :precondition (and (at B ?m) (not (= ?m ?l)))
           :effect (and (at B ?l) (not (at B ?m))
                        (forall (?z)
                            (when (and (in ?z) (not (= ?z B)))
                                  (and (at ?z ?l) (not (at ?z ?m)))))) )

      ; Put the item in the briefcase.
      (:action put-in
           :parameters (?x - physob ?l - location)
           :precondition (not (= ?x B))     ; the item must not be the briefcase itself
           :effect (when
                 (and (at ?x ?l) (at B ?l)) ; briefcase and item are at the same location
                 (in ?x)) )                 ; ... then the item is in the briefcase.

      ; Take the item out of the briefcase.
      (:action take-out
           :parameters (?x - physob)
           :precondition (not (= ?x B))     ; the item must be the briefcase itself
           :effect (not (in ?x)) )          ; the item is not in the briefcase anymore.
    )
    ";

/// A problem for [`BRIEFCASE_WORLD`]: bring the briefcase and dictionary (`D`) to the office and leave the paycheck at home.
pub const BRIEFCASE_WORLD_PROBLEM: &str = r"
    (define (problem get-paid)
        (:domain briefcase-world)
        (:init
               ; types: locations
               (place home) (place office)
               ; types: objects
               (object p) (object d) (object b)
               ; setup
               (at B home) (at P home) (at D home) (in P))
        (:goal (and (at B office) (at D office) (at P home)))
    )
    ";

#[test]
fn parse_domain_works() {
    let domain = Domain::from_str(BRIEFCASE_WORLD).unwrap();

    // The input was parsed completely, nothing followed the domain definition.

    // All elements were parsed.
    check!(domain.name() == "briefcase-world");
    check!(domain.requirements().len() == 4);
    check!(domain.types().len() == 2);
    check!(domain.constants().len() == 3);
    check!(domain.predicates().len() == 2);
    check!(domain.structure().len() == 3);
}

#[test]
fn parse_problem_works() {
    let problem = Problem::from_str(BRIEFCASE_WORLD_PROBLEM).unwrap();

    // The input was parsed completely, nothing followed the problem definition.

    // All elements were parsed.
    check!(problem.name() == "get-paid");
    check!(problem.domain() == "briefcase-world");
    check!(problem.requirements().is_empty());
    check!(problem.init().len() == 9);
    check!(problem.goals().len() == 3);

    let mut atomic_formulas = 0;
    for goal in problem.goals() {
        match goal {
            PreconditionGoalDescription::Preference(pref) => match pref {
                PreferenceGoalDescription::Goal(goal) => match goal {
                    GoalDescription::AtomicFormula(af) => match af {
                        AtomicFormula::Predicate(_) => atomic_formulas += 1,
                        AtomicFormula::Equality(_) => {}
                    },
                    GoalDescription::Literal(literal) => match literal {
                        // The AtomicFormula variant takes precedence over the Literal variant
                        // because a Literal in the context of a Goal Description can only
                        // occur with the :negative−preconditions domain requirement.
                        TermLiteral::AtomicFormula(_) | TermLiteral::NotAtomicFormula(_) => {}
                    },
                    GoalDescription::And(_)
                    | GoalDescription::Or(_)
                    | GoalDescription::Not(_)
                    | GoalDescription::Imply(_, _)
                    | GoalDescription::Exists(_, _)
                    | GoalDescription::ForAll(_, _)
                    | GoalDescription::FluentComparison(_) => {}
                },
                PreferenceGoalDescription::Preference(_) => {}
            },
            PreconditionGoalDescription::Forall(_, _) => {}
        }
    }
    check!(atomic_formulas == 3);
}
