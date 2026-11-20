; Exercises the generator's less common paths: a ternary predicate, an
; `either` parameter, a supertype parameter that needs a type check, names
; that aren't Rust identifiers, numeric operators and comparisons, numeric
; effects under forall and when, and a nullary derived predicate.
(define (domain kitchen-sink)
  (:requirements :adl :typing :numeric-fluents :derived-predicates)
  (:types vehicle - object truck - vehicle place self)
  (:constants depot - place)
  (:predicates (route ?a ?b ?c - place) (at ?t - truck ?p - place) (self) (type ?x) (any-route))
  (:functions (fuel ?t - truck) (load))
  (:derived (any-route) (exists (?a ?b - place) (route ?a ?b depot)))
  (:action go
    :parameters (?v - vehicle ?a ?b - place ?w - (either truck place))
    :precondition (and (at ?v ?a) (route ?a ?b depot) (or (or) (at ?v ?a)) (type ?w)
                       (< (- (fuel ?v)) (* 2 (/ (fuel ?v) 3)))
                       (> (+ (load) 1) 0))
    :effect (and (not (at ?v ?a)) (at ?v ?b) (not (self))
                 (forall (?t - truck) (when (at ?t ?b) (increase (fuel ?t) 1)))
                 (decrease (load) 1) (scale-up (load) 2) (scale-down (load) 2))))
