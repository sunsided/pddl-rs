//! Renders PDDL parse errors with `miette`, marking the offending input.
//!
//! Run with `cargo run --example miette --features miette`.

use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use pddl::{Domain, Parser};

/// Inputs with typical mistakes.
const INPUTS: &[&str] = &[
    "(define (domain d)\n  (:requirements :strips)\n  (:predicats (p ?x)))",
    "(define (domain d)\n  (:predicates (p ?x))\n  (:action a\n    :parameters (?x - )\n    :effect (p ?x)))",
    "(define (domain d)\n  (:predicates (p ?x))\n  (:action a :parameters (?x) :effect (p ?x))",
];

fn main() {
    let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
    for input in INPUTS {
        let error = Domain::from_str(input).expect_err("every input has a mistake");
        let report = Report::new(error).with_source_code(*input);
        let mut out = String::new();
        handler
            .render_report(&mut out, report.as_ref())
            .expect("rendering into a String doesn't fail");
        println!("{out}");
    }
}
