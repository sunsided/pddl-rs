//! Provides parsers for domain definitions.

use crate::parsers::{
    parse_constants_def, parse_domain_constraints_def, parse_functions_def, parse_predicates_def,
    parse_require_def, parse_structure_def,
};
use crate::parsers::{parse_name, parse_types_def, prefix_expr, space_separated_list1, ws};
use crate::types::{ConGD, Types};
use crate::types::{
    Constants, Domain, Functions, PredicateDefinitions, Requirements, StructureDefs,
};
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses a domain definition.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_action_def, parse_domain};
/// # use pddl::types::Name;
///
/// let input = r#"(define (domain briefcase-world)
///       (:requirements :strips :equality :typing :conditional-effects)
///       (:types location physob)
///       (:constants B P D - physob)
///       (:predicates (at ?x - physob ?y - location)
///                    (in ?x ?y - physob))
///       (:constraints (and))
///
///       (:action mov-B
///            :parameters (?m ?l - location)
///            :precondition (and (at B ?m) (not (= ?m ?l)))
///            :effect (and (at B ?l) (not (at B ?m))
///                         (forall (?z)
///                             (when (and (in ?z) (not (= ?z B)))
///                                   (and (at ?z ?l) (not (at ?z ?m)))))) )
///
///       (:action put-in
///            :parameters (?x - physob ?l - location)
///            :precondition (not (= ?x B))
///            :effect (when (and (at ?x ?l) (at B ?l))
///                  (in ?x)) )
///
///       (:action take-out
///            :parameters (?x - physob)
///            :precondition (not (= ?x B))
///            :effect (not (in ?x)) )
///     )"#;
///
/// let (remainder, domain) = parse_domain(input).unwrap();
///
/// assert_eq!(remainder, "");
/// assert_eq!(domain.name(), &Name::new("briefcase-world"));
/// assert_eq!(domain.requirements().len(), 4);
/// assert_eq!(domain.types().len(), 2);
/// assert_eq!(domain.constants().len(), 3);
/// assert_eq!(domain.predicates().len(), 2);
/// assert!(domain.constraints().is_empty());
/// assert_eq!(domain.structure().len(), 3);
/// ```
pub fn parse_domain(input: &str) -> IResult<&str, Domain> {
    map(
        ws(prefix_expr(
            "define",
            tuple((
                prefix_expr("domain", parse_name),
                opt(preceded(
                    multispace1,
                    prefix_expr(":extends", space_separated_list1(parse_name)),
                )),
                opt(preceded(multispace1, parse_require_def)),
                // :typing
                opt(preceded(multispace1, parse_types_def)),
                opt(preceded(multispace1, parse_constants_def)),
                opt(preceded(multispace1, parse_predicates_def)),
                // :fluents
                opt(preceded(multispace1, parse_functions_def)),
                // :constraints
                opt(preceded(multispace1, parse_domain_constraints_def)),
                opt(preceded(
                    multispace1,
                    map(
                        space_separated_list1(parse_structure_def),
                        StructureDefs::new,
                    ),
                )),
            )),
        )),
        |(
            name,
            extends,
            require,
            types,
            constants,
            predicates,
            functions,
            constraints,
            structure,
        )| {
            Domain::new(
                name,
                extends.unwrap_or(Vec::default()),
                require.unwrap_or(Requirements::default()),
                types.unwrap_or(Types::default()),
                constants.unwrap_or(Constants::default()),
                predicates.unwrap_or(PredicateDefinitions::default()),
                functions.unwrap_or(Functions::default()),
                constraints.unwrap_or(ConGD::default()),
                structure.unwrap_or(StructureDefs::default()),
            )
        },
    )(input)
}
