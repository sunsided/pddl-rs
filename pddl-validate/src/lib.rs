//! # PDDL plan validation
//!
//! An independent plan validator for PDDL, a second opinion next to
//! [VAL](https://github.com/KCL-Planning/VAL). It implements the published semantics on
//! the [`pddl`] syntax tree and shares no grounding or search code with any planner, so
//! it does not share their bugs either.
//!
//! ```
//! use pddl::{Domain, Parser, Problem};
//! use pddl_validate::{ValidationStatus, Validator, ViolationKind};
//!
//! let domain = Domain::from_str(
//!     "(define (domain logistics) (:requirements :typing)
//!        (:types truck place)
//!        (:predicates (at ?t - truck ?p - place))
//!        (:action drive
//!          :parameters (?t - truck ?from ?to - place)
//!          :precondition (at ?t ?from)
//!          :effect (and (not (at ?t ?from)) (at ?t ?to))))",
//! ).unwrap();
//! let problem = Problem::from_str(
//!     "(define (problem deliver) (:domain logistics)
//!        (:objects t1 - truck home shop - place)
//!        (:init (at t1 home))
//!        (:goal (at t1 shop)))",
//! ).unwrap();
//! let validator = Validator::new(&domain, &problem).unwrap();
//!
//! let report = validator.validate_text("(drive t1 home shop)\n; cost = 1 (unit cost)");
//! assert_eq!(report.status(), ValidationStatus::Valid);
//!
//! let report = validator.validate_text("(drive t1 shop home)");
//! let violation = report.violation().unwrap();
//! assert_eq!(violation.step.as_ref().unwrap().number, 1);
//! assert!(matches!(violation.kind, ViolationKind::PreconditionUnsatisfied(_)));
//! assert_eq!(violation.advice(),
//!     "(drive t1 shop home) has an unsatisfied precondition at step 1: (Set (at t1 shop) to true)");
//! ```
//!
//! ## Scope
//!
//! - **Classical plans** (level 1): STRIPS, typing, negative and disjunctive
//!   preconditions, equality, `exists`, `forall`, `imply`, conditional effects and
//!   stratified derived predicates.
//! - **Numeric plans** (level 2): numeric fluents in conditions and effects (`assign`,
//!   `increase`, `decrease`, `scale-up`, `scale-down`), action costs and the problem's
//!   `:metric`, which [`Report::metric`] returns for a valid plan.
//!
//! - **Temporal plans** (level 3): durative actions with `at start`, `over all` and
//!   `at end` conditions and effects, timestamped plans (`t: (action args) [d]`),
//!   duration constraints and inequalities, conditional effects with `at start` or
//!   `at end` conditions, linear continuous effects (`#t`), timed initial literals, the
//!   epsilon separation of interfering events ([`Validator::with_epsilon`]) and
//!   `total-time` as the makespan.
//!
//! - **PDDL+** (level 4): processes, which run while their precondition holds and change
//!   fluents linearly, and events, which fire as soon as their precondition holds,
//!   in cascades of up to 1000 rounds at one time. The validator finds the times at
//!   which processes switch and events trigger exactly, at the roots of the linear
//!   conditions.
//!
//! Every construct outside this scope makes the report
//! [`ValidationStatus::Unsupported`], never valid: continuous change whose rate depends
//! on a changing fluent (such as acceleration), conditions that are not linear in time,
//! discrete effects of processes, dynamics that switch without end, derived predicates
//! in temporal plans, preferences, constraints and object fluents.
//!
//! ## Semantics
//!
//! - Names are case-insensitive. Reports show them in ASCII lowercase.
//! - Closed world: an atom not in the state is false.
//! - A step's precondition and the conditions of its `when` effects are evaluated in the
//!   state before the step. The next state is the old one minus deleted atoms plus added
//!   ones, so an atom that a step both deletes and adds stays true.
//! - Derived predicates are the least fixpoint of their rules, stratum by stratum, and are
//!   recomputed after every step.
//! - Numbers are `f64`. Comparisons allow a relative tolerance
//!   ([`Validator::with_tolerance`], [`DEFAULT_TOLERANCE`]). Reading a fluent without a
//!   value, dividing by zero or leaving the finite range makes the plan invalid. A
//!   nullary `total-cost` without an initial value starts at 0.
//! - Updates of one fluent in one step combine if all are `increase` or `decrease`;
//!   any other combination is a [`ViolationKind::ConflictingUpdates`].
//! - In a metric, `total-time` is the number of steps of a sequential plan and the
//!   makespan of a temporal one.
//! - Temporal plans follow PDDL 2.1 and VAL. Events at one time form a happening whose
//!   conditions are checked in the state before it and whose effects apply at once.
//!   `over all` conditions must hold strictly between an action's start and end, and
//!   interfering events must be at least epsilon apart.
//! - Validation stops at the first violation. Steps count from 1, as in VAL.
//!
//! The crate is `no_std` with `alloc`. That is nominal for now: the `pddl` crate it reads
//! needs `std`.

#![no_std]
#![forbid(unsafe_code)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

mod atom;
mod derived;
mod effects;
mod error;
mod eval;
mod model;
mod plan;
mod render;
mod report;
mod state;
mod validator;

pub use atom::Atom;
pub use error::{ModelError, ModelErrorKind};
pub use plan::{Plan, PlanError, PlanErrorKind, PlanStep};
pub use report::{
    Construct, FluentChange, Metric, Outcome, Report, StepRef, StepTrace, Unmet, Unsupported,
    Violation, ViolationKind, MAX_ALTERNATIVES,
};
pub use validator::{Validator, DEFAULT_EPSILON, DEFAULT_TOLERANCE};

/// The overall result of validating a plan.
///
/// [`ValidationStatus::Unsupported`] is distinct from both other outcomes: a plan that uses
/// constructs the validator cannot interpret is never reported as valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationStatus {
    /// The plan is executable and reaches the goal.
    Valid,
    /// The plan violates the domain or problem semantics.
    Invalid,
    /// The plan uses a construct the validator does not support.
    Unsupported,
}
