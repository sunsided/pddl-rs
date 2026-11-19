//! The types and the parser behind the [`pddl`](https://docs.rs/pddl) crate.
//!
//! Depend on `pddl` instead: it re-exports everything in this crate and adds
//! the compile-time `domain!`, `problem!` and `pddl!` macros behind its
//! `macros` feature. This crate exists so that `pddl-macros` can use the
//! parser without depending on `pddl`, which re-exports it.

// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
pub mod parsers;
#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
pub mod syntax;
mod types;
pub mod visitor;

// re-export Parser trait.
#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
pub use parsers::{ParseError, Parser};

// re-export types
pub use types::*;

#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
#[cfg(feature = "pretty")]
pub mod pretty_print;

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
#[cfg(feature = "serde")]
pub mod serde;

#[cfg_attr(docsrs, doc(cfg(feature = "sas-plus")))]
#[cfg(feature = "sas-plus")]
pub mod sas_plus;

#[cfg_attr(docsrs, doc(cfg(feature = "codegen")))]
#[cfg(feature = "codegen")]
pub mod codegen;

#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
#[cfg(feature = "pretty")]
pub use pretty_print::{Pretty, PrettyPrinted, PrettyRenderer};
