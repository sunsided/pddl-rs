//! Grounds the fixtures in `tests/data/sas-plus` (the PDDL inputs of the
//! `sas-plus` translator fixtures) and checks the results against known optima.

use pddl::sas_plus::{ground, GroundError};
use pddl::{Domain, Parser, Problem};
use sas_plus::fdr::SasFile;

fn ground_fixture(name: &str) -> Result<SasFile, GroundError> {
    let dir = format!("{}/tests/data/sas-plus/{name}", env!("CARGO_MANIFEST_DIR"));
    let read = |file: &str| std::fs::read_to_string(format!("{dir}/{file}")).unwrap();
    let domain = Domain::from_str(&read("domain.pddl")).unwrap();
    let problem = Problem::from_str(&read("problem.pddl")).unwrap();
    ground(&domain, &problem)
}

/// Grounds a fixture and checks that the output parses back to the same file.
fn ground_round_trip(name: &str) -> SasFile {
    let file = ground_fixture(name).unwrap();
    let text = file.to_string();
    assert_eq!(
        text.parse::<SasFile>().unwrap(),
        file,
        "{name} does not round-trip"
    );
    file
}

/// Checks the optimal plan cost with exhaustive search.
fn assert_optimum(name: &str, optimum: u64) {
    let task = ground_round_trip(name).to_task();
    task.validate().unwrap();
    let h_star = task.perfect_heuristic();
    assert_eq!(h_star(&task.initial), Some(optimum), "{name}");
}

#[test]
fn sussman_anomaly() {
    assert_optimum("sussman-anomaly", 3);
}

#[test]
fn gripper_2() {
    assert_optimum("gripper-2", 3);
}

#[test]
fn gripper_4() {
    assert_optimum("gripper-4", 9);
}

#[test]
fn gripper_costs() {
    let file = ground_round_trip("gripper-costs");
    assert!(file.use_metric);
    assert_optimum("gripper-costs", 7);
}

#[test]
fn conditional_effects() {
    for name in ["courier", "get-paid4"] {
        let file = ground_round_trip(name);
        assert!(
            file.operators
                .iter()
                .flat_map(|op| &op.effects)
                .any(|effect| !effect.conditions.is_empty()),
            "{name} has no conditional effects"
        );
    }
}

#[test]
fn courier_costs_come_from_static_functions() {
    let file = ground_round_trip("courier");
    let costs: Vec<_> = file
        .operators
        .iter()
        .filter(|op| op.name.starts_with("drive "))
        .map(|op| op.cost)
        .collect();
    assert_ne!(costs, [] as [u32; 0]);
    assert!(costs.iter().all(|&cost| cost > 1), "{costs:?}");
}

#[test]
fn larger_problems() {
    for name in ["bw-large-a", "logistics-e"] {
        assert!(!ground_round_trip(name).operators.is_empty(), "{name}");
    }
}

/// Fast Downward's translator finds the same operators; only the variables
/// differ, because it merges mutex atoms into multi-valued variables.
#[test]
fn operator_counts_match_fast_downward() {
    for (name, operators) in [
        ("sussman-anomaly", 18),
        ("gripper-2", 18),
        ("gripper-4", 34),
        ("bw-large-a", 648),
        ("logistics-e", 756),
        ("gripper-costs", 18),
        ("courier", 18),
        ("get-paid4", 44),
    ] {
        assert_eq!(
            ground_fixture(name).unwrap().operators.len(),
            operators,
            "{name}"
        );
    }
}

#[test]
fn derived_predicates_are_unsupported() {
    assert_eq!(
        ground_fixture("wiring"),
        Err(GroundError::Unsupported("derived predicates"))
    );
}
