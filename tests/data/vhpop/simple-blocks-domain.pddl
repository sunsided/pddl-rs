; Blocks world domain used in (Nguyen & Kambhampati 2001).

(define (domain simple-blocks)
  (:requirements :equality)
  (:constants table)
  (:predicates (on ?x ?y) (clear ?x))
  (:action put-table
	   :parameters (?x ?z)
	   :precondition (and (on ?x ?z) (clear ?x) (not (= ?x table))
			      (not (= ?z table)) (not (= ?x ?z)))
	   :effect (and (on ?x table) (clear ?z) (not (on ?x ?z))))
  (:action put
	   :parameters (?x ?y ?z)
	   :precondition (and (on ?x ?z) (clear ?x) (clear ?y)
			      (not (= ?x table)) (not (= ?y table))
			      (not (= ?x ?y)) (not (= ?y ?z)) (not (= ?x ?z)))
	   :effect (and (on ?x ?y) (clear ?z)
			(not (on ?x ?z)) (not (clear ?y)))))
