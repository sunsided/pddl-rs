//! Provides parsers for requirements.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{is_name, prefix, word};
use crate::types::requirement::names;
use crate::types::{Name, Requirement, Requirements};

/// The known requirement keys.
const KEYS: &[(&str, Requirement)] = &[
    (names::STRIPS, Requirement::Strips),
    (names::TYPING, Requirement::Typing),
    (
        names::NEGATIVE_PRECONDITIONS,
        Requirement::NegativePreconditions,
    ),
    (
        names::DISJUNCTIVE_PRECONDITIONS,
        Requirement::DisjunctivePreconditions,
    ),
    (names::EQUALITY, Requirement::Equality),
    (
        names::EXISTENTIAL_PRECONDITIONS,
        Requirement::ExistentialPreconditions,
    ),
    (
        names::UNIVERSAL_PRECONDITIONS,
        Requirement::UniversalPreconditions,
    ),
    (
        names::QUANTIFIED_PRECONDITIONS,
        Requirement::QuantifiedPreconditions,
    ),
    (names::CONDITIONAL_EFFECTS, Requirement::ConditionalEffects),
    (names::FLUENTS, Requirement::Fluents),
    (names::NUMERIC_FLUENTS, Requirement::NumericFluents),
    (names::OBJECT_FLUENTS, Requirement::ObjectFluents),
    (names::ADL, Requirement::Adl),
    (names::DURATIVE_ACTIONS, Requirement::DurativeActions),
    (
        names::DURATION_INEQUALITIES,
        Requirement::DurationInequalities,
    ),
    (names::CONTINUOUS_EFFECTS, Requirement::ContinuousEffects),
    (names::DERIVED_PREDICATES, Requirement::DerivedPredicates),
    (
        names::TIMED_INITIAL_LITERALS,
        Requirement::TimedInitialLiterals,
    ),
    (names::PREFERENCES, Requirement::Preferences),
    (names::CONSTRAINTS, Requirement::Constraints),
    (names::ACTION_COSTS, Requirement::ActionCosts),
];

/// Parses a requirements definition, i.e. `(:requirements <require-key>⁺)`.
///
/// Unknown keys such as `:foo-bar` are kept apart in [`Requirements::unknown`],
/// so a newer or planner-specific requirement doesn't fail the whole file.
pub(crate) fn parse_require_def(node: &Node) -> R<Requirements> {
    let keys = prefix(node, ":requirements", |args| args.list1(parse_key))?;
    let mut known = Vec::new();
    let mut unknown = Vec::new();
    for key in keys {
        match key {
            Ok(requirement) => known.push(requirement),
            Err(name) => unknown.push(name),
        }
    }
    Ok(Requirements::new(known).with_unknown(unknown))
}

/// A known requirement, or an unknown key `:<name>` kept as written.
fn parse_key(node: &Node) -> R<Result<Requirement, Name>> {
    if let Ok(requirement) = parse_require_key(node) {
        return Ok(Ok(requirement));
    }
    match node.atom() {
        Some(key) if key.strip_prefix(':').is_some_and(is_name) => Ok(Err(Name::from(key))),
        _ => Err(node.error()),
    }
}

/// Parses a known requirement key, e.g. `:strips`, ignoring case.
pub(crate) fn parse_require_key(node: &Node) -> R<Requirement> {
    word(node, KEYS)
}

crate::parsers::impl_parser!(Requirements => parse_require_def, Requirement => parse_require_key);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Parser, Requirement, Requirements};

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
        assert!(Requirement::from_str(":strips").is_value(Requirement::Strips));
        assert!(Requirement::from_str(":typing").is_value(Requirement::Typing));
        assert!(Requirement::from_str(":negative-preconditions")
            .is_value(Requirement::NegativePreconditions));
        assert!(Requirement::from_str(":disjunctive-preconditions")
            .is_value(Requirement::DisjunctivePreconditions));
        assert!(Requirement::from_str(":equality").is_value(Requirement::Equality));
        assert!(Requirement::from_str(":existential-preconditions")
            .is_value(Requirement::ExistentialPreconditions));
        assert!(Requirement::from_str(":universal-preconditions")
            .is_value(Requirement::UniversalPreconditions));
        assert!(Requirement::from_str(":quantified-preconditions")
            .is_value(Requirement::QuantifiedPreconditions));
        assert!(
            Requirement::from_str(":conditional-effects").is_value(Requirement::ConditionalEffects)
        );
        assert!(Requirement::from_str(":fluents").is_value(Requirement::Fluents));
        assert!(Requirement::from_str(":numeric-fluents").is_value(Requirement::NumericFluents));
        assert!(Requirement::from_str(":adl").is_value(Requirement::Adl));
        assert!(Requirement::from_str(":durative-actions").is_value(Requirement::DurativeActions));
        assert!(Requirement::from_str(":duration-inequalities")
            .is_value(Requirement::DurationInequalities));
        assert!(
            Requirement::from_str(":continuous-effects").is_value(Requirement::ContinuousEffects)
        );
        assert!(
            Requirement::from_str(":derived-predicates").is_value(Requirement::DerivedPredicates)
        );
        assert!(Requirement::from_str(":timed-initial-literals")
            .is_value(Requirement::TimedInitialLiterals));
        assert!(Requirement::from_str(":preferences").is_value(Requirement::Preferences));
        assert!(Requirement::from_str(":constraints").is_value(Requirement::Constraints));
        assert!(Requirement::from_str(":action-costs").is_value(Requirement::ActionCosts));

        assert!(Requirement::from_str(":unknown").is_err());
        assert!(Requirement::from_str("invalid").is_err());
    }

    #[test]
    fn test_parse_requirements() {
        assert!(Requirements::from_str("(:requirements :adl)")
            .is_value(Requirements::new([Requirement::Adl])));
        assert!(
            Requirements::from_str("(:requirements :strips :typing)").is_value(Requirements::new(
                [Requirement::Strips, Requirement::Typing]
            ))
        );
        assert!(
            Requirements::from_str("(:requirements\n:strips   :typing  )").is_value(
                Requirements::new([Requirement::Strips, Requirement::Typing])
            )
        );
    }
}
