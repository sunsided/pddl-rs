(define (domain logistics)
  (:requirements :typing)
  (:types truck place)
  (:predicates (at ?t - truck ?p - place))
  (:action drive
    :parameters (?t - truck ?from ?to - place)
    :precondition (at ?t ?from)
    :effect (and (not (at ?t ?from)) (at ?t ?to))))
