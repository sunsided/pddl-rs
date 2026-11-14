//! Provides parsers for domain definitions.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{
    parse_constants_def, parse_domain_constraints_def, parse_functions_def, parse_name,
    parse_predicates_def, parse_require_def, parse_structure_def, parse_timeless_def,
    parse_types_def, prefix, set_once,
};
use crate::types::{Domain, StructureDefs};

/// Parses a domain definition, i.e. `(define (domain <name>) <section>*)`.
///
/// The sections may appear in any order; each one except actions at most once.
pub(crate) fn parse_domain(node: &Node) -> R<Domain> {
    prefix(node, "define", |args| {
        let name = prefix(args.next()?, "domain", |args| parse_name(args.next()?))?;
        let mut extends = None;
        let mut requirements = None;
        let mut types = None;
        let mut constants = None;
        let mut predicates = None;
        let mut functions = None;
        let mut constraints = None;
        let mut timeless = None;
        let mut structure = Vec::new();
        for section in args.rest() {
            if section.atom().is_some() {
                return Err(section.error());
            }
            let head = section.head().unwrap_or_default().to_ascii_lowercase();
            match head.as_str() {
                ":extends" => {
                    let names = prefix(section, ":extends", |args| args.list1(parse_name))?;
                    set_once(&mut extends, names, section)?;
                }
                ":requirements" => {
                    set_once(&mut requirements, parse_require_def(section)?, section)?;
                }
                ":types" => set_once(&mut types, parse_types_def(section)?, section)?,
                ":constants" => set_once(&mut constants, parse_constants_def(section)?, section)?,
                ":predicates" => {
                    set_once(&mut predicates, parse_predicates_def(section)?, section)?;
                }
                ":functions" => set_once(&mut functions, parse_functions_def(section)?, section)?,
                ":constraints" => {
                    let value = parse_domain_constraints_def(section)?;
                    set_once(&mut constraints, value, section)?;
                }
                ":timeless" => set_once(&mut timeless, parse_timeless_def(section)?, section)?,
                ":action" | ":durative-action" | ":derived" => {
                    structure.push(parse_structure_def(section)?);
                }
                _ => return Err(section.error().reason("unknown section").fatal()),
            }
        }
        Ok(Domain::builder(name, StructureDefs::new(structure))
            .with_extends(extends.unwrap_or_default())
            .with_requirements(requirements.unwrap_or_default())
            .with_types(types.unwrap_or_default())
            .with_constants(constants.unwrap_or_default())
            .with_predicates(predicates.unwrap_or_default())
            .with_functions(functions.unwrap_or_default())
            .with_constraints(constraints.unwrap_or_default())
            .with_timeless(timeless.unwrap_or_default()))
    })
}

crate::parsers::impl_parser!(Domain => parse_domain);

#[cfg(test)]
mod tests {

    use crate::*;

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

        let domain = Domain::from_str(input).unwrap();

        assert_eq!(domain.name(), &Name::new("briefcase-world"));
        assert_eq!(domain.requirements().len(), 4);
        assert_eq!(domain.types().len(), 2);
        assert_eq!(domain.constants().len(), 3);
        assert_eq!(domain.predicates().len(), 2);
        assert!(domain.constraints().is_empty());
        assert_eq!(domain.structure().len(), 3);
    }

    #[test]
    fn doc_example_0() {
        let input = r#"(define (domain briefcase-world)
              (:requirements :strips :equality :typing :conditional-effects)
              (:types location physob)
              (:constants B P D - physob)
              (:predicates (at ?x - physob ?y - location)
                           (in ?x ?y - physob))
              (:constraints (and))
        
              (:action mov-B
                   :parameters (?m ?l - location)
                   :precondition (and (at B ?m) (not (= ?m ?l)))
                   :effect (and (at B ?l) (not (at B ?m))
                                (forall (?z)
                                    (when (and (in ?z) (not (= ?z B)))
                                          (and (at ?z ?l) (not (at ?z ?m)))))) )
        
              (:action put-in
                   :parameters (?x - physob ?l - location)
                   :precondition (not (= ?x B))
                   :effect (when (and (at ?x ?l) (at B ?l))
                         (in ?x)) )
        
              (:action take-out
                   :parameters (?x - physob)
                   :precondition (not (= ?x B))
                   :effect (not (in ?x)) )
            )"#;

        let domain = Domain::from_str(input).unwrap();
        assert_eq!(domain.name(), &Name::new("briefcase-world"));
        assert_eq!(domain.requirements().len(), 4);
        assert_eq!(domain.types().len(), 2);
        assert_eq!(domain.constants().len(), 3);
        assert_eq!(domain.predicates().len(), 2);
        assert!(domain.constraints().is_empty());
        assert_eq!(domain.structure().len(), 3);
    }

    #[test]
    fn doc_example_1() {
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

        let domain = Domain::from_str(input).unwrap();

        assert_eq!(domain.name(), &Name::new("briefcase-world"));
        assert_eq!(domain.requirements().len(), 4);
        assert_eq!(domain.types().len(), 2);
        assert_eq!(domain.constants().len(), 3);
        assert_eq!(domain.predicates().len(), 2);
        assert!(domain.constraints().is_empty());
        assert_eq!(domain.structure().len(), 3);
    }
}
