(define (domain hanoi-3-domain)
  (:requirements :equality :disjunctive-preconditions)
  (:constants d1 d2 d3)
  (:predicates (thing ?x) (on ?x ?y) (clear ?x))

  (:action move-d1
           :parameters (?from ?to)
           :precondition (and (thing ?from) (thing ?to) (on d1 ?from) (clear ?to) (not (on d1 ?to)) (not (= ?to d1)))
           :effect (and (on d1 ?to) (not (clear ?to)) (clear ?from) (not (on d1 ?from))))

  (:action move-d2
           :parameters (?from ?to)
           :precondition (and (thing ?from) (thing ?to) (on d2 ?from) (clear ?to) (not (on d2 ?to)) (not (on d1 d2)) (not (= ?to d1)) (not (= ?to d2)))
           :effect (and (on d2 ?to) (not (clear ?to)) (clear ?from) (not (on d2 ?from))))

  (:action move-d3
           :parameters (?from ?to)
           :precondition (and (thing ?from) (thing ?to) (on d3 ?from) (clear ?to) (not (on d3 ?to)) (not (on d1 d3)) (not (on d2 d3)) (not (= ?to d1)) (not (= ?to d2)) (not (= ?to d3)))
           :effect (and (on d3 ?to) (not (clear ?to)) (clear ?from) (not (on d3 ?from))))
)
