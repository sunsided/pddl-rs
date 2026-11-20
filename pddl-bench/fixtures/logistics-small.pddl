; One package from the post office of pgh to the post office of bos, for
; the logistics-e domain: by truck to the airport, by plane, by truck again.
; Optimal: 9 actions (load, drive, unload; load, fly, unload; load, drive, unload).
(define (problem log-small)
  (:domain logistics-strips)
  (:objects package1 airplane1 pgh bos pgh-truck bos-truck
            pgh-po pgh-airport bos-po bos-airport)
  (:init (obj package1) (airplane airplane1)
         (city pgh) (city bos) (truck pgh-truck) (truck bos-truck)
         (location pgh-po) (location pgh-airport) (location bos-po) (location bos-airport)
         (airport pgh-airport) (airport bos-airport)
         (in-city pgh-po pgh) (in-city pgh-airport pgh)
         (in-city bos-po bos) (in-city bos-airport bos)
         (at package1 pgh-po) (at pgh-truck pgh-po) (at bos-truck bos-airport)
         (at airplane1 pgh-airport))
  (:goal (at package1 bos-po)))
