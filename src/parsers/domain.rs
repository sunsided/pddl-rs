//! Provides parsers for domain definitions.

use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::{preceded, tuple};

use crate::parsers::{
    parse_constants_def, parse_domain_constraints_def, parse_functions_def, parse_predicates_def,
    parse_require_def, parse_structure_def, ParseResult, Span,
};
use crate::parsers::{parse_name, parse_types_def, prefix_expr, space_separated_list1, ws2};
use crate::types::{
    Constants, Domain, Functions, PredicateDefinitions, Requirements, StructureDefs,
};
use crate::types::{DomainConstraintsDef, Types};

/// Parses a domain definition.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_action_def, parse_domain, preamble::*};
/// # use pddl::Name;
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
pub fn parse_domain<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Domain> {
    map(
        ws2(prefix_expr(
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

impl crate::parsers::Parser for Domain {
    type Item = Domain;

    /// Parses a domain definition.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{Domain, Name, Parser};
    /// let input = r#"
    ///  ; A toy domain.
    ///  (define (domain briefcase-world)
    ///       ; If no requirements are provided, :strips is implied.
    ///       (:requirements :strips :equality :typing :conditional-effects)
    ///       (:types location physob) ; type definitions could also be represented as predicates
    ///       (:constants B P D - physob)
    ///       (:predicates (at ?x - physob ?y - location)
    ///                    (in ?x ?y - physob))
    ///       (:constraints (and))
    ///
    ///       ; Move briefcase from one location to another.
    ///       (:action mov-B
    ///            :parameters (?m ?l - location)
    ///            :precondition (and (at B ?m) (not (= ?m ?l)))
    ///            :effect (and (at B ?l) (not (at B ?m))
    ///                         (forall (?z)
    ///                             (when (and (in ?z) (not (= ?z B)))
    ///                                   (and (at ?z ?l) (not (at ?z ?m)))))) )
    ///
    ///       ; Put the item in the briefcase if it is not already in there.
    ///       (:action put-in
    ///            :parameters (?x - physob ?l - location)
    ///            :precondition (not (= ?x B))
    ///            :effect (when (and (at ?x ?l) (at B ?l))
    ///                  (in ?x)) )
    ///
    ///       ; Take the item out of the briefcase if it is in there.
    ///       (:action take-out
    ///            :parameters (?x - physob)
    ///            :precondition (not (= ?x B))
    ///            :effect (not (in ?x)) )
    ///     )"#;
    ///
    /// let (_, domain) = Domain::parse(input).unwrap();
    ///
    /// assert_eq!(domain.name(), &Name::new("briefcase-world"));
    /// assert_eq!(domain.requirements().len(), 4);
    /// assert_eq!(domain.types().len(), 2);
    /// assert_eq!(domain.constants().len(), 3);
    /// assert_eq!(domain.predicates().len(), 2);
    /// assert!(domain.constraints().is_empty());
    /// assert_eq!(domain.structure().len(), 3);
    /// ```
    ///
    /// ## See also
    /// See [`parse_domain`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_domain(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, Name, Parser};

    #[test]
    fn test_parse() {
        let input = r#"
         ; A toy domain.
         (define (domain briefcase-world)
              ; If no requirements are provided, :strips is implied.
              (:requirements :strips :equality :typing :conditional-effects)
              (:types location physob) ; type definitions could also be represented as predicates
              (:constants B P D - physob)
              (:predicates (at ?x - physob ?y - location)
                           (in ?x ?y - physob))
              (:constraints (and))

              ; Move briefcase from one location to another.
              (:action mov-B
                   :parameters (?m ?l - location)
                   :precondition (and (at B ?m) (not (= ?m ?l)))
                   :effect (and (at B ?l) (not (at B ?m))
                                (forall (?z)
                                    (when (and (in ?z) (not (= ?z B)))
                                          (and (at ?z ?l) (not (at ?z ?m)))))) )

              ; Put the item in the briefcase if it is not already in there.
              (:action put-in
                   :parameters (?x - physob ?l - location)
                   :precondition (not (= ?x B))
                   :effect (when (and (at ?x ?l) (at B ?l))
                         (in ?x)) )

              ; Take the item out of the briefcase if it is in there.
              (:action take-out
                   :parameters (?x - physob)
                   :precondition (not (= ?x B))
                   :effect (not (in ?x)) )
            )"#;

        let (_, domain) = Domain::parse(input).unwrap();

        assert_eq!(domain.name(), &Name::new("briefcase-world"));
        assert_eq!(domain.requirements().len(), 4);
        assert_eq!(domain.types().len(), 2);
        assert_eq!(domain.constants().len(), 3);
        assert_eq!(domain.predicates().len(), 2);
        assert!(domain.constraints().is_empty());
        assert_eq!(domain.structure().len(), 3);
    }
}
