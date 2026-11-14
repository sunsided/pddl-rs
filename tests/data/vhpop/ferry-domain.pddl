; The ferry domain from the UCPOP distribution.

(define (domain ferry-domain)
  (:requirements :equality)
  (:constants ferry)
  (:predicates (auto ?x) (place ?x) (at ?x ?y) (at-ferry ?x) (empty-ferry)
	       (on ?x ?y))
  (:action board
	   :parameters (?x ?y)
	   :precondition (and (auto ?x) (place ?y)
			      (at ?x ?y) (at-ferry ?y) (empty-ferry))
	   :effect (and (on ?x ferry)
			(not (at ?x ?y))
			(not (empty-ferry))))
  (:action sail
	   :parameters (?x ?y)
	   :precondition (and (place ?x) (place ?y)
			      (at-ferry ?x) (not (= ?x ?y)))
	   :effect (and (at-ferry ?y)
			(not (at-ferry ?x))))
  (:action debark
	   :parameters (?x ?y)
	   :precondition (and (auto ?x) (place ?y)
			      (on ?x ferry) (at-ferry ?y))
	   :effect (and (not (on ?x ferry))
			(at ?x ?y)
			(empty-ferry))))
