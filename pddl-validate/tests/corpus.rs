//! Validates hand-written plans for the repository's test corpus (`tests/data`), and
//! cross-checks the verdict of the VAL output the VS Code extension's tests use.
//!
//! The corpus is not part of this crate's package, so neither is this file.

use std::path::PathBuf;

use pddl::{Domain, Parser, Problem};
use pddl_validate::{ModelErrorKind, Plan, Report, ValidationStatus, Validator, ViolationKind};

/// Returns the path of a file below the repository root.
fn repo(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(path)
}

/// Reads a file below the repository root.
fn read(path: &str) -> String {
    std::fs::read_to_string(repo(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// A parsed fixture.
struct Fixture {
    /// The domain.
    domain: Domain,
    /// The problem.
    problem: Problem,
}

impl Fixture {
    /// Reads `tests/data/sas-plus/<name>/{domain,problem}.pddl`.
    fn sas_plus(name: &str) -> Self {
        let dir = format!("tests/data/sas-plus/{name}");
        Self {
            domain: Domain::from_str(&read(&format!("{dir}/domain.pddl"))).unwrap(),
            problem: Problem::from_str(&read(&format!("{dir}/problem.pddl"))).unwrap(),
        }
    }

    /// Validates `plan`.
    fn run(&self, plan: &str) -> Report {
        Validator::new(&self.domain, &self.problem)
            .unwrap()
            .validate_text(plan)
    }

    /// Checks that `plan` is valid and, if `optimal`, that dropping any step makes it
    /// invalid. Swapping two adjacent steps must never panic.
    fn assert_valid(&self, plan: &str, optimal: bool) {
        let validator = Validator::new(&self.domain, &self.problem).unwrap();
        let plan: Plan = plan.parse().unwrap();
        let report = validator.validate(&plan);
        assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");

        let steps = plan.steps();
        for skip in 0..steps.len() {
            let mut dropped = steps.to_vec();
            dropped.remove(skip);
            let status = validator.validate(&Plan::new(dropped)).status();
            if optimal {
                assert_eq!(status, ValidationStatus::Invalid, "dropping step {skip}");
            }
        }

        for i in 0..steps.len().saturating_sub(1) {
            let mut swapped = steps.to_vec();
            swapped.swap(i, i + 1);
            let _ = validator.validate(&Plan::new(swapped));
        }
    }
}

#[test]
fn sussman_anomaly() {
    Fixture::sas_plus("sussman-anomaly").assert_valid(
        "(puton c table a)\n(puton b c table)\n(puton a b table)",
        true,
    );
}

#[test]
fn gripper_2() {
    Fixture::sas_plus("gripper-2").assert_valid(
        "(pick ball1 rooma left)\n(move rooma roomb)\n(drop ball1 roomb left)",
        true,
    );
}

#[test]
fn gripper_4() {
    let plan = "(pick ball1 rooma left) (pick ball2 rooma right) (move rooma roomb)
                (drop ball1 roomb left) (drop ball2 roomb right) (move roomb rooma)
                (pick ball3 rooma left) (move rooma roomb) (drop ball3 roomb left)";
    Fixture::sas_plus("gripper-4").assert_valid(plan, true);
}

#[test]
fn get_paid_4_with_conditional_effects() {
    // The briefcase carries what is in it: put the cheque in, drive to the office, take
    // it out, drive on to the bank, take out the paycheck, and drive home.
    let plan = "(put-in d home) (mov-b home office) (take-out d) (mov-b office bank)
                (take-out p) (mov-b bank home)";
    let fixture = Fixture::sas_plus("get-paid4");
    fixture.assert_valid(plan, false);

    // Without taking the paycheck out, it rides home again.
    let report = fixture.run(
        "(put-in d home) (mov-b home office) (take-out d) (mov-b office bank) (mov-b bank home)",
    );
    let violation = report.violation().unwrap();
    assert_eq!(violation.step, None);
    assert!(matches!(violation.kind, ViolationKind::GoalUnsatisfied(_)));
}

#[test]
fn bw_large_a() {
    let plan = "(put-table blockc blockb) (put-table blockb blocka) (put-table blocke blockd)
                (put blocka blocke table) (put-table blocki blockh) (put blocki blockd table)
                (put blockh blocki blockg) (put blockc blockg table) (put blockb blockc table)";
    Fixture::sas_plus("bw-large-a").assert_valid(plan, false);
}

#[test]
fn action_costs_are_level_two() {
    for name in ["gripper-costs", "courier", "wiring"] {
        let report = Fixture::sas_plus(name).run("");
        assert_eq!(report.status(), ValidationStatus::Unsupported, "{name}");
    }
}

/// The plan of the VS Code extension's VAL test and the verdict of the real VAL on it
/// (`editors/vscode/test/val-output.txt`): step 2 fails.
#[test]
fn agrees_with_val_on_the_extension_fixture() {
    let workspace = "editors/vscode/test/workspace";
    let val = read("editors/vscode/test/val-output.txt");
    assert!(val.contains("has an unsatisfied precondition at time 2"));

    let domain = Domain::from_str(&read(&format!("{workspace}/domain.pddl"))).unwrap();
    let text = read(&format!("{workspace}/problems/deliver.pddl"));
    let plan = read(&format!("{workspace}/problems/deliver.plan"));

    // VAL accepts the problem's goal `(at t1 ham)` although `ham` is not declared. This
    // validator rejects the problem instead.
    let problem = Problem::from_str(&text).unwrap();
    let error = Validator::new(&domain, &problem).unwrap_err();
    assert_eq!(error.kind, ModelErrorKind::UnknownObject("ham".into()));

    // With the goal fixed, both fail the plan at step 2. VAL reports the unsatisfied
    // precondition `(at t1 shop)`; this validator reports earlier that `shop` is not
    // declared at all.
    let problem = Problem::from_str(&text.replace("ham", "home")).unwrap();
    let report = Validator::new(&domain, &problem)
        .unwrap()
        .validate_text(&plan);
    let violation = report.violation().unwrap();
    assert_eq!(violation.step.as_ref().unwrap().number, 2);
    assert_eq!(
        violation.kind,
        ViolationKind::UnknownObject {
            parameter: "from".into(),
            object: "shop".into()
        }
    );

    // Step 1, `(drive t1 home home)`, deletes and adds `(at t1 home)`; as in VAL it stays.
    assert!(report.trace()[0].added.is_empty() && report.trace()[0].deleted.is_empty());
}
