//! # A PDDL 3.1 parser, strongly typed
//!
//! This crates provides a PDDL 3.1 parser implementation based on [nom](https://crates.io/crates/nom).
//!
//! ## Default crate features
//!
//! * `parser` - Enables parsing of PDDL types through the [`Parser`] trait.
//! * `interning` - Enables string interning for [`Name`] types to reduce memory footprint.
//! * `pretty` - Enables pretty-printing of PDDL types via the
//!   `Pretty` extension trait. The feature provides round-trip-safe
//!   rendering: `parse → pretty() → parse` preserves AST equality.
//!
//! ## Example
//!
//! The two core types of a PDDL are [`Domain`] and [`Problem`]. This example shows how to
//! parse them:
//!
//! ```
//! # #[cfg(feature = "parser")]
//! # fn main() {
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
//! # }
//! # #[cfg(not(feature = "parser"))]
//! # fn main() {}
//! ```
//!
//! ## Multi-Definition Files
//!
//! Some PDDL files contain multiple domain and problem definitions. Use [`PddlFile`] to parse them:
//!
//! ```
//! # #[cfg(feature = "parser")]
//! # fn main() {
//! use pddl::{Parser, PddlFile};
//!
//! const MULTI_DEF: &'static str = r#"
//!     (define (domain my-domain)
//!         (:requirements :strips)
//!         (:predicates (p))
//!     )
//!     (define (problem my-problem)
//!         (:domain my-domain)
//!         (:init (p))
//!         (:goal (p))
//!     )
//! "#;
//!
//! let file = PddlFile::from_str(MULTI_DEF).unwrap();
//! assert_eq!(file.domain_count(), 1);
//! assert_eq!(file.problem_count(), 1);
//! # }
//! # #[cfg(not(feature = "parser"))]
//! # fn main() {}
//! ```
//!
//! ## PDDL Type System Guide
//!
//! This crate provides a comprehensive type system that models the PDDL 3.1 specification.
//! The type names may look unwieldy at first glance, but they follow a deliberate design principle:
//!
//! **Every type name maps directly to a BNF non-terminal from the PDDL 3.1 spec.**
//!
//! For example, the BNF rule `<c-effect>` becomes [`ConditionalEffect`], `<da-gd>` becomes
//! [`DurativeActionGoalDefinition`], and `<con-GD>` becomes [`ConstraintGoalDefinition`]. This 1:1 mapping
//! makes it trivial to cross-reference the spec while reading or writing code.
//!
//! ### BNF Naming Convention
//!
//! The PDDL 3.1 specification defines its grammar using BNF (Backus-Naur Form). Each
//! non-terminal in the BNF is written as `<something>`. This crate uses those names
//! directly, with minor transformations:
//!
//! | BNF Non-Terminal | Rust Type | Notes |
//! |------------------|-----------|-------|
//! | `<gd>` | [`GoalDefinition`] | Goal definition (the core logical formula type) |
//! | `<pre-GD>` | [`PreconditionGoalDefinition`] | Precondition-specific wrapper |
//! | `<precondition>` | [`PreconditionGoalDefinitions`] | Collection of preconditions |
//! | `<da-gd>` | [`DurativeActionGoalDefinition`] | Durative action goals (timed) |
//! | `<con-GD>` | [`ConstraintGoalDefinition`] | Constraint goals (temporal constraints) |
//! | `<con2-GD>` | [`ConstraintGoalDefinitionInner`] | Inner constraint goal (goal or nested con-GD) |
//! | `<pref-con-GD>` | [`PreferenceConstraintGoalDefinition`] | Preference constraint goals |
//! | `<pref-GD>` | [`PreferenceGoalDefinition`] | Preferred goal (goal or named preference) |
//! | `<c-effect>` | [`ConditionalEffect`] | Conditional effect |
//! | `<p-effect>` | [`PrimitiveEffect`] | Primitive effect |
//! | `<effect>` | [`Effects`] | Collection of effects (the `(and ...)` block) |
//! | `<da-effect>` | [`DurativeActionEffect`] | Durative action effect |
//! | `<f-exp>` | [`FluentExpression`] | Fluent/numeric expression |
//! | `<f-comp>` | [`FluentComparison`] | Fluent comparison |
//! | `<f-head>` | [`FunctionHead`] | Fluent head (function reference) |
//! | `<atomic formula (skeleton)>` | [`AtomicFormulaSkeleton`] | Predicate declaration with typed variables |
//!
//! When you see a type with an abbreviated name, look for the corresponding BNF rule
//! in the spec — it will almost certainly match.
//!
//! ### Type Groups
//!
//! #### Atoms & Formulas
//!
//! The building blocks of PDDL logical expressions:
//!
//! - [`AtomicFormula<T>`] — An atomic formula parameterized by its term type `T`.
//!   Can be either an equality (`(= x y)`) or a predicate application (`(on a b)`).
//!   See [`EqualityAtomicFormula`] and [`PredicateAtomicFormula`].
//! - [`AtomicFormulaSkeleton`] — A predicate declaration with typed variables, used
//!   in `(:predicates ...)` blocks. Think of it as a "template" for [`AtomicFormula`].
//! - [`Literal<T>`] — A possibly-negated [`AtomicFormula<T>`]. Represents `(not (on a b))`.
//! - [`Predicate`] — A named predicate (wraps a [`Name`]).
//!
//! **Generic parameter `T`**: [`AtomicFormula<T>`] and [`Literal<T>`] are generic.
//! - `AtomicFormula<Name>` — Used in problem initial states (concrete objects only).
//! - `AtomicFormula<Term>` — Used in action goals/preconditions (may contain [`Variable`]s).
//!
//! #### Goals & Preconditions
//!
//! This is where the naming proliferation is most apparent. Each variant serves a
//! distinct purpose in the PDDL spec:
//!
//! - [`GoalDefinition`] (`<gd>`) — The core logical formula type. Supports `and`, `or`,
//!   `not`, `imply`, `exists`, `forall`, atomic formulas, literals, and fluent comparisons.
//!   Used in action preconditions, goal definitions, and effect conditions.
//!
//! - [`PreconditionGoalDefinition`] (`<pre-GD>`) — A simplified variant used specifically
//!   for action preconditions. Contains either a [`PreferenceGoalDefinition`] or a `forall` quantifier.
//!   Multiple are collected in [`PreconditionGoalDefinitions`].
//!
//! - [`ProblemGoalDefinition`] (`<goal>` in problem context) — A thin wrapper around [`PreconditionGoalDefinitions`] used by
//!   [`Problem`] for the `(:goal ...)` section.
//!
//! - [`DurativeActionGoalDefinition`] (`<da-gd>`) — Goals for durative actions. Uses
//!   [`TimedGoalDefinition`] (time-qualified goals like `(at start (on a b))`) instead of plain
//!   [`GoalDefinition`].
//!
//! - [`ConstraintGoalDefinition`] (`<con-GD>`) — Temporal constraint goals for problem-level constraints.
//!   Supports `always`, `sometime`, `within`, `at-most-once`, `sometime-after`,
//!   `sometime-before`, `always-within`, `hold-during`, `hold-after`. Used by
//!   [`ProblemConstraintsDef`].
//!
//! - [`ConstraintGoalDefinitionInner`] (`<con2-GD>`) — An inner component of [`ConstraintGoalDefinition`], representing either
//!   a plain [`GoalDefinition`] or a nested [`ConstraintGoalDefinition`].
//!
//! - [`PreferenceConstraintGoalDefinition`] (`<pref-con-GD>`) — Constraint goals with optional preference names.
//!   Used in [`ProblemConstraintsDef`].
//!
//! - [`PreferenceGoalDefinition`] — Either a plain [`GoalDefinition`] or a named [`Preference`].
//!   Used in [`PreconditionGoalDefinition`].
//!
//! #### Effects
//!
//! The effects hierarchy mirrors the BNF structure:
//!
//! ```text
//! Effects (Vec<ConditionalEffect>)        ← the (and ...) block
//! └── ConditionalEffect                   ← one effect element
//!       ├── PrimitiveEffect               ← primitive effect
//!       ├── Forall(ForallConditionalEffect)  ← universal quantification over effects
//!       └── When(WhenConditionalEffect)   ← conditional effect (when <cond> <effect>)
//!             └── EffectCondition
//!                   ├── PrimitiveEffect   ← primitive effect (same as above)
//!                   ├── Forall...
//!                   └── When...
//! ```
//!
//! - [`PrimitiveEffect`] (`<p-effect>`) — A primitive effect: setting/negating an atomic formula,
//!   or assigning a numeric/object fluent.
//! - [`ConditionalEffect`] (`<c-effect>`) — A potentially conditional effect. Wraps [`PrimitiveEffect`],
//!   [`ForallConditionalEffect`], or [`WhenConditionalEffect`].
//! - [`Effects`] (`<effect>`) — A collection of [`ConditionalEffect`] values, representing the
//!   `(and ...)` block in PDDL.
//! - [`DurativeActionEffect`] (`<da-effect>`) — Effects for durative actions, using
//!   [`TimedEffect`] (time-qualified effects like `(at end (on a b))`).
//!
//! #### Numeric & Fluents
//!
//! Types for numeric reasoning in PDDL:
//!
//! - [`FluentExpression`] (`<f-exp>`) — A fluent expression: a number, function call, negation,
//!   binary operation (`+`, `-`, `*`, `/`), or multi-operand operation.
//! - [`FunctionHead`] (`<f-head>`) — A function head: either a bare function symbol or a
//!   [`FunctionTerm`] (function applied to arguments).
//! - [`FluentComparison`] (`<f-comp>`) — A fluent comparison: `<`, `<=`, `>`, `>=`, `=` applied
//!   to two [`FluentExpression`] values.
//! - [`TimedFluentExpression`] (`<f-exp-t>`) — Time-qualified fluent expression (for durative actions).
//! - [`DurativeActionFluentExpression`] (`<f-exp-da>`) — Durative-action-specific fluent expression.
//! - [`FunctionHead`] variants: [`BasicFunctionTerm`], [`FunctionTerm`].
//! - [`AssignOp`] / [`TimedAssignOperator`] — Assignment operators (`assign`, `scale-up`, etc.).
//!
//! #### Terms
//!
//! The fundamental building blocks that appear in formulas:
//!
//! - [`Term`] — A term: either a [`Name`] (concrete object), [`Variable`], or
//!   [`FunctionTerm`] (fluent application).
//! - [`Name`] — A named constant/object.
//! - [`Variable`] — A logical variable (e.g., `?x`).
//! - [`TypedList<T>`] — A list of items with shared type annotations, e.g.,
//!   `(?x ?y - robot obj1 obj2 - package)`.
//!
//! ### Common Confusion Points
//!
//! | Confused Pair | Difference |
//! |---------------|------------|
//! | [`AtomicFormulaSkeleton`] vs [`AtomicFormula`] | Skeleton declares a predicate's signature (used in `:predicates`); Formula applies it to concrete terms (used in goals/init). |
//! | [`PrimitiveEffect`] vs [`ConditionalEffect`] vs [`Effects`] | `PrimitiveEffect` is a single primitive effect; `ConditionalEffect` may add conditions/quantifiers; `Effects` is the `(and ...)` collection. |
//! | [`GoalDefinition`] vs [`PreconditionGoalDefinition`] | `GoalDefinition` is the full logical formula; `PreconditionGoalDefinition` is a simplified wrapper used only in action preconditions. |
//! | [`GoalDefinition`] vs [`DurativeActionGoalDefinition`] | `GoalDefinition` is timeless; `DurativeActionGoalDefinition` uses [`TimedGoalDefinition`] for time-qualified goals `(at start ...)`. |
//! | [`ConstraintGoalDefinition`] vs [`GoalDefinition`] | `ConstraintGoalDefinition` is for problem-level temporal constraints (`always`, `sometime`); `GoalDefinition` is the standard logical formula. |
//! | [`ProblemGoalDefinition`] vs [`GoalDefinition`] | `ProblemGoalDefinition` wraps [`PreconditionGoalDefinitions`] for the problem's `(:goal ...)`; `GoalDefinition` is the individual formula. |
//! | [`PreferenceGoalDefinition`] vs [`PreferenceConstraintGoalDefinition`] | `PreferenceGoalDefinition` is for action-level preferences; `PreferenceConstraintGoalDefinition` is for problem-level constraint preferences. |
//! | [`Effects`] vs [`ConditionalEffect`] vs [`EffectCondition`] | `Effects` is the top-level `(and ...)` collection of [`ConditionalEffect`]s (action `:effect` body); `ConditionalEffect` is one element that may be primitive, `forall`-quantified, or `when`-conditional; `EffectCondition` is the recursive `ConditionalEffect` tree nested inside `when` clauses. |
//! | [`FluentExpression`] vs [`FunctionHead`] | `FluentExpression` is any numeric expression; `FunctionHead` is specifically a function reference (with or without args). |
//! | [`Literal<T>`] vs [`AtomicFormula<T>`] | `Literal` can be negated; `AtomicFormula` is always positive. |
//!
//! ### PDDL Snippet → Rust Type Mapping
//!
//! ```text
//! ;; PDDL domain definition
//! (define (domain blocksworld)
//!   (:predicates (on ?x ?y)      ← AtomicFormulaSkeleton
//!                (clear ?x))      ← AtomicFormulaSkeleton
//!
//!   (:action pick-up
//!     :parameters (?x)            ← TypedList<Variable>
//!     :precondition (and          ← PreconditionGoalDefinitions
//!       (clear ?x)                ← GoalDefinition::AtomicFormula
//!       (handempty))              ← GoalDefinition::AtomicFormula
//!     :effect (and                ← Effects
//!       (not (clear ?x))          ← ConditionalEffect::PrimitiveEffect(PrimitiveEffect::NotAtomicFormula)
//!       (holding ?x)))            ← ConditionalEffect::PrimitiveEffect(PrimitiveEffect::AtomicFormula)
//!
//!   (:goal (and                   ← ProblemGoalDefinition → PreconditionGoalDefinitions
//!     (on a b)                    ← GoalDefinition::AtomicFormula
//!     (on b c)))                  ← GoalDefinition::AtomicFormula
//! )
//!
//! ;; PDDL problem initial state
//! (define (problem bw-1)
//!   (:init
//!     (on a b)                    ← InitElement::Literal (Literal<Name>)
//!     (clear a)                   ← InitElement::Literal
//!     (= (total-cost) 0))         ← InitElement::IsValue (numeric fluent)
//! )
//! ```
//!
//! ### Navigating Further
//!
//! Each type has its own documentation with detailed examples and PDDL requirement annotations.
//! Use your IDE's "Go to Definition" or hover documentation to explore individual types.

// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
pub mod parsers;
mod types;
pub mod visitor;

// re-export Parser trait.
#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
pub use parsers::Parser;

// re-export types
pub use types::*;

#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
#[cfg(feature = "pretty")]
pub mod pretty_print;

#[cfg_attr(docsrs, doc(cfg(feature = "pretty")))]
#[cfg(feature = "pretty")]
pub use pretty_print::{Pretty, PrettyPrinted, PrettyRenderer};
