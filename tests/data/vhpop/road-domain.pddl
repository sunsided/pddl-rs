; A traveling domain from the UCPOP distribution.

(define (domain road-operators)
  (:predicates (at ?x ?y) (road ?x ?y) (bridge ?x ?y) (vehicle ?x) (place ?x))
  (:action drive
	   :parameters (?vehicle ?location1 ?location2)
	   :precondition (and (at ?vehicle ?location1)
			      (road ?location1 ?location2))
	   :effect (and (at ?vehicle ?location2)
			(not (at ?vehicle ?location1))))
  (:action cross
	   :parameters (?vehicle ?location1 ?location2)
	   :precondition (and (at ?vehicle ?location1)
			      (bridge ?location1 ?location2))
	   :effect (and (at ?vehicle ?location2)
			(not (at ?vehicle ?location1)))))
