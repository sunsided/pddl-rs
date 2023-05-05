//! Provides parsers for requirements.

use crate::parsers::{prefix_expr, space_separated_list1, ParseResult, Span};
use crate::types::requirement::{names, Requirement};
use crate::types::Requirements;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

/// Parses a requirement definition, i.e. `(:requirements <require-key>)⁺`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_require_def, preamble::*};
/// # use pddl::{Requirement, Requirements};
/// assert!(parse_require_def("(:requirements :adl)").is_value(Requirements::new([Requirement::Adl])));
/// assert!(parse_require_def("(:requirements :strips :typing)").is_value(Requirements::new([Requirement::Strips, Requirement::Typing])));
/// assert!(parse_require_def("(:requirements\n:strips   :typing  )").is_value(Requirements::new([Requirement::Strips, Requirement::Typing])));
///```
pub fn parse_require_def<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Requirements> {
    map(
        prefix_expr(":requirements", space_separated_list1(parse_require_key)),
        Requirements::new,
    )(input.into())
}

/// Parses a requirement key, i.e. `:strips`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_require_key, preamble::*};
/// # use pddl::Requirement;
/// assert!(parse_require_key(":strips").is_value(Requirement::Strips));
/// assert!(parse_require_key(":typing").is_value(Requirement::Typing));
/// assert!(parse_require_key(":negative-preconditions").is_value(Requirement::NegativePreconditions));
/// assert!(parse_require_key(":disjunctive-preconditions").is_value(Requirement::DisjunctivePreconditions));
/// assert!(parse_require_key(":equality").is_value(Requirement::Equality));
/// assert!(parse_require_key(":existential-preconditions").is_value(Requirement::ExistentialPreconditions));
/// assert!(parse_require_key(":universal-preconditions").is_value(Requirement::UniversalPreconditions));
/// assert!(parse_require_key(":quantified-preconditions").is_value(Requirement::QuantifiedPreconditions));
/// assert!(parse_require_key(":conditional-effects").is_value(Requirement::ConditionalEffects));
/// assert!(parse_require_key(":fluents").is_value(Requirement::Fluents));
/// assert!(parse_require_key(":numeric-fluents").is_value(Requirement::NumericFluents));
/// assert!(parse_require_key(":adl").is_value(Requirement::Adl));
/// assert!(parse_require_key(":durative-actions").is_value(Requirement::DurativeActions));
/// assert!(parse_require_key(":duration-inequalities").is_value(Requirement::DurationInequalities));
/// assert!(parse_require_key(":continuous-effects").is_value(Requirement::ContinuousEffects));
/// assert!(parse_require_key(":derived-predicates").is_value(Requirement::DerivedPredicates));
/// assert!(parse_require_key(":timed-initial-literals").is_value(Requirement::TimedInitialLiterals));
/// assert!(parse_require_key(":preferences").is_value(Requirement::Preferences));
/// assert!(parse_require_key(":constraints").is_value(Requirement::Constraints));
/// assert!(parse_require_key(":action-costs").is_value(Requirement::ActionCosts));
///
/// assert!(parse_require_key(":unknown").is_err());
/// assert!(parse_require_key("invalid").is_err());
///```
pub fn parse_require_key(input: Span) -> ParseResult<Requirement> {
    map(
        alt((
            tag(names::STRIPS),
            tag(names::TYPING),
            tag(names::NEGATIVE_PRECONDITIONS),
            tag(names::DISJUNCTIVE_PRECONDITIONS),
            tag(names::EQUALITY),
            tag(names::EXISTENTIAL_PRECONDITIONS),
            tag(names::UNIVERSAL_PRECONDITIONS),
            tag(names::QUANTIFIED_PRECONDITIONS),
            tag(names::CONDITIONAL_EFFECTS),
            tag(names::FLUENTS),
            tag(names::NUMERIC_FLUENTS),
            tag(names::OBJECT_FLUENTS),
            tag(names::ADL),
            tag(names::DURATIVE_ACTIONS),
            tag(names::DURATION_INEQUALITIES),
            tag(names::CONTINUOUS_EFFECTS),
            tag(names::DERIVED_PREDICATES),
            tag(names::TIMED_INITIAL_LITERALS),
            tag(names::PREFERENCES),
            tag(names::CONSTRAINTS),
            tag(names::ACTION_COSTS),
        )),
        |x: Span| Requirement::try_from(*x.fragment()).expect("unhandled variant"),
    )(input.into())
}

impl<'a> crate::parsers::Parser<'a> for Requirements {
    type Item = Requirements;

    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_require_def(input.into())
    }
}

impl<'a> crate::parsers::Parser<'a> for Requirement {
    type Item = Requirement;

    /// See [`parse_require_key`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_require_key(input.into())
    }
}
