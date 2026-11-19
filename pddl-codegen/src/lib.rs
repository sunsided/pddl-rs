//! Generates typed Rust planning code from PDDL.
//!
//! [`lifted`] turns a domain into the items of a module (level B1 of #132):
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
//! `forall` and `when` in effects). Anything else is an [`Error`] that names it.

#![forbid(unsafe_code)]

use std::fmt::{Display, Formatter};

use pddl_core::Domain;
use proc_macro2::TokenStream;

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
    let model = model::Model::new(domain)?;
    lifted::generate(&model)
}
