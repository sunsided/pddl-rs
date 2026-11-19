; A van carries loaded parcels along when it drives (conditional effect);
; driving costs the road length, loading and unloading cost 1.
(define (domain courier)
  (:requirements :strips :typing :conditional-effects :action-costs)
  (:types location parcel)
  (:predicates (van-at ?l - location) (at ?p - parcel ?l - location)
               (loaded ?p - parcel) (road ?from ?to - location))
  (:functions (road-length ?from ?to - location) - number (total-cost) - number)
  (:action drive
    :parameters (?from ?to - location)
    :precondition (and (van-at ?from) (road ?from ?to))
    :effect (and (not (van-at ?from)) (van-at ?to)
                 (forall (?p - parcel)
                   (when (loaded ?p) (and (not (at ?p ?from)) (at ?p ?to))))
                 (increase (total-cost) (road-length ?from ?to))))
  (:action load
    :parameters (?p - parcel ?l - location)
    :precondition (and (van-at ?l) (at ?p ?l) (not (loaded ?p)))
    :effect (and (loaded ?p) (increase (total-cost) 1)))
  (:action unload
    :parameters (?p - parcel ?l - location)
    :precondition (and (van-at ?l) (loaded ?p))
    :effect (and (not (loaded ?p)) (increase (total-cost) 1))))
