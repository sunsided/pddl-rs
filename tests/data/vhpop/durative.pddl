(define (domain durative-domain)
  (:requirements :durative-actions :duration-inequalities)
  (:predicates (a) (b) (c) (d))
  (:action a
	   :parameters ()
	   :effect (a))
  (:durative-action da1
		    :parameters ()
		    :duration (and (>= ?duration 10) (<= ?duration 20))
		    :condition (and (at start (a)) (at end (c)))
		    :effect (and (at end (b)) (at start (d))))
  (:durative-action da2
		    :parameters ()
		    :duration (and (>= ?duration 15) (<= ?duration 30))
		    :condition (at start (d))
		    :effect (at end (c))))

(define (problem durative-problem)
  (:domain durative-domain)
  (:goal (b)))
