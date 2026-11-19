; Switches that can only be turned on; written for these tests.
(define (domain switches)
  (:requirements :strips :negative-preconditions)
  (:predicates (on ?s))
  (:action flip
    :parameters (?s)
    :precondition (not (on ?s))
    :effect (on ?s)))
