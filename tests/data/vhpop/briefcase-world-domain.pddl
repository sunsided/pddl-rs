; The briefcase world domain from the UCPOP distribution.

(define (domain briefcase-world)
  (:requirements :equality :conditional-effects)
  (:constants b p d)
  (:predicates (at ?x ?y) (in ?x) (place ?x) (object ?x))
  (:action mov-b
	   :parameters (?m ?l)
	   :precondition (and (not (= ?m ?l)) (at b ?m))
	   :effect (and (at b ?l) (not (at b ?m))
			(when (in p)
			  (and (at p ?l) (not (at p ?m))))
			(when (in d)
			  (and (at d ?l) (not (at d ?m))))))
  (:action take-out
	   :parameters (?x)
	   :precondition (not (= ?x b))
	   :effect (not (in ?x)))
  (:action put-in
	   :parameters (?x ?l)
	   :precondition (not (= ?x b))
	   :effect (when (and (at ?x ?l) (at b ?l))
		     (in ?x))))
