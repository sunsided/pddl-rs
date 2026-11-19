//! Compile-time checked PDDL for the [`pddl`](https://docs.rs/pddl) crate.
//!
//! The macros parse PDDL while your crate compiles and turn every syntax error
//! into a compile error, all of them in one build. At run time, the PDDL is
//! parsed once, on first use.
//!
//! | Macro | Input | Expands to |
//! |---|---|---|
//! | [`domain!`] / [`domain_file!`] | one `(define (domain ...))` | `LazyLock<pddl::Domain>` |
//! | [`problem!`] / [`problem_file!`] | one `(define (problem ...))` | `LazyLock<pddl::Problem>` |
//! | [`pddl!`] / [`pddl_file!`] | any number of definitions | `LazyLock<pddl::PddlFile>` |
//! | [`validated!`] / [`validated_file!`] | any number of definitions | the checked source, `&'static str` |
//!
//! The inline forms take one string literal; raw strings (`r#"..."#`) are the
//! easiest to write. The `_file` forms take a path that, like
//! [`include_str!`], is relative to the file that calls the macro (or to the
//! crate root where the compiler can't name that file, as in tokens that
//! another macro generated). Cargo
//! rebuilds the crate when the PDDL file changes.
//!
//! The usual way in is the `pddl` crate's `macros` feature, which re-exports
//! these macros as `pddl::domain!` and so on. The `LazyLock` expansions refer
//! to `::pddl`, so your crate needs `pddl` as a direct dependency with its
//! `parser` feature (enabled by default), whether you call the macros through
//! `pddl` or here. [`validated!`] and [`validated_file!`] expand to text alone
//! and need neither.
//!
//! ## Example
//! ```
//! use std::sync::LazyLock;
//! use pddl::{Domain, Problem};
//!
//! static DOMAIN: LazyLock<Domain> = pddl_macros::domain!(r#"
//!     (define (domain switches)
//!       (:predicates (on ?s))
//!       (:action flip :parameters (?s) :precondition (not (on ?s)) :effect (on ?s)))
//! "#);
//!
//! static PROBLEM: LazyLock<Problem> = pddl_macros::problem!(r#"
//!     (define (problem one) (:domain switches)
//!       (:objects s1) (:init) (:goal (on s1)))
//! "#);
//!
//! assert_eq!(DOMAIN.name(), "switches");
//! assert_eq!(PROBLEM.domain(), "switches");
//! ```
//!
//! A typo fails the build:
//! ```compile_fail
//! static DOMAIN: std::sync::LazyLock<pddl::Domain> = pddl_macros::domain!(r#"
//!     (define (domain d) (:predicats (on ?s)))
//! "#);
//! ```

#![forbid(unsafe_code)]

use proc_macro::TokenStream;

use expand::{Form, Kind};

mod expand;
#[cfg(feature = "codegen")]
mod task;

/// Checks one PDDL domain at compile time and expands to a
/// `LazyLock<pddl::Domain>` that parses it on first use.
///
/// Takes one string literal. A problem, or more than one definition, is a
/// compile error.
///
/// ```
/// static DOMAIN: std::sync::LazyLock<pddl::Domain> =
///     pddl_macros::domain!("(define (domain d) (:predicates (p)))");
/// assert_eq!(DOMAIN.predicates().len(), 1);
/// ```
#[proc_macro]
pub fn domain(input: TokenStream) -> TokenStream {
    expand::run(Kind::Domain, Form::Inline, input)
}

/// Like [`domain!`], but reads the PDDL from a file, relative to the file that
/// calls the macro.
///
/// `static DOMAIN: LazyLock<Domain> = domain_file!("blocks-domain.pddl");`
#[proc_macro]
pub fn domain_file(input: TokenStream) -> TokenStream {
    expand::run(Kind::Domain, Form::File, input)
}

/// Checks one PDDL problem at compile time and expands to a
/// `LazyLock<pddl::Problem>` that parses it on first use.
///
/// Takes one string literal. A domain, or more than one definition, is a
/// compile error.
///
/// ```
/// static PROBLEM: std::sync::LazyLock<pddl::Problem> =
///     pddl_macros::problem!("(define (problem p) (:domain d) (:init) (:goal (and)))");
/// assert_eq!(PROBLEM.domain(), "d");
/// ```
#[proc_macro]
pub fn problem(input: TokenStream) -> TokenStream {
    expand::run(Kind::Problem, Form::Inline, input)
}

/// Like [`problem!`], but reads the PDDL from a file, relative to the file that
/// calls the macro.
///
/// `static PROBLEM: LazyLock<Problem> = problem_file!("bw-large-a.pddl");`
#[proc_macro]
pub fn problem_file(input: TokenStream) -> TokenStream {
    expand::run(Kind::Problem, Form::File, input)
}

/// Checks any number of PDDL domains and problems, in any order, at compile
/// time and expands to a `LazyLock<pddl::PddlFile>` that parses them on first use.
///
/// ```
/// static FILE: std::sync::LazyLock<pddl::PddlFile> = pddl_macros::pddl!(r#"
///     (define (domain d) (:predicates (p)))
///     (define (problem p) (:domain d) (:init) (:goal (p)))
/// "#);
/// assert_eq!((FILE.domain_count(), FILE.problem_count()), (1, 1));
/// ```
#[proc_macro]
pub fn pddl(input: TokenStream) -> TokenStream {
    expand::run(Kind::File, Form::Inline, input)
}

/// Like [`pddl!`], but reads the PDDL from a file, relative to the file that
/// calls the macro.
///
/// `static FILE: LazyLock<PddlFile> = pddl_file!("logistics.pddl");`
#[proc_macro]
pub fn pddl_file(input: TokenStream) -> TokenStream {
    expand::run(Kind::File, Form::File, input)
}

/// Checks PDDL at compile time like [`pddl!`], but expands to the source text
/// itself, a `&'static str`, for callers that parse it their own way.
///
/// ```
/// const SOURCE: &str = pddl_macros::validated!("(define (domain d))");
/// assert!(pddl::parsers::check(SOURCE).is_empty());
/// ```
#[proc_macro]
pub fn validated(input: TokenStream) -> TokenStream {
    expand::run(Kind::Validated, Form::Inline, input)
}

/// Like [`validated!`], but reads the PDDL from a file, relative to the file
/// that calls the macro. Expands to the checked text of that file.
///
/// `const SOURCE: &str = validated_file!("blocks-domain.pddl");`
#[proc_macro]
pub fn validated_file(input: TokenStream) -> TokenStream {
    expand::run(Kind::Validated, Form::File, input)
}

/// Generates typed planning code for a PDDL domain into a module: one id type
/// per PDDL type, `State`, `Statics`, `Action` and `Task`, which loads problems
/// at run time and implements `pddl::runtime::Task`.
///
/// `domain = "blocks.pddl"` reads a file relative to the calling file and
/// rebuilds when it changes; `domain_source = "(define ...)"` takes the PDDL
/// itself. Items already in the module stay. Unsupported requirements and
/// constructs are compile errors. Needs the `codegen` feature.
///
/// With `problem = "p.pddl"` (or `problem_source = "..."`) the task is
/// grounded while the crate compiles instead (level B2): the module holds
/// `State = [u64; WORDS]`, the mask tables `OPERATORS`, the names `NAMES`, and
/// a unit struct `Task`. Only STRIPS with negative preconditions grounds this
/// way. `max_operators = N` (default 100 000) and `max_facts = N` (default
/// 10 000) turn a too large task into a compile error rather than a slow build.
///
/// ```
/// #[pddl_macros::task(domain_source = r#"
///     (define (domain switches)
///       (:predicates (on ?s))
///       (:action flip :parameters (?s) :precondition (not (on ?s)) :effect (on ?s)))
/// "#)]
/// mod switches {}
///
/// use pddl::Parser;
/// use pddl::runtime::Task as _;
///
/// let problem = pddl::Problem::from_str(
///     "(define (problem p) (:domain switches) (:objects a b) (:init) (:goal (on a)))",
/// )
/// .unwrap();
/// let task = switches::Task::new(&problem).unwrap();
/// let mut successors = Vec::new();
/// task.successors(&task.initial_state(), &mut successors);
/// assert_eq!(successors.len(), 2);
/// assert_eq!(task.describe(successors[0].0), "(flip a)");
/// ```
#[cfg(feature = "codegen")]
#[proc_macro_attribute]
pub fn task(attr: TokenStream, item: TokenStream) -> TokenStream {
    task::run(attr, item)
}
