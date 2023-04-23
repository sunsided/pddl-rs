use pddl::parsers::{parse_domain, parse_problem};
use pddl::types::{AtomicFormula, GoalDefinition, PreGD, PreferenceGD, TermLiteral};

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
    let (remainder, domain) = parse_domain(BRIEFCASE_WORLD).unwrap();

    // The input was parsed completely, nothing followed the domain definition.
    assert_eq!(remainder, "");

    // All elements were parsed.
    assert_eq!(domain.name(), &"briefcase-world".into());
    assert_eq!(domain.requirements().len(), 4);
    assert_eq!(domain.types().len(), 2);
    assert_eq!(domain.constants().len(), 3);
    assert_eq!(domain.predicates().len(), 2);
    assert_eq!(domain.structure().len(), 3);
}

#[test]
fn parse_problem_works() {
    let (remainder, problem) = parse_problem(BRIEFCASE_WORLD_PROBLEM).unwrap();

    // The input was parsed completely, nothing followed the problem definition.
    assert_eq!(remainder, "");

    // All elements were parsed.
    assert_eq!(problem.name(), &"get-paid".into());
    assert_eq!(problem.domain(), &"briefcase-world".into());
    assert!(problem.requirements().is_empty());
    assert_eq!(problem.init().len(), 9);
    assert!(matches! { problem.goal(), PreGD::And(_) });

    match problem.goal() {
        PreGD::Preference(pref) => match pref {
            PreferenceGD::Goal(goal) => match goal {
                GoalDefinition::AtomicFormula(af) => match af {
                    AtomicFormula::Equality(_) => {}
                    AtomicFormula::Predicate(_) => {}
                },
                GoalDefinition::Literal(literal) => match literal {
                    TermLiteral::AtomicFormula(_) => {}
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
        PreGD::And(_) => {}
        PreGD::Forall(_, _) => {}
    }
}