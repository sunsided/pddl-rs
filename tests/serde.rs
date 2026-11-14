//! Parsed PDDL survives a round trip through JSON with the `serde` feature.

use assert2::check;

use pddl::{Domain, Name, Number, Parser, PddlFile};
use std::fs;
use std::path::Path;

/// Every corpus file the parser accepts serializes to JSON and deserializes back to
/// an equal value, which serializes to the same JSON (so spellings survive too, which
/// `==` alone wouldn't show: names compare ignoring case).
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
            check!(back.domains == file.domains, "{}", path.display());
            check!(back.problems == file.problems, "{}", path.display());
            check!(
                serde_json::to_string(&back).unwrap() == json,
                "{}",
                path.display()
            );
            checked += 1;
        }
    }
    check!(checked > 80);
}

#[test]
fn names_are_plain_strings() {
    check!(serde_json::to_string(&Name::new("At")).unwrap() == r#""At""#);
    check!(serde_json::from_str::<Name>(r#""At""#).unwrap() == Name::new("at"));
}

#[test]
fn non_finite_numbers_are_rejected() {
    check!(serde_json::to_string(&Number::from(1.5)).unwrap() == "1.5");
    // JSON has no NaN, but other formats do.
    check!(serde_json::from_str::<Number>("1e999").is_err());
}

/// Shows the JSON shape of a small domain, to judge it by.
#[test]
fn shape_of_a_small_domain() {
    let domain = Domain::from_str(
        "(define (domain d)
           (:requirements :strips :typing)
           (:types truck)
           (:predicates (at ?t - truck))
           (:action drive
             :parameters (?t - truck)
             :precondition (and (at ?t) (not (at ?t)))
             :effect (not (at ?t))))",
    )
    .unwrap();
    let json = serde_json::to_string_pretty(&domain).unwrap();
    println!("{json}");
    check!(json.contains(r#""drive""#));
}
