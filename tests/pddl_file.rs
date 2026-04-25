use pddl::{Name, Parser, PddlFile, Problem};

pub const MULTI_DEF_FILE: &str = r#"
; Multi-definition PDDL file containing multiple domains and problems.
;
; This example demonstrates a PDDL file with several (define ...) blocks
; in alternating order, which is common in planning repositories.

(define (domain blocks-world)
  (:requirements :strips)
  (:predicates (clear ?x) (on ?x ?y) (on-table ?x) (hand-empty) (holding ?x))
  (:action pick-up
    :parameters (?b)
    :precondition (and (clear ?b) (on-table ?b) (hand-empty))
    :effect (and (not (on-table ?b)) (not (clear ?b)) (not (hand-empty)) (holding ?b)))
  (:action put-down
    :parameters (?b)
    :precondition (holding ?b)
    :effect (and (not (holding ?b)) (clear ?b) (hand-empty) (on-table ?b)))
  (:action stack
    :parameters (?b ?c)
    :precondition (and (holding ?b) (clear ?c))
    :effect (and (not (holding ?b)) (not (clear ?c)) (clear ?b) (hand-empty) (on ?b ?c)))
  (:action unstack
    :parameters (?b ?c)
    :precondition (and (on ?b ?c) (clear ?b) (hand-empty))
    :effect (and (holding ?b) (clear ?c) (not (clear ?b)) (not (hand-empty)) (not (on ?b ?c))))
)

(define (problem blocks-world-problem-1)
  (:domain blocks-world)
  (:init (on-table c) (on-table b) (on a c) (clear a) (hand-empty))
  (:goal (and (on b c) (on a b)))
)

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
       :effect (when (and (at ?x ?l) (at B ?l)) (in ?x)) )

  (:action take-out
       :parameters (?x - physob)
       :precondition (not (= ?x B))
       :effect (not (in ?x)) )
)

(define (problem briefcase-problem-1)
    (:domain briefcase-world)
    (:init (place home) (place office)
           (object p) (object d) (object b)
           (at B home) (at P home) (at D home) (in P))
    (:goal (and (at B office) (at D office) (at P home)))
)

(define (problem briefcase-problem-2)
    (:domain briefcase-world)
    (:init (place home) (place office) (place store)
           (object p) (object d) (object b)
           (at B home) (at P home) (at D home) (in P))
    (:goal (and (at B store) (at P office) (at D home)))
)

(define (problem briefcase-problem-3)
    (:domain briefcase-world)
    (:init (place office)
           (object p) (object b)
           (at B office) (at P office))
    (:goal (and (at B office) (at P office)))
)

(define (problem blocks-world-problem-2)
  (:domain blocks-world)
  (:init (on-table a) (on-table b) (on-table c) (clear a) (clear b) (clear c) (hand-empty))
  (:goal (and (on c b) (on b a)))
)
"#;

#[test]
fn parse_multi_definition_file() {
    let (remainder, file) = PddlFile::parse(MULTI_DEF_FILE).unwrap();

    assert!(remainder.is_empty());

    // 2 domains, 5 problems — alternating order in the file
    assert_eq!(file.domain_count(), 2);
    assert_eq!(file.problem_count(), 5);
    assert!(!file.is_empty());
}

#[test]
fn parse_multi_definition_file_via_from_str() {
    let file = PddlFile::from_str(MULTI_DEF_FILE).unwrap();

    assert_eq!(file.domain_count(), 2);
    assert_eq!(file.problem_count(), 5);
}

#[test]
fn domains_are_correct() {
    let file = PddlFile::from_str(MULTI_DEF_FILE).unwrap();

    // blocks-world comes first in the file
    let blocks = &file.domains[0];
    assert_eq!(blocks.name(), &Name::new("blocks-world"));
    assert_eq!(blocks.predicates().len(), 5);
    assert_eq!(blocks.structure().len(), 4);

    // briefcase-world comes second
    let briefcase = &file.domains[1];
    assert_eq!(briefcase.name(), &Name::new("briefcase-world"));
    assert_eq!(briefcase.types().len(), 2);
    assert_eq!(briefcase.constants().len(), 3);
    assert_eq!(briefcase.predicates().len(), 2);
    assert_eq!(briefcase.structure().len(), 3);
}

#[test]
fn problems_correct_order_and_domain_refs() {
    let file = PddlFile::from_str(MULTI_DEF_FILE).unwrap();
    let problems: Vec<&Problem> = file.problems.iter().collect();

    assert_eq!(problems.len(), 5);

    // Problems appear in file order — blocks-world-problem-1 before briefcase problems
    assert_eq!(problems[0].name(), &Name::new("blocks-world-problem-1"));
    assert_eq!(problems[0].domain(), &Name::new("blocks-world"));
    assert_eq!(problems[0].goals().len(), 2);

    assert_eq!(problems[1].name(), &Name::new("briefcase-problem-1"));
    assert_eq!(problems[1].domain(), &Name::new("briefcase-world"));
    assert_eq!(problems[1].goals().len(), 3);

    assert_eq!(problems[2].name(), &Name::new("briefcase-problem-2"));
    assert_eq!(problems[2].goals().len(), 3);

    assert_eq!(problems[3].name(), &Name::new("briefcase-problem-3"));
    assert_eq!(problems[3].goals().len(), 2);

    // blocks-world-problem-2 comes after all briefcase problems
    assert_eq!(problems[4].name(), &Name::new("blocks-world-problem-2"));
    assert_eq!(problems[4].domain(), &Name::new("blocks-world"));
    assert_eq!(problems[4].goals().len(), 2);
}

#[test]
fn parse_domains_only_from_multi_file() {
    let (remainder, domains) = pddl::parsers::parse_domains(MULTI_DEF_FILE).unwrap();

    assert!(remainder.is_empty());
    assert_eq!(domains.len(), 2);
    assert_eq!(domains[0].name(), &Name::new("blocks-world"));
    assert_eq!(domains[1].name(), &Name::new("briefcase-world"));
}

#[test]
fn parse_problems_only_from_multi_file() {
    let (remainder, problems) = pddl::parsers::parse_problems(MULTI_DEF_FILE).unwrap();

    assert!(remainder.is_empty());
    assert_eq!(problems.len(), 5);
    assert_eq!(problems[0].name(), &Name::new("blocks-world-problem-1"));
    assert_eq!(problems[4].name(), &Name::new("blocks-world-problem-2"));
}
