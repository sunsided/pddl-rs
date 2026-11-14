(define (problem ho-demo)
  (:domain ho-world)
  (:init (object wall) (object plumbing) (holey-walls) (water on))
  (:goal (and (water on) (not (holey-walls)))))
