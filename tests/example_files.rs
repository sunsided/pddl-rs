//! Parses third-party example domains and problems from `tests/data/`.

use pddl::{Parser, PddlFile};
use std::fs;
use std::path::Path;

/// Files that are not valid PDDL and must be rejected.
const INVALID: &[&str] = &[
    // The problem has no `(:init ...)` section, which the grammar requires.
    "vhpop/durative.pddl",
];

#[test]
fn example_files_parse() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    let mut checked = 0;
    let mut failures = Vec::new();
    for folder in ["vhpop", "potoroo"] {
        for entry in fs::read_dir(root.join(folder)).unwrap() {
            let path = entry.unwrap().path();
            if path.extension() != Some("pddl".as_ref()) {
                continue;
            }
            let name = format!("{folder}/{}", path.file_name().unwrap().to_string_lossy());
            let source = fs::read_to_string(&path).unwrap();
            let result = PddlFile::from_str(&source);
            match (result, INVALID.contains(&name.as_str())) {
                (Ok(file), false) if !file.is_empty() => {}
                (Ok(_), false) => failures.push(format!("{name}: no definitions")),
                (Err(e), false) => failures.push(format!("{name}: {e}")),
                (Ok(_), true) => failures.push(format!("{name}: parsed but is invalid")),
                (Err(_), true) => {}
            }
            checked += 1;
        }
    }
    assert_eq!(checked, 89, "expected 86 VHPOP and 3 Potoroo files");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
