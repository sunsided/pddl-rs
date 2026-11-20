(define (problem p) (:domain kitchen-sink)
  (:objects t1 - truck home shop - place)
  (:init (at t1 home) (route home shop depot) (type home) (= (fuel t1) 3) (= (load) 1))
  (:goal (and (at t1 shop) (any-route))))
