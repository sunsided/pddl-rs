//! PDDL survives a round trip through NOVA, and `schema/pddl.novs` describes
//! what the `serde` feature writes.

use assert2::check;

use pddl::{Domain, Parser, PddlFile, Problem};
use std::fs;
use std::path::Path;

/// The NOVA schema in `schema/pddl.novs`.
fn nova_schema() -> novafile::schema::Schema {
    novafile::schema::parse_schema(include_str!("../../schema/pddl.novs"))
        .expect("schema/pddl.novs is a valid NOVA schema")
}

/// `schema/pddl.novs` rejects a section that doesn't belong to the definition.
#[test]
fn nova_schema_checks_sections() {
    let schema = nova_schema();
    let problem_type = novafile::schema::parse_type("Problem").unwrap();
    let problem = "problem: p\ndomain: d\ninit: []\ngoal: [on, a]\n";
    let value = novafile::text::parse(problem).unwrap();
    check!(schema.resolve_as(&value, &problem_type).is_ok());

    let value = novafile::text::parse(&format!("{problem}actions: {{}}\n")).unwrap();
    check!(schema.resolve_as(&value, &problem_type).is_err());
}

/// Every form the corpus lacks round-trips through NOVA text and is valid under
/// `schema/pddl.novs`: constraints, preferences, numeric and object-fluent
/// effects, continuous effects, duration constraints and metrics.
#[test]
fn nova_schema_accepts_every_form() {
    let domain = "(define (domain d)
      (:requirements :adl :fluents :durative-actions :constraints :preferences :object-fluents)
      (:types truck place - object)
      (:constants depot - place)
      (:predicates (at ?t - truck ?p - place) (ok))
      (:functions (fuel ?t - truck) (total-cost) (home ?t - truck) - place)
      (:constraints (and (always (ok)) (sometime (at-most-once (ok)))
        (within 5 (ok)) (sometime-after (ok) (ok)) (sometime-before (ok) (ok))
        (always-within 3 (ok) (ok)) (hold-during 1 2 (ok)) (hold-after 4 (ok))
        (forall (?t - truck) (at end (at ?t depot)))))
      (:action drive
        :parameters (?t - truck ?from ?to - (either place truck))
        :precondition (and (at ?t ?from) (not (= ?from ?to)) (or (ok) (imply (ok) (ok)))
          (exists (?x - place) (at ?t ?x)) (>= (fuel ?t) (- (* 2 (fuel ?t) 3) (/ 1 (- 4))))
          (preference p1 (ok)) (forall (?y - place) (at ?t ?y)))
        :effect (and (not (at ?t ?from)) (at ?t ?to) (increase (total-cost) 1)
          (decrease (fuel ?t) (+ 1 2)) (scale-up (fuel ?t) 2) (scale-down (fuel ?t) 2)
          (assign (fuel ?t) 0) (assign (home ?t) ?to) (assign (home ?t) undefined)
          (forall (?p - place) (when (ok) (and (ok) (not (ok)))))))
      (:durative-action load
        :parameters (?t - truck)
        :duration (and (<= ?duration 5) (at start (>= ?duration (fuel ?t))))
        :condition (and (at start (ok)) (over all (ok)) (at end (ok))
          (preference p2 (at start (ok))) (forall (?p - place) (over all (at ?t ?p))))
        :effect (and (at start (not (ok))) (at end (increase (fuel ?t) (* ?duration 2)))
          (increase (total-cost) (* #t 2)) (decrease (fuel ?t) #t)
          (forall (?p - place) (at end (at ?t ?p)))
          (when (at start (ok)) (at end (ok)))))
      (:derived (parked ?t - truck) (at ?t depot)))";
    let problem = "(define (problem p) (:domain d)
      (:objects t1 - truck)
      (:init (at t1 depot) (not (ok)) (at 10 (ok)) (= (fuel t1) 5) (= (home t1) depot))
      (:goal (and (at t1 depot) (preference p3 (ok))))
      (:constraints (and (preference p4 (always (ok))) (forall (?t - truck) (sometime (ok)))))
      (:metric minimize (+ (total-cost) (* 2 (is-violated p1)) (- total-time) (fuel t1))))";
    let file = PddlFile {
        domains: vec![Domain::from_str(domain).unwrap()],
        problems: vec![Problem::from_str(problem).unwrap()],
    };
    let nova = novafile::serde::to_string(&file).unwrap();
    let back: PddlFile = novafile::serde::from_str(&nova).unwrap_or_else(|e| panic!("{e}\n{nova}"));
    check!(serde_json::to_string(&back).unwrap() == serde_json::to_string(&file).unwrap());
    let value = novafile::serde::to_value(&file).unwrap();
    if let Err(issues) = nova_schema().validate(&value) {
        panic!("not valid under schema/pddl.novs: {issues:?}\n{nova}");
    }
}

/// `schema/pddl.novs` rejects malformed formulas, not just misplaced sections.
#[test]
fn nova_schema_checks_formulas() {
    let schema = nova_schema();
    let problem_type = novafile::schema::parse_type("Problem").unwrap();
    let with_goal = |goal: &str| {
        let text = format!("problem: p\ndomain: d\ninit: []\ngoal: {goal}\n");
        schema.resolve_as(&novafile::text::parse(&text).unwrap(), &problem_type)
    };
    check!(with_goal("{imply: ([a], [b])}").is_ok());
    check!(with_goal(r#"{">=": ([f], 2)}"#).is_ok());
    // Unknown keyword, wrong arity, an effect in a goal, a number as an atom.
    check!(with_goal("{nand: [[a], [b]]}").is_err());
    check!(with_goal("{imply: ([a],)}").is_err());
    check!(with_goal("{increase: ([f], 1)}").is_err());
    check!(with_goal("5").is_err());
    check!(with_goal("[]").is_err());

    // `(and)` is no duration constraint: PDDL needs at least one.
    let domain_type = novafile::schema::parse_type("Domain").unwrap();
    let with_duration = |duration: &str| {
        let text = format!("domain: d\ndurative-actions:\n  a:\n    duration: {duration}\n");
        schema.resolve_as(&novafile::text::parse(&text).unwrap(), &domain_type)
    };
    check!(with_duration(r#"{and: [{"<=": ("?duration", 5)}]}"#).is_ok());
    check!(with_duration("{and: []}").is_err());
}

/// Every corpus file the parser accepts serializes to NOVA, deserializes back to
/// a value that serializes to the same JSON, and is valid under `schema/pddl.novs`.
#[test]
fn corpus_round_trips_through_nova() {
    let schema = nova_schema();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data");
    let mut checked = 0;
    for folder in ["vhpop", "potoroo"] {
        for entry in fs::read_dir(root.join(folder)).unwrap() {
            let path = entry.unwrap().path();
            if path.extension() != Some("pddl".as_ref()) {
                continue;
            }
            let Ok(file) = PddlFile::from_str(&fs::read_to_string(&path).unwrap()) else {
                continue;
            };
            let json = serde_json::to_string(&file).unwrap();
            let nova = novafile::serde::to_string(&file).unwrap();
            let back: PddlFile = novafile::serde::from_str(&nova)
                .unwrap_or_else(|e| panic!("{}: {e}\n{nova}", path.display()));
            check!(
                serde_json::to_string(&back).unwrap() == json,
                "{}",
                path.display()
            );
            let value = novafile::serde::to_value(&file).unwrap();
            if let Err(issues) = schema.validate(&value) {
                panic!(
                    "{}: not valid under schema/pddl.novs: {issues:?}",
                    path.display()
                );
            }
            checked += 1;
        }
    }
    check!(checked > 80);
}
