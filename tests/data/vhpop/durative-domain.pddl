(define (domain durative-domain)
  (:requirements :durative-actions)
  (:predicates (a) (b) (c))
  (:durative-action da
		    :parameters ()
		    :duration (= ?duration 0.0001)
		    :condition (and (over all (a)) (at end (a)))
		    :effect (and (at start (a)) (at end (b)))))
