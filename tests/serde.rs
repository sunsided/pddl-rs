//! PDDL survives a round trip through JSON with the `serde` feature.

use assert2::check;

use pddl::{Domain, FluentExpression, Name, Number, Parser, PddlFile, Problem};
use serde_json::json;
use std::fs;
use std::path::Path;

/// Every corpus file the parser accepts serializes to JSON and deserializes back to
/// a value that serializes to the same JSON. Problems also compare equal; domains
/// may come back with their actions, durative actions and derived predicates
/// regrouped, which the JSON doesn't distinguish.
#[test]
fn corpus_round_trips_through_json() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data");
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
            let back: PddlFile = serde_json::from_str(&json)
                .unwrap_or_else(|e| panic!("{}: {e}\n{json}", path.display()));
            check!(
                serde_json::to_string(&back).unwrap() == json,
                "{}",
                path.display()
            );
            check!(back.problems == file.problems, "{}", path.display());
            checked += 1;
        }
    }
    check!(checked > 80);
}

#[test]
fn a_domain_reads_like_pddl() {
    let domain = Domain::from_str(
        "(define (domain logistics)
           (:requirements :strips :typing :action-costs)
           (:types truck location)
           (:predicates (at ?t - truck ?l - location))
           (:functions (distance ?a ?b - location) (total-cost) - number
                       (home ?t - truck) - location)
           (:action drive
             :parameters (?t - truck ?from ?to - location)
             :precondition (and (at ?t ?from) (not (= ?from ?to)))
             :effect (and (not (at ?t ?from)) (at ?t ?to)
                          (increase (total-cost) (distance ?from ?to)))))",
    )
    .unwrap();
    let expected = json!({
        "domain": "logistics",
        "requirements": ["strips", "typing", "action-costs"],
        "types": {"truck": "object", "location": "object"},
        "predicates": {"at": {"?t": "truck", "?l": "location"}},
        "functions": {
            "distance": {"?a": "location", "?b": "location"},
            "total-cost": {},
            "home": {"parameters": {"?t": "truck"}, "type": "location"}
        },
        "actions": {
            "drive": {
                "parameters": {"?t": "truck", "?from": "location", "?to": "location"},
                "precondition": {"and": [["at", "?t", "?from"], {"not": ["=", "?from", "?to"]}]},
                "effect": {"and": [
                    {"not": ["at", "?t", "?from"]},
                    ["at", "?t", "?to"],
                    {"increase": [["total-cost"], ["distance", "?from", "?to"]]}
                ]}
            }
        }
    });
    check!(serde_json::to_value(&domain).unwrap() == expected);
    check!(serde_json::from_value::<Domain>(expected).unwrap() == domain);
}

#[test]
fn a_problem_reads_like_pddl() {
    let problem = Problem::from_str(
        "(define (problem p1) (:domain logistics)
           (:objects t1 - truck a - location)
           (:init (at t1 a) (= (distance a depot) 5) (at 10 (open depot)))
           (:goal (at t1 depot))
           (:metric minimize (total-cost)))",
    )
    .unwrap();
    let expected = json!({
        "problem": "p1",
        "domain": "logistics",
        "objects": {"t1": "truck", "a": "location"},
        "init": [["at", "t1", "a"], {"=": [["distance", "a", "depot"], 5]}, {"at": [10, ["open", "depot"]]}],
        "goal": ["at", "t1", "depot"],
        "metric": {"minimize": ["total-cost"]}
    });
    check!(serde_json::to_value(&problem).unwrap() == expected);
    check!(serde_json::from_value::<Problem>(expected).unwrap() == problem);
}

#[test]
fn durative_actions_and_derived_predicates() {
    let domain = Domain::from_str(
        "(define (domain d)
           (:durative-action load
             :parameters (?t - truck)
             :duration (and (>= ?duration 1) (at end (<= ?duration (- (cap ?t) 2))))
             :condition (and (at start (at ?t depot)) (preference p (over all (free ?t))))
             :effect (and (at end (loaded ?t)) (increase (fuel ?t) (* #t 2))))
           (:derived (parked ?t - truck) (at ?t depot)))",
    )
    .unwrap();
    let expected = json!({
        "domain": "d",
        "durative-actions": {
            "load": {
                "parameters": {"?t": "truck"},
                "duration": {"and": [
                    {">=": ["?duration", 1]},
                    {"at end": {"<=": ["?duration", {"-": [["cap", "?t"], 2]}]}}
                ]},
                "condition": {"and": [
                    {"at start": ["at", "?t", "depot"]},
                    {"preference": {"over all": ["free", "?t"]}, "name": "p"}
                ]},
                "effect": {"and": [
                    {"at end": ["loaded", "?t"]},
                    {"increase": [["fuel", "?t"], {"*": ["#t", 2]}]}
                ]}
            }
        },
        "derived": [
            {"predicate": "parked", "parameters": {"?t": "truck"}, "condition": ["at", "?t", "depot"]}
        ]
    });
    check!(serde_json::to_value(&domain).unwrap() == expected);
    check!(serde_json::from_value::<Domain>(expected).unwrap() == domain);
}

#[test]
fn negation_and_subtraction_are_told_apart() {
    for pddl in [
        "(- (f ?x))",
        "(- (f ?x) 1)",
        "(- 1 (f ?x))",
        "(- 3)",
        "(+ 1 2 3)",
    ] {
        let exp = FluentExpression::from_str(pddl).unwrap();
        let json = serde_json::to_value(&exp).unwrap();
        check!(
            serde_json::from_value::<FluentExpression>(json).unwrap() == exp,
            "{pddl}"
        );
    }
}

#[test]
fn names_are_plain_strings() {
    check!(serde_json::to_string(&Name::new("At")).unwrap() == r#""At""#);
    check!(serde_json::from_str::<Name>(r#""At""#).unwrap() == Name::new("at"));
}

#[test]
fn numbers_are_numbers() {
    check!(serde_json::to_string(&Number::from(5)).unwrap() == "5");
    check!(serde_json::to_string(&Number::from(1.5)).unwrap() == "1.5");
    check!(serde_json::from_str::<Number>("1e999").is_err());
}

#[test]
fn input_cannot_change_the_pddl_structure() {
    let injected = json!({"problem": "p) (:goal (x", "domain": "d", "init": [], "goal": ["g"]});
    let error = serde_json::from_value::<Problem>(injected).unwrap_err();
    check!(error.to_string().contains("is not a valid PDDL name"));
}

#[test]
fn mistakes_are_reported() {
    let unknown = json!({"domain": "d", "action": {}});
    let error = serde_json::from_value::<Domain>(unknown).unwrap_err();
    check!(error.to_string() == "unknown key `action` in a domain");

    // The grammar still applies: `not` takes one goal, not two.
    let invalid =
        json!({"problem": "p", "domain": "d", "init": [], "goal": {"not": [["a"], ["b"]]}});
    check!(serde_json::from_value::<Problem>(invalid).is_err());
}
