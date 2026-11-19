; Cables carry power from the source; a node is powered (derived) if a cable
; leads to it from a powered node. Cables may only be laid into dead nodes,
; and laying one costs the duct length.
(define (domain wiring)
  (:requirements :strips :typing :negative-preconditions :derived-predicates :action-costs)
  (:types node)
  (:predicates (source ?n - node) (duct ?from ?to - node)
               (cable ?from ?to - node) (powered ?n - node))
  (:functions (duct-length ?from ?to - node) - number (total-cost) - number)
  (:derived (powered ?n - node)
    (or (source ?n)
        (exists (?m - node) (and (cable ?m ?n) (powered ?m)))))
  (:action lay-cable
    :parameters (?from ?to - node)
    :precondition (and (duct ?from ?to) (not (cable ?from ?to)) (not (powered ?to)))
    :effect (and (cable ?from ?to) (increase (total-cost) (duct-length ?from ?to)))))
