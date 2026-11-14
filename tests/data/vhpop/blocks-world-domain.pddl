; The 1-operator blocks world domain from the UCPOP distribution.

(define (domain blocks-world-domain)
  (:requirements :equality :conditional-effects)
  (:constants table)
  (:predicates (on ?x ?y) (clear ?x) (block ?x))
  (:action puton
	   :parameters (?x ?y ?z)
	   :precondition (and (on ?x ?z) (clear ?x) (clear ?y)
			      (not (= ?y ?z)) (not (= ?x ?z))
			      (not (= ?x ?y)) (not (= ?x table)))
	   :effect (and (on ?x ?y) (not (on ?x ?z))
			(when (not (= ?z table)) (clear ?z))
			(when (not (= ?y table)) (not (clear ?y))))))
