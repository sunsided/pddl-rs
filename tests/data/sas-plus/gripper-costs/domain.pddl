; Gripper with action costs: moving costs 3, picking and dropping cost 1.
(define (domain gripper-costs)
  (:requirements :strips :typing :action-costs)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:functions (total-cost) - number)
  (:action move
    :parameters (?from ?to - room)
    :precondition (at-robby ?from)
    :effect (and (at-robby ?to) (not (at-robby ?from)) (increase (total-cost) 3)))
  (:action pick
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g)) (increase (total-cost) 1)))
  (:action drop
    :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)) (increase (total-cost) 1))))
