//! # A PDDL 3.1 parser, strongly typed
//!
//! This crates provides a PDDL 3.1 parser implementation based on [nom](https://crates.io/crates/nom).
//!
//! ## Default crate features
//!
//! * `parser` - Enables parsing of PDDL types through the [`Parser`] trait.
//! * `interning` - Enables string interning for [`Name`] types to reduce memory footprint.
//!
//! ## Example
//!
//! The two core types of a PDDL are [`Domain`] and [`Problem`]. This example shows how to
//! parse them:
//!
//! ```
//! use pddl::{Parser, Domain, Problem};
//!
//! const BRIEFCASE_WORLD: &'static str = r#"
//!     (define (domain briefcase-world)
//!         (:requirements :strips :equality :typing :conditional-effects)
//!         (:types location physob)
//!         (:constants B P D - physob)
//!         (:predicates (at ?x - physob ?y - location)
//!                      (in ?x ?y - physob))
//!
//!         (:action mov-B
//!             :parameters (?m ?l - location)
//!             :precondition (and (at B ?m) (not (= ?m ?l)))
//!             :effect (and (at B ?l) (not (at B ?m))
//!                          (forall (?z)
//!                              (when (and (in ?z) (not (= ?z B)))
//!                                    (and (at ?z ?l) (not (at ?z ?m)))))) )
//!
//!         (:action put-in
//!             :parameters (?x - physob ?l - location)
//!             :precondition (not (= ?x B))
//!             :effect (when (and (at ?x ?l) (at B ?l))
//!                     (in ?x)) )
//!
//!         (:action take-out
//!             :parameters (?x - physob)
//!             :precondition (not (= ?x B))
//!             :effect (not (in ?x)) )
//!     )
//!     "#;
//!
//! const BRIEFCASE_WORLD_PROBLEM: &'static str = r#"
//!     (define (problem get-paid)
//!         (:domain briefcase-world)
//!         (:init (place home) (place office)
//!             (object p) (object d) (object b)
//!             (at B home) (at P home) (at D home) (in P))
//!         (:goal (and (at B office) (at D office) (at P home)))
//!     )
//!     "#;
//!
//! let domain = Domain::from_str(BRIEFCASE_WORLD).unwrap();
//! let problem = Problem::from_str(BRIEFCASE_WORLD_PROBLEM).unwrap();
//!
//! // All elements were parsed.
//! assert_eq!(domain.name(), "briefcase-world");
//! assert_eq!(domain.requirements().len(), 4);
//! assert_eq!(domain.types().len(), 2);
//! assert_eq!(domain.constants().len(), 3);
//! assert_eq!(domain.predicates().len(), 2);
//! assert_eq!(domain.structure().len(), 3);
//!
//! // All elements were parsed.
//! assert_eq!(problem.name(), "get-paid");
//! assert_eq!(problem.domain(), "briefcase-world");
//! assert!(problem.requirements().is_empty());
//! assert_eq!(problem.init().len(), 9);
//! assert_eq!(problem.goals().len(), 3);
//! ```

// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
pub mod parsers;
mod types;
pub(crate) mod visitor;

// re-export Parser trait.
#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
pub use parsers::Parser;

// re-export types
pub use types::*;
