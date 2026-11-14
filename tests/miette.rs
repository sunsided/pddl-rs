//! Parse errors render as `miette` reports that mark the offending input.

use assert2::check;
use miette::{GraphicalReportHandler, GraphicalTheme, Report};
use pddl::{Domain, Parser};

/// Parses `input` as a domain and renders the error as a report, without colours,
/// so the output is the same on every terminal.
fn render(input: &'static str) -> String {
    let error = Domain::from_str(input).expect_err("the input has a mistake");
    let report = Report::new(error).with_source_code(input);
    let handler =
        GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor()).with_width(100);
    let mut out = String::new();
    handler
        .render_report(&mut out, report.as_ref())
        .expect("rendering into a String doesn't fail");
    out
}

#[test]
fn an_unknown_section_is_marked_with_its_reason_and_a_suggestion() {
    let out = render("(define (domain d)\n  (:requirements :strips)\n  (:predicats (p ?x)))");
    check!(
        out == "\
pddl::parse

  × line 3, column 3: unknown section: unexpected `(:predicats (p ?x)))`
   ╭─[3:3]
 2 │   (:requirements :strips)
 3 │   (:predicats (p ?x)))
   ·   ─────┬─────
   ·        ╰── unknown section
   ╰────
  help: did you mean `:predicates`?
"
    );
}

#[test]
fn a_mistake_without_a_reason_is_marked_as_unexpected() {
    let out = render(
        "(define (domain d)\n  (:predicates (p ?x))\n  (:action a\n    :parameters (?x - )\n    :effect (p ?x)))",
    );
    check!(
        out == "\
pddl::parse

  × line 4, column 23: in `:parameters`: unexpected `)     :effect (p ?x)))`
   ╭─[4:23]
 3 │   (:action a
 4 │     :parameters (?x - )
   ·                       ┬
   ·                       ╰── unexpected
 5 │     :effect (p ?x)))
   ╰────
"
    );
}

/// The error is at the end of the input, which a report can't mark, so the
/// report marks the `(` that is never closed instead.
#[test]
fn a_missing_closing_parenthesis_marks_the_unclosed_one() {
    let out = render(
        "(define (domain d)\n  (:predicates (p ?x))\n  (:action a :parameters () :effect (p ?x))\n",
    );
    check!(
        out == "\
pddl::parse

  × line 4, column 1: expected `)`: unexpected end of input
   ╭─[4:1]
 1 │ (define (domain d)
   · ┬
   · ╰── this `(` is never closed
 2 │   (:predicates (p ?x))
 3 │   (:action a :parameters () :effect (p ?x))
   ╰────
"
    );
}

/// The innermost unclosed `(` is marked, not the outermost.
#[test]
fn the_innermost_unclosed_parenthesis_is_marked() {
    let out = render("(define (domain d)\n  (:predicates (p ?x)");
    check!(out.contains("2 │   (:predicates (p ?x)\n   ·   ┬\n   ·   ╰── this `(` is never closed"));
}

/// Also when the grammar, not the reader, finds the input ending too early.
#[test]
fn an_unclosed_parenthesis_is_marked_when_a_form_is_cut_short() {
    let out = render("(define (domain d)\n  (:action a :parameters () :effect");
    check!(out.contains(
        "2 │   (:action a :parameters () :effect\n   ·   ┬\n   ·   ╰── this `(` is never closed"
    ));
}
