//! Provides parsers for requirements.

use crate::parsers::{definition_section, space_separated_list1};
use crate::types::requirement::{names, Requirement};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::error::ErrorKind;
use nom::{error_position, IResult};

/// Parses a requirement definition, i.e. `(:requirements <require-key>)⁺`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_require_def;
/// # use pddl::types::Requirement;
/// assert_eq!(parse_require_def("(:requirements :adl)"), Ok(("", vec![Requirement::Adl])));
/// assert_eq!(parse_require_def("(:requirements :strips :typing)"), Ok(("", vec![Requirement::Strips, Requirement::Typing])));
/// assert_eq!(parse_require_def("(:requirements\n:strips   :typing  )"), Ok(("", vec![Requirement::Strips, Requirement::Typing])));
///```
pub fn parse_require_def(input: &str) -> IResult<&str, Vec<Requirement>> {
    let (remaining, req_keys) =
        definition_section(":requirements", space_separated_list1(parse_require_key))(input)?;

    let mut reqs = Vec::with_capacity(req_keys.len());
    for key in req_keys {
        match Requirement::try_from(key) {
            Ok(r) => reqs.push(r),
            Err(_e) => return Err(nom::Err::Failure(error_position!(input, ErrorKind::OneOf))),
        }
    }

    Ok((remaining, reqs))
}

/// Parses a requirement key, i.e. `:strips`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_require_key;
/// assert_eq!(parse_require_key(":strips"), Ok(("", ":strips")));
/// assert_eq!(parse_require_key(":typing"), Ok(("", ":typing")));
/// assert_eq!(parse_require_key(":negative-preconditions"), Ok(("", ":negative-preconditions")));
/// assert_eq!(parse_require_key(":disjunctive-preconditions"), Ok(("", ":disjunctive-preconditions")));
/// assert_eq!(parse_require_key(":equality"), Ok(("", ":equality")));
/// assert_eq!(parse_require_key(":existential-preconditions"), Ok(("", ":existential-preconditions")));
/// assert_eq!(parse_require_key(":universal-preconditions"), Ok(("", ":universal-preconditions")));
/// assert_eq!(parse_require_key(":quantified-preconditions"), Ok(("", ":quantified-preconditions")));
/// assert_eq!(parse_require_key(":conditional-effects"), Ok(("", ":conditional-effects")));
/// assert_eq!(parse_require_key(":fluents"), Ok(("", ":fluents")));
/// assert_eq!(parse_require_key(":numeric-fluents"), Ok(("", ":numeric-fluents")));
/// assert_eq!(parse_require_key(":adl"), Ok(("", ":adl")));
/// assert_eq!(parse_require_key(":durative-actions"), Ok(("", ":durative-actions")));
/// assert_eq!(parse_require_key(":durative-inequalities"), Ok(("", ":durative-inequalities")));
/// assert_eq!(parse_require_key(":continuous-effects"), Ok(("", ":continuous-effects")));
/// assert_eq!(parse_require_key(":derived-predicates"), Ok(("", ":derived-predicates")));
/// assert_eq!(parse_require_key(":timed-initial-literals"), Ok(("", ":timed-initial-literals")));
/// assert_eq!(parse_require_key(":preferences"), Ok(("", ":preferences")));
/// assert_eq!(parse_require_key(":constraints"), Ok(("", ":constraints")));
/// assert_eq!(parse_require_key(":action-costs"), Ok(("", ":action-costs")));
///
/// assert!(parse_require_key(":unknown").is_err());
/// assert!(parse_require_key("invalid").is_err());
///```
pub fn parse_require_key(input: &str) -> IResult<&str, &str> {
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
        tag(names::DURATIVE_INEQUALITIES),
        tag(names::CONTINUOUS_EFFECTS),
        tag(names::DERIVED_PREDICATES),
        tag(names::TIMED_INITIAL_LITERALS),
        tag(names::PREFERENCES),
        tag(names::CONSTRAINTS),
        tag(names::ACTION_COSTS),
    ))(input)
}
