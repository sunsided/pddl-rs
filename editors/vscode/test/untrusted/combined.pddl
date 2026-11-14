(define (domain d)
  (:predicates (ready)))

(define (problem p)
  (:domain d)
  (:init)
  (:goal (done)))
