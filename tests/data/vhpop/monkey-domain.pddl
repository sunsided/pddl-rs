; The monkey domain from the UCPOP distribution.

(define (domain monkey-domain)
  (:requirements :equality)
  (:constants monkey box knife bananas glass water waterfountain)
  (:predicates (on-floor) (at ?x ?y) (onbox ?x) (hasknife) (hasbananas)
	       (hasglass) (haswater) (location ?x))
  (:action go-to
	   :parameters (?x ?y)
	   :precondition (and (not (= ?y ?x)) (on-floor) (at monkey ?y))
	   :effect (and (at monkey ?x) (not (at monkey ?y))))
  (:action climb
	   :parameters (?x)
	   :precondition (and (at box ?x) (at monkey ?x))
	   :effect (and (onbox ?x) (not (on-floor))))
  (:action push-box
	   :parameters (?x ?y)
	   :precondition (and (not (= ?y ?x)) (at box ?y) (at monkey ?y)
			      (on-floor))
	   :effect (and (at monkey ?x) (not (at monkey ?y))
			(at box ?x) (not (at box ?y))))
  (:action get-knife
	   :parameters (?y)
	   :precondition (and (at knife ?y) (at monkey ?y))
	   :effect (and (hasknife) (not (at knife ?y))))
  (:action grab-bananas
	   :parameters (?y)
	   :precondition (and (hasknife) (at bananas ?y) (onbox ?y))
	   :effect (hasbananas))
  (:action pickglass
	   :parameters (?y)
	   :precondition (and (at glass ?y) (at monkey ?y))
	   :effect (and (hasglass) (not (at glass ?y))))
  (:action getwater
	   :parameters (?y)
	   :precondition (and (hasglass)
			      (at waterfountain ?y)
			      (at monkey ?y)
			      (onbox ?y))
	   :effect (haswater)))
