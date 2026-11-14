//! Checks every example file of the parser's corpus with the language server.
//!
//! The corpus lives in the `pddl` crate's `tests/data`, so this file is not
//! part of the published crate.

use std::path::Path;

use assert2::check;
use lsp_types::Url;
use pddl::syntax::{self, Node};
use pddl_lsp::format::format;
use pddl_lsp::line_index::Encoding;
use pddl_lsp::workspace::{Document, Workspace};

/// Problems in the corpus that are real mistakes, by file and message.
const EXPECTED: &[(&str, &str)] = &[
    // The problem has no `(:init ...)` section.
    ("durative.pddl", "missing `:init`: unexpected `) `"),
    // The corpus has these problems without their domains.
    ("gripper-40.pddl", "names from it are not checked"),
    ("logistics-e.pddl", "names from it are not checked"),
    ("logistics-f.pddl", "names from it are not checked"),
];

#[test]
fn the_corpus_has_no_unexpected_problems() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data");
    for folder in ["vhpop", "potoroo"] {
        // Each folder is its own workspace: both define domains of the same name.
        let mut workspace = Workspace::default();
        for entry in std::fs::read_dir(root.join(folder)).unwrap() {
            let path = entry.unwrap().path();
            if path.extension() != Some("pddl".as_ref()) {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            let uri = Url::from_file_path(&path).unwrap();
            workspace
                .documents
                .insert(uri, Document::new(text, Encoding::Utf8, true));
        }
        let mut found = Vec::new();
        for (uri, document) in &workspace.documents {
            let name = uri
                .path_segments()
                .unwrap()
                .next_back()
                .unwrap()
                .to_string();
            for problem in workspace.problems(uri) {
                let line = document
                    .lines
                    .position(&document.text, problem.span.start)
                    .line
                    + 1;
                let entry = format!("{folder}/{name}:{line}: {}", problem.message);
                if !EXPECTED
                    .iter()
                    .any(|(f, m)| entry.starts_with(&format!("{folder}/{f}")) && entry.ends_with(m))
                {
                    found.push(entry);
                }
            }
        }
        check!(found.is_empty(), "{}", found.join("\n"));
        // The analysis found names to check at all.
        let references = workspace
            .documents
            .values()
            .flat_map(|d| &d.analysis.references);
        check!(references.count() > 500);
    }
}

/// The atoms and comments of `text`, which formatting must keep.
fn content(text: &str) -> Vec<String> {
    fn atoms(node: &Node, out: &mut Vec<String>) {
        match node.children() {
            None => out.push(node.atom().unwrap().to_string()),
            Some(children) => {
                out.push("(".into());
                for child in children {
                    atoms(child, out);
                }
                out.push(")".into());
            }
        }
    }
    let tree = syntax::read(text).unwrap();
    let mut out = Vec::new();
    for node in &tree.nodes {
        atoms(node, &mut out);
    }
    out.extend(
        tree.comments
            .iter()
            .map(|c| text[c.clone()].trim_end().to_string()),
    );
    out
}

#[test]
fn keeps_content_and_is_idempotent_on_the_corpus() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../tests/data");
    let mut checked = 0;
    for folder in ["vhpop", "potoroo"] {
        for entry in std::fs::read_dir(root.join(folder)).unwrap() {
            let path = entry.unwrap().path();
            if path.extension() != Some("pddl".as_ref()) {
                continue;
            }
            let input = std::fs::read_to_string(&path).unwrap();
            let Some(once) = format(&input) else {
                continue;
            };
            check!(content(&once) == content(&input), "{}", path.display());
            check!(
                format(&once).as_deref() == Some(once.as_str()),
                "{}",
                path.display()
            );
            checked += 1;
        }
    }
    check!(checked > 80);
}
