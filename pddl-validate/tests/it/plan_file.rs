//! The plan file format and its errors.

use pddl_validate::{Plan, PlanErrorKind, PlanStep, ValidationStatus, ViolationKind};

use crate::classical::{MOVE_ONE, TRANSPORT};
use crate::Task;

/// Parses `text` and returns the error kind with its line and column.
fn error(text: &str) -> (PlanErrorKind, usize, usize) {
    let error = text.parse::<Plan>().expect_err("the plan is malformed");
    (error.kind, error.line, error.column)
}

#[test]
fn reads_planner_output() {
    let text = "\u{feff}; plan for p\r\n(pick ball1 rooma left)\r\n(move rooma\n roomb) (drop ball1 roomb left) ; inline\n; cost = 3 (unit cost)\n";
    let plan: Plan = text.parse().unwrap();
    let expected = [
        PlanStep::new("pick", ["ball1", "rooma", "left"]),
        PlanStep::new("move", ["rooma", "roomb"]),
        PlanStep::new("drop", ["ball1", "roomb", "left"]),
    ];
    assert_eq!(plan.len(), 3);
    for (step, expected) in plan.steps().iter().zip(&expected) {
        assert_eq!(step.action(), expected.action());
        assert_eq!(step.arguments(), expected.arguments());
    }

    let lines: Vec<usize> = plan.steps().iter().map(PlanStep::line).collect();
    assert_eq!(lines, [2, 3, 4]);
}

#[test]
fn reads_empty_plans() {
    assert!("".parse::<Plan>().unwrap().is_empty());
    assert!("; only a comment\n\n".parse::<Plan>().unwrap().is_empty());
}

#[test]
fn reports_malformed_plans_with_positions() {
    assert_eq!(error("(a b"), (PlanErrorKind::Unclosed, 1, 1));
    assert_eq!(error("(a)\n  )"), (PlanErrorKind::Unopened, 2, 3));
    assert_eq!(error("(a (b))"), (PlanErrorKind::Nested, 1, 4));
    assert_eq!(error("\n ( )"), (PlanErrorKind::EmptyStep, 2, 2));
    assert_eq!(
        error("(a)\r\nstep"),
        (PlanErrorKind::UnexpectedText("step".into()), 2, 1)
    );
    assert_eq!(error("1:"), (PlanErrorKind::DanglingTimestamp, 1, 3));
    assert_eq!(error("1: ;\n)"), (PlanErrorKind::DanglingTimestamp, 1, 3));
    assert_eq!(error("(a) [2"), (PlanErrorKind::MalformedDuration, 1, 5));
    assert_eq!(error("(a) [2 3]"), (PlanErrorKind::MalformedDuration, 1, 5));
    assert_eq!(
        error("(a) []"),
        (PlanErrorKind::InvalidNumber(String::new()), 1, 6)
    );
    assert_eq!(
        error("x1: (a)"),
        (PlanErrorKind::InvalidNumber("x1".into()), 1, 1)
    );
    assert_eq!(
        error("inf: (a)"),
        (PlanErrorKind::InvalidNumber("inf".into()), 1, 1)
    );
    assert_eq!(
        error("(a [1])"),
        (PlanErrorKind::UnexpectedText("[".into()), 1, 4)
    );
    assert_eq!(
        error("]"),
        (PlanErrorKind::UnexpectedText("]".into()), 1, 1)
    );
}

#[test]
fn reads_temporal_plans() {
    let text = "0.000: (load p1 t1 depot) [2.000]\n2.5:(drive t1 depot shop)[ 1e1 ]\n13: (unload p1 t1 shop)";
    let plan: Plan = text.parse().unwrap();
    let timing: Vec<(Option<f64>, Option<f64>)> = plan
        .steps()
        .iter()
        .map(|step| (step.time(), step.duration()))
        .collect();
    assert_eq!(
        timing,
        [
            (Some(0.0), Some(2.0)),
            (Some(2.5), Some(10.0)),
            (Some(13.0), None)
        ]
    );
    assert_eq!(plan.steps()[1].to_string(), "(drive t1 depot shop)");
}

#[test]
fn a_line_comment_hides_parentheses() {
    let plan: Plan = "; (not a step\n(a) ; ) neither".parse().unwrap();
    assert_eq!(plan.len(), 1);
}

#[test]
fn malformed_plans_are_invalid() {
    let violation = Task::new(TRANSPORT, MOVE_ONE).violation("(load p1 t1 depot");
    assert_eq!(violation.step, None);
    assert!(
        matches!(violation.kind, ViolationKind::PlanSyntax(_)),
        "{violation:?}"
    );
    assert_eq!(
        violation.to_string(),
        "cannot read the plan: line 1, column 1: `(` is never closed"
    );
}

#[test]
fn plans_built_in_code() {
    let plan: Plan = [
        PlanStep::new("load", ["p1", "t1", "depot"]),
        PlanStep::new("drive", ["t1", "depot", "shop"]),
        PlanStep::new("unload", ["p1", "t1", "shop"]),
    ]
    .into_iter()
    .collect();
    let task = Task::new(TRANSPORT, MOVE_ONE);
    let report = task.validator().validate(&plan);
    assert_eq!(report.status(), ValidationStatus::Valid);
    assert_eq!(report.trace()[0].step.as_ref().unwrap().line, 0);
}

/// A tiny deterministic generator (64-bit LCG), so the fuzz test needs no dependency.
struct Lcg(u64);

impl Lcg {
    /// Returns the next pseudo-random number.
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0 >> 33
    }
}

#[test]
fn random_text_never_panics() {
    const ALPHABET: &[char] = &[
        '(', ')', ' ', '\n', '\r', ';', 'a', 'L', '?', '-', '0', '.', ':', '[', ']', '\u{feff}',
        'é', '\t',
    ];
    let task = Task::new(TRANSPORT, MOVE_ONE);
    let validator = task.validator();
    let mut random = Lcg(7);
    for _ in 0..2000 {
        let len = random.next() % 40;
        let text: String = (0..len)
            .map(|_| ALPHABET[usize::try_from(random.next()).unwrap() % ALPHABET.len()])
            .collect();
        let _ = text.parse::<Plan>();
        let _ = validator.validate_text(&text);
    }
}

#[test]
fn a_duration_cut_off_by_the_end_of_the_file() {
    assert_eq!(
        error("(a) [ ; 2]"),
        (PlanErrorKind::MalformedDuration, 1, 5)
    );
}

#[test]
fn every_plan_error_explains_itself_with_its_position() {
    let cases = [
        ("(a", "line 1, column 1: `(` is never closed"),
        (")", "line 1, column 1: `)` has no matching `(`"),
        ("(a (b))", "line 1, column 4: a step cannot contain `(`"),
        ("()", "line 1, column 1: a step needs an action name"),
        ("x", "line 1, column 1: unexpected `x` outside a step"),
        ("1:", "line 1, column 3: a timestamp needs a step after it"),
        (
            "(a) [2",
            "line 1, column 5: a duration is one number in `[` and `]`",
        ),
        (
            "-1: (a)",
            "line 1, column 1: `-1` is not a finite, non-negative number",
        ),
    ];
    for (text, message) in cases {
        let error = text.parse::<Plan>().expect_err(text);
        assert_eq!(error.to_string(), message, "{text}");
    }
}

#[test]
fn a_lone_carriage_return_ends_a_line() {
    let plan: Plan = "(a)\r(b)\r\n(c)\n(d)".parse().unwrap();
    let lines: Vec<usize> = plan.steps().iter().map(PlanStep::line).collect();
    assert_eq!(lines, [1, 2, 3, 4]);
    assert!(!plan.is_empty());
    assert!("; no steps".parse::<Plan>().unwrap().is_empty());
}
