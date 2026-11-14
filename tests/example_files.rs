//! Parses third-party example domains and problems from `tests/data/`.

use pddl::{Parser, PddlFile};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Files that are not valid PDDL and must be rejected, with the expected error reason.
const INVALID: &[(&str, &str)] = &[
    // The problem has no `(:init ...)` section, which the grammar requires.
    ("vhpop/durative.pddl", "missing `:init`"),
];

/// Counts `(define (<kind>` headers, ignoring case and whitespace; no file mentions one in a comment.
fn count_headers(source: &str, kind: &str) -> usize {
    let compact: String = source.to_ascii_lowercase().split_whitespace().collect();
    compact.matches(&format!("(define({kind}")).count()
}

#[test]
fn example_files_parse() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let mut checked = 0;
    let mut seen_invalid = HashSet::new();
    let mut failures = Vec::new();
    for folder in ["vhpop", "potoroo"] {
        let dir = root.join(folder);
        let entries = fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display()));
        for entry in entries {
            let path = entry
                .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
                .path();
            if path.extension() != Some("pddl".as_ref()) {
                continue;
            }
            let name = format!("{folder}/{}", path.file_name().unwrap().to_string_lossy());
            let source =
                fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let expected_error = INVALID.iter().find(|(n, _)| *n == name).map(|(_, r)| *r);
            match (PddlFile::from_str(&source), expected_error) {
                (Ok(file), None) => {
                    let expected = (
                        count_headers(&source, "domain"),
                        count_headers(&source, "problem"),
                    );
                    let parsed = (file.domain_count(), file.problem_count());
                    if parsed != expected || file.is_empty() {
                        failures.push(format!(
                            "{name}: parsed (domains, problems) {parsed:?}, expected {expected:?}"
                        ));
                    }
                }
                (Err(e), None) => failures.push(format!("{name}: {e}")),
                (Ok(_), Some(_)) => failures.push(format!("{name}: parsed but is invalid")),
                (Err(e), Some(reason)) => {
                    seen_invalid.insert(name.clone());
                    if e.reason() != Some(reason) {
                        failures.push(format!("{name}: expected `{reason}`, got: {e}"));
                    }
                }
            }
            checked += 1;
        }
    }
    for (name, _) in INVALID {
        if !seen_invalid.contains(*name) {
            failures.push(format!(
                "{name}: listed in INVALID but not rejected or not found"
            ));
        }
    }
    assert_eq!(checked, 89, "expected 86 VHPOP and 3 Potoroo files");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
