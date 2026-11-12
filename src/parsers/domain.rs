//! Provides parsers for domain definitions.

use nom::branch::alt;
use nom::combinator::map;
use nom::Parser;

use crate::parsers::{
    fold_sections, parse_name, parse_types_def, prefix_expr, set_once, space_separated_list1, ws2,
};
use crate::parsers::{
    parse_constants_def, parse_domain_constraints_def, parse_functions_def, parse_predicates_def,
    parse_require_def, parse_structure_def, parse_timeless_def, ParseResult, Span,
};
use crate::types::{
    Constants, Domain, Functions, Name, PredicateDefinitions, Requirements, StructureDef,
    StructureDefs,
};
use crate::types::{DomainConstraintsDef, Timeless, Types};

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
    ws2(prefix_expr("define", parse_domain_body)).parse(input.into())
}

/// A top-level part of a domain definition.
enum Section {
    Extends(Vec<Name>),
    Requirements(Requirements),
    Types(Types),
    Constants(Constants),
    Predicates(PredicateDefinitions),
    Functions(Functions),
    Constraints(DomainConstraintsDef),
    Timeless(Timeless),
    Structure(StructureDef),
}

/// Parses `(domain <name>)` followed by the sections in any order.
///
/// Each section may appear at most once; structure definitions (actions etc.)
/// keep their relative order and may be interleaved with the sections.
fn parse_domain_body(input: Span) -> ParseResult<Domain> {
    let (input, name) = prefix_expr("domain", parse_name).parse(input)?;

    let mut extends = None;
    let mut requirements = None;
    let mut types = None;
    let mut constants = None;
    let mut predicates = None;
    let mut functions = None;
    let mut constraints = None;
    let mut timeless = None;
    let mut structure = Vec::new();

    let section = alt((
        map(
            prefix_expr(":extends", space_separated_list1(parse_name)),
            Section::Extends,
        ),
        map(parse_require_def, Section::Requirements),
        map(parse_types_def, Section::Types),
        map(parse_constants_def, Section::Constants),
        map(parse_predicates_def, Section::Predicates),
        map(parse_functions_def, Section::Functions),
        map(parse_domain_constraints_def, Section::Constraints),
        map(parse_timeless_def, Section::Timeless),
        map(parse_structure_def, Section::Structure),
    ));
    let (input, ()) = fold_sections(input, section, |section, at| match section {
        Section::Extends(v) => set_once(&mut extends, v, at),
        Section::Requirements(v) => set_once(&mut requirements, v, at),
        Section::Types(v) => set_once(&mut types, v, at),
        Section::Constants(v) => set_once(&mut constants, v, at),
        Section::Predicates(v) => set_once(&mut predicates, v, at),
        Section::Functions(v) => set_once(&mut functions, v, at),
        Section::Constraints(v) => set_once(&mut constraints, v, at),
        Section::Timeless(v) => set_once(&mut timeless, v, at),
        Section::Structure(v) => {
            structure.push(v);
            Ok(())
        }
    })?;

    let domain = Domain::builder(name, StructureDefs::new(structure))
        .with_extends(extends.unwrap_or_default())
        .with_requirements(requirements.unwrap_or_default())
        .with_types(types.unwrap_or_default())
        .with_constants(constants.unwrap_or_default())
        .with_predicates(predicates.unwrap_or_default())
        .with_functions(functions.unwrap_or_default())
        .with_constraints(constraints.unwrap_or_default())
        .with_timeless(timeless.unwrap_or_default());
    Ok((input, domain))
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
    fn sections_in_any_order() {
        let ordered = Domain::from_str(
            "(define (domain d) (:requirements :strips) (:types t) (:predicates (p)) (:functions (c)) \
             (:action a :parameters () :effect (p)) (:action b :parameters () :effect (p)))",
        )
        .unwrap();
        let shuffled = Domain::from_str(
            "(define (domain d) (:functions (c)) (:action a :parameters () :effect (p)) \
             (:predicates (p)) (:types t) (:action b :parameters () :effect (p)) (:requirements :strips))",
        )
        .unwrap();
        assert_eq!(ordered, shuffled);
    }

    #[test]
    fn duplicate_section_is_reported_at_the_second_occurrence() {
        let input = "(define (domain d)\n  (:predicates (p))\n  (:predicates (q)))";
        let error = Domain::from_str(input).unwrap_err();
        assert_eq!((error.line(), error.column()), (3, 3));
        assert_eq!(error.found(), Some("(:predicates (q)))"));
    }

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
