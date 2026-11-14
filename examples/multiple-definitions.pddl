; Two domains and three problems in one file. Each problem uses the domain
; named in its (:domain ...), so the same names can mean different things.

; Trucks drive packages between places, using fuel.
(define (domain logistics)
  (:requirements :strips :typing :numeric-fluents)
  (:types
    truck package - object
    place)
  (:constants depot - place)
  (:predicates
    (at ?o - object ?p - place)
    (in ?pkg - package ?t - truck)
    (road ?from ?to - place))
  (:functions (fuel ?t - truck) - number)

  (:action drive
    :parameters (?t - truck ?from ?to - place)
    :precondition (and (at ?t ?from) (road ?from ?to) (>= (fuel ?t) 1))
    :effect (and (not (at ?t ?from)) (at ?t ?to) (decrease (fuel ?t) 1)))

  (:action load
    :parameters (?pkg - package ?t - truck ?p - place)
    :precondition (and (at ?pkg ?p) (at ?t ?p))
    :effect (and (not (at ?pkg ?p)) (in ?pkg ?t)))

  (:action unload
    :parameters (?pkg - package ?t - truck ?p - place)
    :precondition (and (in ?pkg ?t) (at ?t ?p))
    :effect (and (not (in ?pkg ?t)) (at ?pkg ?p))))

; A ferry carries cars across a river, one at a time.
(define (domain ferry)
  (:requirements :strips)
  (:predicates
    (at ?x ?bank)
    (at-ferry ?bank)
    (on ?car)
    (empty-ferry))

  (:action board
    :parameters (?car ?bank)
    :precondition (and (at ?car ?bank) (at-ferry ?bank) (empty-ferry))
    :effect (and (on ?car) (not (at ?car ?bank)) (not (empty-ferry))))

  (:action sail
    :parameters (?from ?to)
    :precondition (at-ferry ?from)
    :effect (and (at-ferry ?to) (not (at-ferry ?from))))

  (:action debark
    :parameters (?car ?bank)
    :precondition (and (on ?car) (at-ferry ?bank))
    :effect (and (at ?car ?bank) (empty-ferry) (not (on ?car)))))

(define (problem deliver-one)
  (:domain logistics)
  (:objects
    t1 - truck
    parcel - package
    shop - place)
  (:init
    (at t1 depot)
    (at parcel depot)
    (road depot shop)
    (= (fuel t1) 5))
  (:goal (at parcel shop)))

(define (problem deliver-two)
  (:domain logistics)
  (:objects
    t1 t2 - truck
    letter box - package
    shop home - place)
  (:init
    (at t1 depot)
    (at t2 home)
    (at letter depot)
    (at box home)
    (road depot shop)
    (road home shop)
    (= (fuel t1) 2)
    (= (fuel t2) 1))
  (:goal (and (at letter shop) (at box shop))))

; `at` here is the ferry's `at`, with untyped arguments.
(define (problem cross-river)
  (:domain ferry)
  (:objects car1 car2 north south)
  (:init
    (at car1 north)
    (at car2 north)
    (at-ferry north)
    (empty-ferry))
  (:goal (and (at car1 south) (at car2 south))))
