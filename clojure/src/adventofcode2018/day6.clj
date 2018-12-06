(ns adventofcode2018.day6
  (:require [clojure.string :refer [trim trim-newline split]]))

;; Helper
(defn- parse-input
  [lines]
  (as-> lines <>
    (split <> #"\n")
    (map #(split % #"\s*,\s*") <>)
    (map #(map read-string %) <>)))
(defn- get-bounds
  [coords]
  (let [[xs ys] (apply mapv vector coords)]
    {:min-x (apply min xs)
     :max-x (apply max xs)
     :min-y (apply min ys)
     :max-y (apply max ys)}))
(defn- get-distance
  [[x1 y1] [x2 y2]]
  (+ (Math/abs (- x1 x2))
     (Math/abs (- y1 y2))))
(defn- get-closest-target
  [targets current]
  (as-> targets <>
    ;; Get all target distances
    (map #(get-distance % current) <>)
    ;; Compute frequencies of distances
    (map-indexed (fn [target-id dist] {:dist dist
                                       :target-id target-id
                                       :freq (-> <> (frequencies) (get dist))})
                 <>)
    ;; Group and find smallest distance(s?)
    (sort-by :dist <>)
    (partition-by :dist <>)
    (first <>)
    ;; If no tie, return target
    (when (= 1 (count <>))
      (first <>))))

;; Part1
(defn part1
  [args]
  (let [targets (parse-input args)
        {:keys [min-x min-y max-x max-y]} (get-bounds targets)
        width (inc (- max-y min-y))
        vector-closest-target (for [x (range min-x (inc max-x))
                                    y (range min-y (inc max-y))]
                                (get-closest-target targets [x y]))
        infinite-targets (as-> vector-closest-target <>
                              (map :target-id <>)
                              (partition-all width <>)
                              [(first <>)
                               (last <>)
                               (map first <>)
                               (map last <>)]
                              (flatten <>)
                              (filter some? <>)
                              (into #{} <>))]
    (println infinite-targets)
    (->> vector-closest-target
         (filter some?)
         (map :target-id)
         (filter #(not (infinite-targets %)))
         (frequencies)
         (sort-by val)
         (last)
         (val)
         )))

;; Part2
(defn part2
  [& args])

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day6.txt" slurp trim trim-newline))]
    (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
