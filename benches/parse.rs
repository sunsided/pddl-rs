//! Parser throughput over the third-party corpus in `tests/data/`.
//!
//! Save a baseline with `cargo bench --bench parse -- --save-baseline <name>` and
//! compare a later change against it with `-- --baseline <name>`.

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use pddl::{Parser, PddlFile};
use std::fs;
use std::hint::black_box;
use std::path::Path;

/// Every corpus file the parser accepts, sorted by path.
fn corpus() -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let mut files = Vec::new();
    for folder in ["vhpop", "potoroo"] {
        for entry in fs::read_dir(root.join(folder)).unwrap() {
            let path = entry.unwrap().path();
            if path.extension() != Some("pddl".as_ref()) {
                continue;
            }
            let source = fs::read_to_string(&path).unwrap();
            if PddlFile::from_str(&source).is_ok() {
                let name = format!("{folder}/{}", path.file_name().unwrap().to_string_lossy());
                files.push((name, source));
            }
        }
    }
    files.sort();
    files
}

fn parse(c: &mut Criterion) {
    let files = corpus();
    let mut group = c.benchmark_group("parse");

    let total: usize = files.iter().map(|(_, source)| source.len()).sum();
    group.throughput(Throughput::Bytes(total as u64));
    group.bench_function("corpus", |b| {
        b.iter(|| {
            for (_, source) in &files {
                black_box(PddlFile::from_str(black_box(source)).unwrap());
            }
        })
    });

    // The largest problem and the largest domain, to see per-file cost on its own.
    for name in ["potoroo/logistics-f.pddl", "vhpop/flat-tire2-domain.pddl"] {
        let (_, source) = files.iter().find(|(n, _)| n == name).unwrap();
        group.throughput(Throughput::Bytes(source.len() as u64));
        group.bench_function(name, |b| {
            b.iter(|| black_box(PddlFile::from_str(black_box(source)).unwrap()))
        });
    }
    group.finish();
}

criterion_group!(benches, parse);
criterion_main!(benches);
