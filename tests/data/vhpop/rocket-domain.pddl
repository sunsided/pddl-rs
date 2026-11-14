; Rocket domain used in (Nguyen & Kambhampati 2001).

(define (domain rockets)
  (:predicates (cargo ?object)
	       (rocket ?rocket)
	       (place ?place)
	       (at ?object ?place)
	       (in ?object ?rocket)
	       (has-fuel ?rocket)
	       (no-fuel ?rocket))
  (:action load
	   :parameters (?object ?rocket ?place)
	   :precondition (and (cargo ?object) (rocket ?rocket) (place ?place)
			      (at ?rocket ?place) (at ?object ?place))
	   :effect (and (in ?object ?rocket) (not (at ?object ?place))))
  (:action unload
	   :parameters (?object ?rocket ?place)
	   :precondition (and (cargo ?object) (rocket ?rocket) (place ?place)
			      (at ?rocket ?place) (in ?object ?rocket))
	   :effect (and (at ?object ?place) (not (in ?object ?rocket))))
  (:action move
	   :parameters (?rocket ?from ?to)
	   :precondition (and (rocket ?rocket) (place ?from) (place ?to)
			      (has-fuel ?rocket) (at ?rocket ?from))
	   :effect (and (at ?rocket ?to) (no-fuel ?rocket)
			(not (has-fuel ?rocket)) (not (at ?rocket ?from))))
  (:action refuel
	   :parameters (?rocket)
	   :precondition (and (rocket ?rocket) (no-fuel ?rocket))
	   :effect (and (has-fuel ?rocket) (not (no-fuel ?rocket)))))
