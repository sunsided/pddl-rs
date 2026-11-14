//! Conformance snapshot of the parser over the third-party corpus in `tests/data/`.
//!
//! Every file is parsed, pretty-printed and compared to a stored snapshot under
//! `tests/data/snapshots/`, and the printed output must parse back to the same AST.
//! Rejected files snapshot their error message instead. A parser change that alters
//! any AST, or any error message, shows up as a snapshot diff.
//!
//! Set `UPDATE_SNAPSHOTS=1` to rewrite the snapshots after an intended change.

use assert2::check;

use pddl::{Domain, Parser, PddlFile, Pretty, Problem};
use std::fs;
use std::path::Path;

const WIDTH: usize = 80;

/// Prints every domain, then every problem, and checks that each one parses back unchanged.
fn print_and_round_trip(name: &str, file: &PddlFile) -> String {
    let mut parts = Vec::new();
    for domain in &file.domains {
        let printed = domain.pretty(WIDTH).to_string();
        let reparsed = Domain::from_str(&printed)
            .unwrap_or_else(|e| panic!("{name}: printed domain does not parse: {e}\n{printed}"));
        check!(&reparsed == domain, "{name}: domain round trip\n{printed}");
        parts.push(printed);
    }
    for problem in &file.problems {
        let printed = problem.pretty(WIDTH).to_string();
        let reparsed = Problem::from_str(&printed)
            .unwrap_or_else(|e| panic!("{name}: printed problem does not parse: {e}\n{printed}"));
        check!(
            &reparsed == problem,
            "{name}: problem round trip\n{printed}"
        );
        parts.push(printed);
    }
    parts.join("\n\n") + "\n"
}

#[test]
fn corpus_matches_snapshots() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let update = std::env::var("UPDATE_SNAPSHOTS").is_ok_and(|value| value == "1");
    let mut checked = 0;
    let mut mismatches = Vec::new();
    for folder in ["vhpop", "potoroo"] {
        let mut paths: Vec<_> = fs::read_dir(root.join(folder))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension() == Some("pddl".as_ref()))
            .collect();
        paths.sort();
        for path in paths {
            let file_name = path.file_name().unwrap().to_string_lossy();
            let name = format!("{folder}/{file_name}");
            let source = fs::read_to_string(&path).unwrap();
            let (actual, extension) = match PddlFile::from_str(&source) {
                Ok(file) => (print_and_round_trip(&name, &file), "pddl"),
                Err(e) => (format!("{e}\n"), "err"),
            };
            let snapshot = root
                .join("snapshots")
                .join(folder)
                .join(file_name.as_ref())
                .with_extension(extension);
            // A file that flips between accepted and rejected changes its extension.
            let other = snapshot.with_extension(if extension == "pddl" { "err" } else { "pddl" });
            if update {
                fs::create_dir_all(snapshot.parent().unwrap()).unwrap();
                fs::write(&snapshot, &actual).unwrap();
                if other.exists() {
                    fs::remove_file(&other).unwrap();
                }
            } else {
                if other.exists() {
                    mismatches.push(format!("{name}: stale snapshot {}", other.display()));
                }
                match fs::read_to_string(&snapshot) {
                    Ok(expected) if expected == actual => {}
                    Ok(_) => {
                        mismatches.push(format!("{name}: differs from {}", snapshot.display()));
                    }
                    Err(e) => mismatches.push(format!("{name}: {}: {e}", snapshot.display())),
                }
            }
            checked += 1;
        }
    }
    check!(checked == 89, "expected 86 VHPOP and 3 Potoroo files");
    check!(
        mismatches.is_empty(),
        "{}\nRun with UPDATE_SNAPSHOTS=1 to accept, then review the diff.",
        mismatches.join("\n")
    );
}
