//! Criterion benchmarks: time to solve gripper-4 with breadth-first search,
//! for the runtime grounder, generated code (B1) and the compile-time
//! grounded task (B2).
//!
//! `cargo bench -p pddl-bench --bench search`

use criterion::{criterion_group, criterion_main, Criterion};
use pddl_bench::domains::{gripper, gripper_4_grounded};
use pddl_bench::{fixture, GroundTask};
use pddl_runtime::search::breadth_first;

/// Benchmarks the three implementations on gripper-4.
fn gripper_4(c: &mut Criterion) {
    let (domain, problem) = fixture("gripper-4");
    let grounder = GroundTask::new(&domain, &problem).unwrap();
    let lifted = gripper::Task::new(&problem).unwrap();
    let mut group = c.benchmark_group("gripper-4 breadth-first");
    group.bench_function("runtime grounder", |b| b.iter(|| breadth_first(&grounder)));
    group.bench_function("B1", |b| b.iter(|| breadth_first(&lifted)));
    group.bench_function("B2", |b| {
        b.iter(|| breadth_first(&gripper_4_grounded::Task));
    });
    group.finish();
}

criterion_group!(benches, gripper_4);
criterion_main!(benches);
