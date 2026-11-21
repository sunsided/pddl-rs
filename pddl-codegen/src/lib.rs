//! Generates typed Rust planning code from PDDL.
//!
//! [`lifted`] turns a domain into the items of a module; the domain is fixed
//! when the crate compiles and problems load at run time:
//!
//! - `types::Block` and so on: one id type per PDDL type. Objects are numbered
//!   so that each type's objects, subtypes included, are one range, which makes
//!   a type check a range check.
//! - `State`: one field per predicate that some action changes, a `bool`,
//!   `BitSet`, `BitRelation` or `BitTable` from `pddl-runtime`.
//! - `Statics`: the same for the static predicates, outside the state.
//! - `Action`: a `Copy` enum with one variant per action and typed fields.
//! - `Task`: loads a problem with `Task::new(&problem)` and implements
//!   `pddl_runtime::Task`. Its successor generation is nested loops over the
//!   parameters that check each precondition as soon as its variables are
//!   bound.
//!
//! The code refers to `::pddl::runtime` and `::pddl::codegen`, so the crate
//! that compiles it needs `pddl` with the `codegen` feature. The usual way in
//! is `#[pddl::task(domain = "blocks.pddl")] mod blocks {}`. A build script can
//! write the tokens to `OUT_DIR` instead:
//!
//! ```ignore
//! // build.rs
//! use pddl::{Domain, Parser};
//!
//! let source = std::fs::read_to_string("blocks.pddl").unwrap();
//! let domain = Domain::from_str(&source).unwrap();
//! let code = pddl_codegen::lifted(&domain).unwrap();
//! let out = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("blocks.rs");
//! std::fs::write(out, code.to_string()).unwrap();
//! println!("cargo::rerun-if-changed=blocks.pddl");
//! ```
//!
//! ```ignore
//! // src/lib.rs
//! pub mod blocks {
//!     include!(concat!(env!("OUT_DIR"), "/blocks.rs"));
//! }
//! ```
//!
//! Supported: STRIPS, `:typing`, `:negative-preconditions`, `:equality` and
//! `:adl` (`or`, `imply`, `exists`, `forall`, `not` over any formula, and
//! `forall` and `when` in effects), numeric fluents with action costs, and
//! stratified derived predicates.
//! Anything else is an [`Error`] that names it.
//!
//! [`grounded`] fixes the problem too: it grounds the task at build time into
//! bit masks over a `[u64; WORDS]` state, for STRIPS with negative
//! preconditions.

#![forbid(unsafe_code)]

use std::fmt::{Display, Formatter};

use pddl_core::{Domain, Problem};
use proc_macro2::TokenStream;

mod grounded;
mod lifted;
mod model;

/// Why code generation failed: an unsupported construct or an inconsistent
/// domain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    /// What went wrong.
    message: String,
}

impl Error {
    /// An error with this message.
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Prefixes the message with the action it occurred in.
    pub(crate) fn context(self, action: &str) -> Self {
        Self::new(format!("action `{action}`: {}", self.message))
    }

    /// What went wrong.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

/// Generates the items of a module that plans in `domain` (level B1): typed
/// ids, `State`, `Statics`, `Action` and `Task`. See the [crate documentation](crate).
///
/// # Errors
/// An [`Error`] for an unsupported requirement or construct, an undeclared
/// name, or PDDL names that map to the same Rust identifier.
pub fn lifted(domain: &Domain) -> Result<TokenStream, Error> {
    lifted_with(domain, &Bindings::default())
}

/// PDDL types bound to Rust types, and predicates and functions attached to
/// Rust functions (#131).
///
/// - A bound type `robot => crate::Robot` puts one `crate::Robot` per object
///   of type `robot` (subtypes included) into `Task::bound().robot`, indexed
///   by the object's id. `Task::new` takes a closure per bound type that
///   makes the value from the object's name. States keep holding ids only.
/// - An attached predicate `near => crate::near` is not stored: preconditions,
///   goals and rules call `crate::near(&bound, a, b) -> bool` with the
///   arguments as typed ids. An attached function returns `f64`. Attachments
///   must be pure, and see only the bound values, not the state: a predicate
///   or function that an action or rule changes can't be attached.
#[derive(Debug, Clone, Default)]
pub struct Bindings {
    /// PDDL type names and the Rust types they bind to.
    pub types: Vec<(String, TokenStream)>,
    /// Predicate or function names and the paths of the Rust functions they
    /// attach to.
    pub attachments: Vec<(String, TokenStream)>,
}

/// Like [`lifted`], with types bound to Rust types and predicates and
/// functions attached to Rust functions.
///
/// # Errors
/// As [`lifted`], and if a binding names an unknown type, predicate or
/// function, or attaches one that actions or rules change.
pub fn lifted_with(domain: &Domain, bindings: &Bindings) -> Result<TokenStream, Error> {
    let model = model::Model::new(domain)?;
    lifted::generate(&model, bindings)
}

/// Limits on the size of a grounded task, so that a large problem fails with
/// a clear error instead of a very slow build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// The most ground actions; 100 000 by default.
    pub max_operators: usize,
    /// The most ground atoms that actions change; 10 000 by default.
    pub max_facts: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_operators: 100_000,
            max_facts: 10_000,
        }
    }
}

/// Grounds `domain` and `problem` with [`pddl_core::sas_plus::ground`] and
/// generates the items of a module that plans in that one task (level B2):
/// `State = [u64; WORDS]`, the mask tables `OPERATORS`, `INITIAL`,
/// `GOAL_TRUE` and `GOAL_FALSE`, the names `NAMES` and `FACTS`, and a unit
/// struct `Task` implementing `pddl_runtime::Task`.
///
/// Compile time grows with the number of ground actions, about linearly:
/// each one is a row of four masks of `WORDS` words.
///
/// # Errors
/// An [`Error`] if grounding fails, the task has conditional effects (B2 is
/// STRIPS with negative preconditions), or it exceeds `limits`.
pub fn grounded(domain: &Domain, problem: &Problem, limits: &Limits) -> Result<TokenStream, Error> {
    let file = pddl_core::sas_plus::ground(domain, problem)
        .map_err(|e| Error::new(format!("grounding failed: {e}")))?;
    grounded::generate(&file, limits)
}
