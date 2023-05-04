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
/// assert!(parse_require_def("(:requirements :adl)".into()).is_value(Requirements::new([Requirement::Adl])));
/// assert!(parse_require_def("(:requirements :strips :typing)".into()).is_value(Requirements::new([Requirement::Strips, Requirement::Typing])));
/// assert!(parse_require_def("(:requirements\n:strips   :typing  )".into()).is_value(Requirements::new([Requirement::Strips, Requirement::Typing])));
///```
pub fn parse_require_def(input: Span) -> ParseResult<Requirements> {
    let (remaining, reqs) =
        prefix_expr(":requirements", space_separated_list1(parse_require_key))(input)?;

    Ok((remaining, Requirements::new(reqs)))
}

/// Parses a requirement key, i.e. `:strips`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_require_key, preamble::*};
/// # use pddl::Requirement;
/// assert!(parse_require_key(":strips".into()).is_value(Requirement::Strips));
/// assert!(parse_require_key(":typing".into()).is_value(Requirement::Typing));
/// assert!(parse_require_key(":negative-preconditions".into()).is_value(Requirement::NegativePreconditions));
/// assert!(parse_require_key(":disjunctive-preconditions".into()).is_value(Requirement::DisjunctivePreconditions));
/// assert!(parse_require_key(":equality".into()).is_value(Requirement::Equality));
/// assert!(parse_require_key(":existential-preconditions".into()).is_value(Requirement::ExistentialPreconditions));
/// assert!(parse_require_key(":universal-preconditions".into()).is_value(Requirement::UniversalPreconditions));
/// assert!(parse_require_key(":quantified-preconditions".into()).is_value(Requirement::QuantifiedPreconditions));
/// assert!(parse_require_key(":conditional-effects".into()).is_value(Requirement::ConditionalEffects));
/// assert!(parse_require_key(":fluents".into()).is_value(Requirement::Fluents));
/// assert!(parse_require_key(":numeric-fluents".into()).is_value(Requirement::NumericFluents));
/// assert!(parse_require_key(":adl".into()).is_value(Requirement::Adl));
/// assert!(parse_require_key(":durative-actions".into()).is_value(Requirement::DurativeActions));
/// assert!(parse_require_key(":duration-inequalities".into()).is_value(Requirement::DurationInequalities));
/// assert!(parse_require_key(":continuous-effects".into()).is_value(Requirement::ContinuousEffects));
/// assert!(parse_require_key(":derived-predicates".into()).is_value(Requirement::DerivedPredicates));
/// assert!(parse_require_key(":timed-initial-literals".into()).is_value(Requirement::TimedInitialLiterals));
/// assert!(parse_require_key(":preferences".into()).is_value(Requirement::Preferences));
/// assert!(parse_require_key(":constraints".into()).is_value(Requirement::Constraints));
/// assert!(parse_require_key(":action-costs".into()).is_value(Requirement::ActionCosts));
///
/// assert!(parse_require_key(":unknown".into()).is_err());
/// assert!(parse_require_key("invalid".into()).is_err());
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
    )(input)
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
