//! Prints a table comparing the runtime grounder, generated code (B1) and the
//! compile-time grounded task (B2): states expanded per second and the time to
//! solve the same instance with the same search.
//!
//! `cargo bench -p pddl-bench --bench compare`

use std::time::{Duration, Instant};

use pddl::{Parser, Problem};
use pddl_bench::domains::{
    blocks, bw_large_a_grounded, gripper, gripper_4_grounded, logistics, logistics_small_grounded,
};
use pddl_bench::{fixture, GroundTask};
use pddl_runtime::search::{astar, breadth_first, Outcome};
use pddl_runtime::Task;

/// Runs `search` until at least a second has passed and returns the
/// expanded states and the time of one run.
fn measure<A>(search: impl Fn() -> Outcome<A>) -> (usize, Duration) {
    let mut runs = 0u32;
    let start = Instant::now();
    let mut expanded = 0;
    while runs == 0 || start.elapsed() < Duration::from_secs(1) {
        expanded = search().expanded;
        runs += 1;
    }
    (expanded, start.elapsed() / runs)
}

/// Prints one row of the table.
fn row(instance: &str, search: &str, implementation: &str, (expanded, time): (usize, Duration)) {
    #[allow(clippy::cast_precision_loss)]
    let rate = expanded as f64 / time.as_secs_f64();
    println!(
        "| {instance} | {search} | {implementation} | {expanded} | {:.3} ms | {:.0} |",
        time.as_secs_f64() * 1e3,
        rate
    );
}

fn main() {
    println!("| instance | search | implementation | expanded | time to solve | states/s |");
    println!("|---|---|---|---:|---:|---:|");

    let (domain, problem) = fixture("gripper-4");
    let grounder = GroundTask::new(&domain, &problem).unwrap();
    let lifted = gripper::Task::new(&problem).unwrap();
    row(
        "gripper-4",
        "breadth-first",
        "runtime grounder",
        measure(|| breadth_first(&grounder)),
    );
    row(
        "gripper-4",
        "breadth-first",
        "B1 (generated)",
        measure(|| breadth_first(&lifted)),
    );
    row(
        "gripper-4",
        "breadth-first",
        "B2 (grounded at compile time)",
        measure(|| breadth_first(&gripper_4_grounded::Task)),
    );

    let (domain, problem) = fixture("bw-large-a");
    let grounder = GroundTask::new(&domain, &problem).unwrap();
    let lifted = blocks::Task::new(&problem).unwrap();
    let b2 = bw_large_a_grounded::Task;
    row(
        "bw-large-a",
        "A*, goal count",
        "runtime grounder",
        measure(|| astar(&grounder, |s| grounder.unsatisfied_goals(s))),
    );
    row(
        "bw-large-a",
        "A*, goal count",
        "B1 (generated)",
        measure(|| astar(&lifted, |s| lifted.unsatisfied_goals(s))),
    );
    row(
        "bw-large-a",
        "A*, goal count",
        "B2 (grounded at compile time)",
        measure(|| astar(&b2, |s| b2.unsatisfied_goals(s))),
    );

    let (domain, _) = fixture("logistics-e");
    let problem = Problem::from_str(include_str!("../fixtures/logistics-small.pddl")).unwrap();
    let grounder = GroundTask::new(&domain, &problem).unwrap();
    let lifted = logistics::Task::new(&problem).unwrap();
    row(
        "logistics-small",
        "breadth-first",
        "runtime grounder",
        measure(|| breadth_first(&grounder)),
    );
    row(
        "logistics-small",
        "breadth-first",
        "B1 (generated)",
        measure(|| breadth_first(&lifted)),
    );
    row(
        "logistics-small",
        "breadth-first",
        "B2 (grounded at compile time)",
        measure(|| breadth_first(&logistics_small_grounded::Task)),
    );
}
