#![allow(dead_code)]

pub const BRIEFCASE_WORLD: &'static str = r#"
    (define (domain briefcase-world)
      (:requirements :strips :equality :typing :conditional-effects)
      (:types location physob)
      (:constants B P D - physob)
      (:predicates (at ?x - physob ?y - location)
                   (in ?x ?y - physob))

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
    )
    "#;

pub const BRIEFCASE_WORLD_PROBLEM: &'static str = r#"
    (define (problem get-paid)
        (:domain briefcase-wordld)
        (:init (place home) (place office)
               (object p) (object d) (object b)
               (at B home) (at P home) (at D home) (in P))
        (:goal (and (at B office) (at D office) (at P home)))
    )
    "#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::parse_domain;

    #[test]
    fn parse_domain_works() {
        let (remainder, domain) = parse_domain(BRIEFCASE_WORLD).unwrap();

        // The input was parsed completely, nothing followed the domain definition.
        assert_eq!(remainder, "");

        // All elements were parsed.
        assert_eq!(domain.requirements().len(), 4);
        assert_eq!(domain.types().len(), 2);
        assert_eq!(domain.constants().len(), 3);
        assert_eq!(domain.predicates().len(), 2);
        assert_eq!(domain.structure().len(), 3);
    }
}
