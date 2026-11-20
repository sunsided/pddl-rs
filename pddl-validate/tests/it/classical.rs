//! Level 1: every kind of invalid classical plan, and the semantics of effects.

use pddl_validate::{Atom, Unmet, ValidationStatus, ViolationKind};

use crate::Task;

/// A typed transport domain with every classical construct: subtypes, `either`,
/// negative preconditions, equality, `or`, `exists`, `forall`, `imply` and conditional
/// effects under `forall`.
pub const TRANSPORT: &str = "
(define (domain transport)
  (:requirements :typing :negative-preconditions :equality :disjunctive-preconditions
                 :existential-preconditions :universal-preconditions :conditional-effects)
  (:types truck van - vehicle vehicle place package - object)
  (:predicates (at ?v - vehicle ?p - place) (road ?a ?b - place)
               (in ?k - package ?v - vehicle) (pkg-at ?k - package ?p - place)
               (closed ?p - place) (fragile ?k - package))
  (:action drive
    :parameters (?v - vehicle ?from ?to - place)
    :precondition (and (at ?v ?from) (road ?from ?to) (not (= ?from ?to)) (not (closed ?to)))
    :effect (and (not (at ?v ?from)) (at ?v ?to)
                 (forall (?k - package)
                   (when (in ?k ?v) (and (not (pkg-at ?k ?from)) (pkg-at ?k ?to))))))
  (:action load
    :parameters (?k - package ?v - truck ?p - place)
    :precondition (and (at ?v ?p) (pkg-at ?k ?p)
                       (or (not (fragile ?k)) (exists (?w - van) (at ?w ?p))))
    :effect (in ?k ?v))
  (:action unload
    :parameters (?k - package ?v - (either truck van) ?p - place)
    :precondition (and (in ?k ?v) (at ?v ?p))
    :effect (not (in ?k ?v)))
  (:action lock
    :parameters (?p - place)
    :precondition (forall (?v - vehicle) (not (at ?v ?p)))
    :effect (closed ?p))
  (:action stay
    :parameters (?v - vehicle ?p - place)
    :precondition (imply (closed ?p) (at ?v ?p))
    :effect (and (not (at ?v ?p)) (at ?v ?p))))";

/// A problem for [`TRANSPORT`]: bring `p1` to the shop.
pub const MOVE_ONE: &str = "
(define (problem move-one) (:domain transport)
  (:objects t1 - truck v1 - van depot shop yard - place p1 glass - package)
  (:init (at t1 depot) (at v1 yard)
         (road depot shop) (road shop depot) (road shop yard) (road depot depot)
         (pkg-at p1 depot) (pkg-at glass depot) (fragile glass))
  (:goal (and (pkg-at p1 shop) (not (in p1 t1)))))";

/// An optimal plan for [`MOVE_ONE`].
pub const MOVE_ONE_PLAN: &str = "(load p1 t1 depot)\n(drive t1 depot shop)\n(unload p1 t1 shop)\n";

/// The transport task.
fn transport() -> Task {
    Task::new(TRANSPORT, MOVE_ONE)
}

/// An atom from a PDDL-like list of names.
fn atom(symbol: &str, arguments: &[&str]) -> Atom {
    Atom::new(symbol, arguments)
}

#[test]
fn valid_plan_with_trace() {
    let report = transport().run(MOVE_ONE_PLAN);
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert_eq!(report.trace().len(), 3);

    let drive = &report.trace()[1];
    assert_eq!(drive.step.as_ref().unwrap().line, 2);
    assert_eq!(
        drive.added,
        [atom("at", &["t1", "shop"]), atom("pkg-at", &["p1", "shop"])]
    );
    assert_eq!(
        drive.deleted,
        [
            atom("at", &["t1", "depot"]),
            atom("pkg-at", &["p1", "depot"])
        ]
    );
}

#[test]
fn names_are_case_insensitive() {
    let plan = "(LOAD P1 T1 Depot)\n(Drive t1 DEPOT shop)\n(unload p1 t1 SHOP)";
    assert_eq!(transport().run(plan).status(), ValidationStatus::Valid);
}

#[test]
fn unknown_action() {
    let kind = transport().step_failure("(fly t1 depot shop)", 1);
    assert_eq!(kind, ViolationKind::UnknownAction("fly".into()));
}

#[test]
fn wrong_arity() {
    let kind = transport().step_failure("(drive t1 depot)", 1);
    assert_eq!(
        kind,
        ViolationKind::Arity {
            expected: 3,
            found: 2
        }
    );
}

#[test]
fn unknown_object() {
    let kind = transport().step_failure("(drive t1 depot mars)", 1);
    assert_eq!(
        kind,
        ViolationKind::UnknownObject {
            parameter: "to".into(),
            object: "mars".into()
        }
    );
}

#[test]
fn type_mismatch() {
    let kind = transport().step_failure("(drive p1 depot shop)", 1);
    assert_eq!(
        kind,
        ViolationKind::TypeMismatch {
            parameter: "v".into(),
            object: "p1".into(),
            expected: "vehicle".into()
        }
    );

    // A van is a vehicle but not a truck.
    let kind = transport().step_failure("(load p1 v1 depot)", 1);
    assert!(
        matches!(&kind, ViolationKind::TypeMismatch { expected, .. } if expected == "truck"),
        "{kind:?}"
    );
}

#[test]
fn subtypes_and_either_types_are_accepted() {
    // `unload` takes `(either truck van)`; the van passes the type check and fails on the
    // precondition instead.
    let kind = transport().step_failure("(unload p1 v1 yard)", 1);
    assert!(
        matches!(kind, ViolationKind::PreconditionUnsatisfied(_)),
        "{kind:?}"
    );
}

#[test]
fn unsatisfied_atom_with_val_style_advice() {
    let violation = transport().violation("(drive t1 shop depot)");
    let ViolationKind::PreconditionUnsatisfied(unmet) = &violation.kind else {
        panic!("{violation:?}");
    };
    assert_eq!(
        *unmet,
        Unmet::Atom {
            atom: atom("at", &["t1", "shop"]),
            expected: true,
            derived: false
        }
    );
    assert_eq!(
        violation.advice(),
        "(drive t1 shop depot) has an unsatisfied precondition at step 1: \
         (Set (at t1 shop) to true)"
    );
}

#[test]
fn unsatisfied_negative_precondition() {
    let kind = transport().step_failure("(lock shop)\n(drive t1 depot shop)", 2);
    assert_eq!(
        kind,
        ViolationKind::PreconditionUnsatisfied(Unmet::Atom {
            atom: atom("closed", &["shop"]),
            expected: false,
            derived: false
        })
    );
}

#[test]
fn unsatisfied_inequality() {
    // `(road depot depot)` holds, so only `(not (= ?from ?to))` fails.
    let kind = transport().step_failure("(drive t1 depot depot)", 1);
    assert_eq!(
        kind,
        ViolationKind::PreconditionUnsatisfied(Unmet::Equality {
            left: "depot".into(),
            right: "depot".into(),
            expected: false
        })
    );
}

#[test]
fn unsatisfied_disjunction_lists_every_alternative() {
    // The glass is fragile and no van is at the depot.
    let kind = transport().step_failure("(load glass t1 depot)", 1);
    let ViolationKind::PreconditionUnsatisfied(Unmet::NoneOf {
        condition,
        alternatives,
        omitted,
    }) = kind
    else {
        panic!("{kind:?}");
    };
    assert_eq!(
        condition,
        "(or (not (fragile glass)) (exists (?w - van) (at ?w depot)))"
    );
    assert_eq!(omitted, 0);
    assert_eq!(
        alternatives[0],
        Unmet::Atom {
            atom: atom("fragile", &["glass"]),
            expected: false,
            derived: false
        }
    );
    let Unmet::NoneOf { alternatives, .. } = &alternatives[1] else {
        panic!("{alternatives:?}");
    };
    assert_eq!(
        alternatives.as_slice(),
        [Unmet::Atom {
            atom: atom("at", &["v1", "depot"]),
            expected: true,
            derived: false
        }]
    );
}

#[test]
fn unsatisfied_universal_blames_the_first_instance() {
    let kind = transport().step_failure("(lock depot)", 1);
    assert_eq!(
        kind,
        ViolationKind::PreconditionUnsatisfied(Unmet::Atom {
            atom: atom("at", &["t1", "depot"]),
            expected: false,
            derived: false
        })
    );
}

#[test]
fn unsatisfied_implication() {
    let kind = transport().step_failure("(lock shop)\n(stay t1 shop)", 2);
    let ViolationKind::PreconditionUnsatisfied(Unmet::NoneOf { alternatives, .. }) = kind else {
        panic!("{kind:?}");
    };
    assert_eq!(
        alternatives,
        [
            Unmet::Atom {
                atom: atom("closed", &["shop"]),
                expected: false,
                derived: false
            },
            Unmet::Atom {
                atom: atom("at", &["t1", "shop"]),
                expected: true,
                derived: false
            },
        ]
    );
}

#[test]
fn unsatisfied_goal() {
    let violation = transport().violation("(load p1 t1 depot)");
    assert_eq!(violation.step, None);
    assert_eq!(
        violation.kind,
        ViolationKind::GoalUnsatisfied(Unmet::Atom {
            atom: atom("pkg-at", &["p1", "shop"]),
            expected: true,
            derived: false
        })
    );
    assert_eq!(
        violation.advice(),
        "The goal is not satisfied: (Set (pkg-at p1 shop) to true)"
    );
}

#[test]
fn the_empty_plan_is_valid_only_if_the_goal_holds_initially() {
    assert_eq!(transport().run("").status(), ValidationStatus::Invalid);

    let reached = MOVE_ONE.replace("(pkg-at p1 shop)", "(pkg-at p1 depot)");
    let task = Task::new(TRANSPORT, &reached);
    assert_eq!(
        task.run("; nothing to do").status(),
        ValidationStatus::Valid
    );
}

#[test]
fn a_step_that_deletes_and_adds_an_atom_keeps_it() {
    let report = transport()
        .run("(stay t1 depot)\n(load p1 t1 depot)\n(drive t1 depot shop)\n(unload p1 t1 shop)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert_eq!(report.trace()[0].added, []);
    assert_eq!(report.trace()[0].deleted, []);
}

#[test]
fn the_trace_stops_before_the_failing_step() {
    let report = transport().run("(load p1 t1 depot)\n(drive t1 shop yard)\n(unload p1 t1 yard)");
    assert_eq!(report.status(), ValidationStatus::Invalid);
    assert_eq!(report.trace().len(), 1);
    let step = report.violation().unwrap().step.clone().unwrap();
    assert_eq!((step.number, step.line), (2, 2));
    assert_eq!(step.action, "(drive t1 shop yard)");
}

/// A light switch whose two conditional effects would undo each other if the second
/// saw the first one's result.
const TOGGLE: &str = "
(define (domain toggle) (:requirements :conditional-effects :negative-preconditions)
  (:predicates (on))
  (:action flip :parameters ()
    :effect (and (when (on) (not (on))) (when (not (on)) (on)))))";

#[test]
fn effect_conditions_are_evaluated_before_the_step() {
    let task = Task::new(
        TOGGLE,
        "(define (problem dark) (:domain toggle) (:init) (:goal (on)))",
    );
    assert_eq!(task.run("(flip)").status(), ValidationStatus::Valid);
    assert_eq!(task.run("(flip)(flip)").status(), ValidationStatus::Invalid);
    assert_eq!(
        task.run("(flip) (flip) (flip)").status(),
        ValidationStatus::Valid
    );
}

/// Power flows from the source along cables (a recursive derived predicate); `dark` is
/// the negation of `powered`, one stratum up.
pub const WIRING: &str = "
(define (domain wiring)
  (:requirements :strips :typing :negative-preconditions :derived-predicates)
  (:types node)
  (:predicates (source ?n - node) (duct ?a ?b - node) (cable ?a ?b - node)
               (powered ?n - node) (dark ?n - node))
  (:derived (powered ?n - node)
    (or (source ?n) (exists (?m - node) (and (cable ?m ?n) (powered ?m)))))
  (:derived (dark ?n - node) (not (powered ?n)))
  (:action lay :parameters (?a ?b - node)
    :precondition (and (duct ?a ?b) (powered ?a) (dark ?b))
    :effect (cable ?a ?b))
  (:action cut :parameters (?a ?b - node)
    :precondition (cable ?a ?b)
    :effect (not (cable ?a ?b))))";

/// A problem for [`WIRING`]: power the lamp through the junction.
pub const LAMP: &str = "
(define (problem lamp) (:domain wiring)
  (:objects plant junction lamp - node)
  (:init (source plant) (duct plant junction) (duct junction lamp))
  (:goal (and (powered lamp) (not (dark junction)))))";

#[test]
fn derived_predicates_follow_the_state() {
    let task = Task::new(WIRING, LAMP);
    let report = task.run("(lay plant junction)\n(lay junction lamp)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");

    let kind = task.step_failure("(lay junction lamp)", 1);
    let powered = Unmet::Atom {
        atom: atom("powered", &["junction"]),
        expected: true,
        derived: true,
    };
    assert_eq!(
        kind,
        ViolationKind::PreconditionUnsatisfied(powered.clone())
    );
    assert_eq!(
        powered.advice(),
        "(Make the rule of derived (powered junction) yield true)"
    );

    let kind = task.step_failure("(lay plant junction)\n(lay plant junction)", 2);
    assert_eq!(
        kind,
        ViolationKind::PreconditionUnsatisfied(Unmet::Atom {
            atom: atom("dark", &["junction"]),
            expected: true,
            derived: true
        })
    );
}

#[test]
fn derived_predicates_are_recomputed_after_deletes() {
    let task = Task::new(WIRING, LAMP);
    let violation =
        task.violation("(lay plant junction)\n(lay junction lamp)\n(cut plant junction)");
    assert_eq!(
        violation.kind,
        ViolationKind::GoalUnsatisfied(Unmet::Atom {
            atom: atom("powered", &["lamp"]),
            expected: true,
            derived: true
        })
    );
}
