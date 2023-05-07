use pddl::{
    AtomicFormula, Domain, GoalDefinition, Parser, PreconditionGoalDefinition, PreferenceGD,
    Problem, TermLiteral,
};

pub const BRIEFCASE_WORLD: &'static str = r#"
    (define (domain briefcase-world)
      (:requirements :strips :equality :typing :conditional-effects)
      (:types location physob)
      (:constants B P D - physob)
      (:predicates (at ?x - physob ?y - location)
                   (in ?x ?y - physob))

      (:action mov-B
           :parameters (?m ?l - location)
           :precondition (and (at B ?m) (not (= ?m ?l)))
           :effect (and (at B ?l) (not (at B ?m))
                        (forall (?z)
                            (when (and (in ?z) (not (= ?z B)))
                                  (and (at ?z ?l) (not (at ?z ?m)))))) )

      (:action put-in
           :parameters (?x - physob ?l - location)
           :precondition (not (= ?x B))
           :effect (when (and (at ?x ?l) (at B ?l))
                 (in ?x)) )

      (:action take-out
           :parameters (?x - physob)
           :precondition (not (= ?x B))
           :effect (not (in ?x)) )
    )
    "#;

pub const BRIEFCASE_WORLD_PROBLEM: &'static str = r#"
    (define (problem get-paid)
        (:domain briefcase-world)
        (:init (place home) (place office)
               (object p) (object d) (object b)
               (at B home) (at P home) (at D home) (in P))
        (:goal (and (at B office) (at D office) (at P home)))
    )
    "#;

#[test]
fn parse_domain_works() {
    let (remainder, domain) = Domain::parse(BRIEFCASE_WORLD).unwrap();

    // The input was parsed completely, nothing followed the domain definition.
    assert!(remainder.is_empty());

    // All elements were parsed.
    assert_eq!(domain.name(), "briefcase-world");
    assert_eq!(domain.requirements().len(), 4);
    assert_eq!(domain.types().len(), 2);
    assert_eq!(domain.constants().len(), 3);
    assert_eq!(domain.predicates().len(), 2);
    assert_eq!(domain.structure().len(), 3);
}

#[test]
fn parse_problem_works() {
    let (remainder, problem) = Problem::parse(BRIEFCASE_WORLD_PROBLEM).unwrap();

    // The input was parsed completely, nothing followed the problem definition.
    assert!(remainder.is_empty());

    // All elements were parsed.
    assert_eq!(problem.name(), "get-paid");
    assert_eq!(problem.domain(), "briefcase-world");
    assert!(problem.requirements().is_empty());
    assert_eq!(problem.init().len(), 9);
    assert_eq!(problem.goals().len(), 3);

    let mut atomic_formulas = 0;
    for goal in problem.goals().iter() {
        match goal {
            PreconditionGoalDefinition::Preference(pref) => match pref {
                PreferenceGD::Goal(goal) => match goal {
                    GoalDefinition::AtomicFormula(af) => match af {
                        AtomicFormula::Predicate(_) => atomic_formulas += 1,
                        AtomicFormula::Equality(_) => {}
                    },
                    GoalDefinition::Literal(literal) => match literal {
                        TermLiteral::AtomicFormula(_) => {
                            // The AtomicFormula variant takes precedence over the Literal variant
                            // because a Literal in the context of a Goal Description can only
                            // occur with the :negative−preconditions domain requirement.
                        }
                        TermLiteral::NotAtomicFormula(_) => {}
                    },
                    GoalDefinition::And(_) => {}
                    GoalDefinition::Or(_) => {}
                    GoalDefinition::Not(_) => {}
                    GoalDefinition::Imply(_, _) => {}
                    GoalDefinition::Exists(_, _) => {}
                    GoalDefinition::ForAll(_, _) => {}
                    GoalDefinition::FComp(_) => {}
                },
                PreferenceGD::Preference(_) => {}
            },
            PreconditionGoalDefinition::Forall(_, _) => {}
        }
    }
    assert_eq!(atomic_formulas, 3);
}
