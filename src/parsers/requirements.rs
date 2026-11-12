//! Provides parsers for requirements.

use nom::branch::alt;
use nom::character::complete::{alpha1, char};
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::Parser;

use crate::parsers::name::parse_any_char;
use crate::parsers::{keyword, prefix_expr, space_separated_list1, ParseResult, Span};
use crate::types::requirement::{names, Requirement};
use crate::types::{Name, Requirements};

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
    let key = alt((
        map(parse_require_key, Key::Known),
        map(parse_unknown_require_key, Key::Unknown),
    ));
    map(
        prefix_expr(":requirements", space_separated_list1(key)),
        |keys| {
            let mut known = Vec::new();
            let mut unknown = Vec::new();
            for key in keys {
                match key {
                    Key::Known(requirement) => known.push(requirement),
                    Key::Unknown(name) => unknown.push(name),
                }
            }
            Requirements::new(known).with_unknown(unknown)
        },
    )
    .parse(input.into())
}

/// A requirement key, known to this crate or not.
enum Key {
    Known(Requirement),
    Unknown(Name),
}

/// Parses a requirement key this crate does not know, such as `:foo-bar`.
/// The returned name includes the leading `:`.
fn parse_unknown_require_key(input: Span) -> ParseResult<Name> {
    map(
        recognize((char(':'), alpha1, many0(parse_any_char))),
        |key: Span| Name::from(*key.fragment()),
    )
    .parse(input)
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
pub fn parse_require_key<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Requirement> {
    map(
        alt((
            keyword(names::STRIPS),
            keyword(names::TYPING),
            keyword(names::NEGATIVE_PRECONDITIONS),
            keyword(names::DISJUNCTIVE_PRECONDITIONS),
            keyword(names::EQUALITY),
            keyword(names::EXISTENTIAL_PRECONDITIONS),
            keyword(names::UNIVERSAL_PRECONDITIONS),
            keyword(names::QUANTIFIED_PRECONDITIONS),
            keyword(names::CONDITIONAL_EFFECTS),
            keyword(names::FLUENTS),
            keyword(names::NUMERIC_FLUENTS),
            keyword(names::OBJECT_FLUENTS),
            keyword(names::ADL),
            keyword(names::DURATIVE_ACTIONS),
            keyword(names::DURATION_INEQUALITIES),
            keyword(names::CONTINUOUS_EFFECTS),
            keyword(names::DERIVED_PREDICATES),
            keyword(names::TIMED_INITIAL_LITERALS),
            keyword(names::PREFERENCES),
            keyword(names::CONSTRAINTS),
            keyword(names::ACTION_COSTS),
        )),
        |x: Span| {
            Requirement::try_from(x.fragment().to_ascii_lowercase().as_str())
                .expect("unhandled variant")
        },
    )
    .parse(input.into())
}

impl crate::parsers::Parser for Requirements {
    type Item = Requirements;

    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_require_def(input)
    }
}

impl crate::parsers::Parser for Requirement {
    type Item = Requirement;

    /// See [`parse_require_key`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_require_key(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{Requirement, Requirements};

    #[test]
    fn unknown_keys_are_kept_separately() {
        let reqs =
            Requirements::from_str("(:requirements :strips :foo-bar :TYPING :typing-x)").unwrap();
        assert_eq!(*reqs, [Requirement::Strips, Requirement::Typing]);
        // `:typing-x` is a different key, not `:typing` followed by garbage.
        assert_eq!(reqs.unknown(), [":foo-bar", ":typing-x"]);
    }

    #[test]
    fn only_unknown_keys() {
        let reqs = Requirements::from_str("(:requirements :foo)").unwrap();
        assert!(reqs.is_empty());
        assert!(!reqs.is_empty_including_unknown());
        assert_eq!(reqs.unknown(), [":foo"]);
        // Unknown keys don't count as requirements, so `:strips` is still implied.
        assert_eq!(
            reqs.to_effective(),
            [Requirement::Strips].into_iter().collect()
        );
    }

    #[test]
    fn a_key_must_still_look_like_a_key() {
        assert!(Requirements::from_str("(:requirements strips)").is_err());
        assert!(Requirements::from_str("(:requirements :)").is_err());
        assert!(Requirements::from_str("(:requirements :1x)").is_err());
    }

    #[test]
    fn test_parse_requirement() {
        assert!(Requirement::parse(":strips").is_value(Requirement::Strips));
        assert!(Requirement::parse(":typing").is_value(Requirement::Typing));
        assert!(Requirement::parse(":negative-preconditions")
            .is_value(Requirement::NegativePreconditions));
        assert!(Requirement::parse(":disjunctive-preconditions")
            .is_value(Requirement::DisjunctivePreconditions));
        assert!(Requirement::parse(":equality").is_value(Requirement::Equality));
        assert!(Requirement::parse(":existential-preconditions")
            .is_value(Requirement::ExistentialPreconditions));
        assert!(Requirement::parse(":universal-preconditions")
            .is_value(Requirement::UniversalPreconditions));
        assert!(Requirement::parse(":quantified-preconditions")
            .is_value(Requirement::QuantifiedPreconditions));
        assert!(
            Requirement::parse(":conditional-effects").is_value(Requirement::ConditionalEffects)
        );
        assert!(Requirement::parse(":fluents").is_value(Requirement::Fluents));
        assert!(Requirement::parse(":numeric-fluents").is_value(Requirement::NumericFluents));
        assert!(Requirement::parse(":adl").is_value(Requirement::Adl));
        assert!(Requirement::parse(":durative-actions").is_value(Requirement::DurativeActions));
        assert!(Requirement::parse(":duration-inequalities")
            .is_value(Requirement::DurationInequalities));
        assert!(Requirement::parse(":continuous-effects").is_value(Requirement::ContinuousEffects));
        assert!(Requirement::parse(":derived-predicates").is_value(Requirement::DerivedPredicates));
        assert!(Requirement::parse(":timed-initial-literals")
            .is_value(Requirement::TimedInitialLiterals));
        assert!(Requirement::parse(":preferences").is_value(Requirement::Preferences));
        assert!(Requirement::parse(":constraints").is_value(Requirement::Constraints));
        assert!(Requirement::parse(":action-costs").is_value(Requirement::ActionCosts));

        assert!(Requirement::parse(":unknown").is_err());
        assert!(Requirement::parse("invalid").is_err());
    }

    #[test]
    fn test_parse_requirements() {
        assert!(Requirements::parse("(:requirements :adl)")
            .is_value(Requirements::new([Requirement::Adl])));
        assert!(
            Requirements::parse("(:requirements :strips :typing)").is_value(Requirements::new([
                Requirement::Strips,
                Requirement::Typing
            ]))
        );
        assert!(
            Requirements::parse("(:requirements\n:strips   :typing  )").is_value(
                Requirements::new([Requirement::Strips, Requirement::Typing])
            )
        );
    }
}
