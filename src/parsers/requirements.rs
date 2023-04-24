//! Provides parsers for requirements.

use crate::parsers::{prefix_expr, space_separated_list1};
use crate::types::requirement::{names, Requirement};
use crate::types::Requirements;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses a requirement definition, i.e. `(:requirements <require-key>)⁺`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_require_def;
/// # use pddl::{Requirement, Requirements};
/// assert_eq!(parse_require_def("(:requirements :adl)"), Ok(("", Requirements::new([Requirement::Adl]))));
/// assert_eq!(parse_require_def("(:requirements :strips :typing)"), Ok(("", Requirements::new([Requirement::Strips, Requirement::Typing]))));
/// assert_eq!(parse_require_def("(:requirements\n:strips   :typing  )"), Ok(("", Requirements::new([Requirement::Strips, Requirement::Typing]))));
///```
pub fn parse_require_def(input: &str) -> IResult<&str, Requirements> {
    let (remaining, reqs) =
        prefix_expr(":requirements", space_separated_list1(parse_require_key))(input)?;

    Ok((remaining, Requirements::new(reqs)))
}

/// Parses a requirement key, i.e. `:strips`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_require_key;
/// # use pddl::Requirement;
/// assert_eq!(parse_require_key(":strips"), Ok(("", Requirement::Strips)));
/// assert_eq!(parse_require_key(":typing"), Ok(("", Requirement::Typing)));
/// assert_eq!(parse_require_key(":negative-preconditions"), Ok(("", Requirement::NegativePreconditions)));
/// assert_eq!(parse_require_key(":disjunctive-preconditions"), Ok(("", Requirement::DisjunctivePreconditions)));
/// assert_eq!(parse_require_key(":equality"), Ok(("", Requirement::Equality)));
/// assert_eq!(parse_require_key(":existential-preconditions"), Ok(("", Requirement::ExistentialPreconditions)));
/// assert_eq!(parse_require_key(":universal-preconditions"), Ok(("", Requirement::UniversalPreconditions)));
/// assert_eq!(parse_require_key(":quantified-preconditions"), Ok(("", Requirement::QuantifiedPreconditions)));
/// assert_eq!(parse_require_key(":conditional-effects"), Ok(("", Requirement::ConditionalEffects)));
/// assert_eq!(parse_require_key(":fluents"), Ok(("", Requirement::Fluents)));
/// assert_eq!(parse_require_key(":numeric-fluents"), Ok(("", Requirement::NumericFluents)));
/// assert_eq!(parse_require_key(":adl"), Ok(("", Requirement::Adl)));
/// assert_eq!(parse_require_key(":durative-actions"), Ok(("", Requirement::DurativeActions)));
/// assert_eq!(parse_require_key(":duration-inequalities"), Ok(("", Requirement::DurationInequalities)));
/// assert_eq!(parse_require_key(":continuous-effects"), Ok(("", Requirement::ContinuousEffects)));
/// assert_eq!(parse_require_key(":derived-predicates"), Ok(("", Requirement::DerivedPredicates)));
/// assert_eq!(parse_require_key(":timed-initial-literals"), Ok(("", Requirement::TimedInitialLiterals)));
/// assert_eq!(parse_require_key(":preferences"), Ok(("", Requirement::Preferences)));
/// assert_eq!(parse_require_key(":constraints"), Ok(("", Requirement::Constraints)));
/// assert_eq!(parse_require_key(":action-costs"), Ok(("", Requirement::ActionCosts)));
///
/// assert!(parse_require_key(":unknown").is_err());
/// assert!(parse_require_key("invalid").is_err());
///```
pub fn parse_require_key(input: &str) -> IResult<&str, Requirement> {
    map_res(
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
        Requirement::try_from,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for Requirements {
    type Item = Requirements;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_require_def(input)
    }
}

impl<'a> crate::parsers::Parser<'a> for Requirement {
    type Item = Requirement;

    /// See [`parse_require_key`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_require_key(input)
    }
}
