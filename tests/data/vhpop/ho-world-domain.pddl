; The ho-world domain from the UCPOP distribution.

(define (domain ho-world)
  (:requirements :equality :negative-preconditions :conditional-effects)
  (:constants wall off on plumbing)
  (:predicates (object ?x) (good-plumbing) (holey-walls) (water ?x))
  (:action fix
	   :parameters (?it)
	   :precondition (object ?it)
	   :effect (and (when (and (= ?it wall) (good-plumbing))
			  (not (holey-walls)))
			(when (and (= ?it wall) (not (good-plumbing))
				   (water off))
			  (not (holey-walls)))
			(when (and (= ?it plumbing) (water off))
			  (good-plumbing))))
  (:action turn-faucet
	   :parameters (?how)
	   :effect (and (water ?how)
			(forall (?s)
				(when (and (not (= ?s ?how)) (water ?s))
				  (not (water ?s))))
			(when (and (= ?how on) (not (good-plumbing)))
			  (holey-walls)))))
