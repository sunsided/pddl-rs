; The briefcase world domain with universally quantified effects from
; the UCPOP distribution.

(define (domain uni-bw)
  (:requirements :equality :conditional-effects)
  (:constants b)
  (:predicates (at ?x ?y) (in ?x) (place ?x) (object ?x))
  (:action mov-b
	   :parameters (?m ?l)
	   :precondition (and (at b ?m) (not (= ?m ?l)))
	   :effect (and (at b ?l) (not (at b ?m))
			(forall (?z)
				(when (and (in ?z) (not (= ?z b)))
				  (and (at ?z ?l)  (not (at ?z ?m)))))))
  (:action take-out
	   :parameters (?x)
	   :precondition (not (= ?x b))
	   :effect (not (in ?x)))
  (:action put-in
	   :parameters (?x ?l)
	   :precondition (not (= ?x b))
	   :effect (when (and (at ?x ?l) (at b ?l))
		     (in ?x))))
