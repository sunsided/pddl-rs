; Optimal: pick, pick (1 + 1), move (3), drop, drop (1 + 1) = 7.
(define (problem gripper-costs-2)
  (:domain gripper-costs)
  (:objects rooma roomb - room ball1 ball2 - ball left right - gripper)
  (:init (at-robby rooma) (free left) (free right)
         (at ball1 rooma) (at ball2 rooma)
         (= (total-cost) 0))
  (:goal (and (at ball1 roomb) (at ball2 roomb)))
  (:metric minimize (total-cost)))
