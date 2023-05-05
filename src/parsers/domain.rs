//! Provides parsers for domain definitions.

use crate::parsers::{
    parse_constants_def, parse_domain_constraints_def, parse_functions_def, parse_predicates_def,
    parse_require_def, parse_structure_def, ParseResult, Span,
};
use crate::parsers::{parse_name, parse_types_def, prefix_expr, space_separated_list1, ws};
use crate::types::{
    Constants, Domain, Functions, PredicateDefinitions, Requirements, StructureDefs,
};
use crate::types::{DomainConstraintsDef, Types};
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::{preceded, tuple};

/// Parses a domain definition.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_action_def, parse_domain, preamble::*};
/// # use pddl::Name;
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
/// assert!(remainder.is_empty());
/// assert_eq!(domain.name(), &Name::new("briefcase-world"));
/// assert_eq!(domain.requirements().len(), 4);
/// assert_eq!(domain.types().len(), 2);
/// assert_eq!(domain.constants().len(), 3);
/// assert_eq!(domain.predicates().len(), 2);
/// assert!(domain.constraints().is_empty());
/// assert_eq!(domain.structure().len(), 3);
/// ```
pub fn parse_domain<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Domain<'a>> {
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
            Domain::builder(name, structure.unwrap_or(StructureDefs::default()))
                .with_extends(extends.unwrap_or(Vec::default()))
                .with_requirements(require.unwrap_or(Requirements::default()))
                .with_types(types.unwrap_or(Types::default()))
                .with_constants(constants.unwrap_or(Constants::default()))
                .with_predicates(predicates.unwrap_or(PredicateDefinitions::default()))
                .with_functions(functions.unwrap_or(Functions::default()))
                .with_constraints(constraints.unwrap_or(DomainConstraintsDef::default()))
        },
    )(input.into())
}

impl<'a> crate::parsers::Parser<'a> for Domain<'a> {
    type Item = Domain<'a>;

    /// See [`parse_domain`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_domain(input)
    }
}
