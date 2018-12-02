(ns adventofcode2018.day2
  (:require [clojure.string :refer [trim trim-newline split]]))

(defn- format-input
  [input]
  (split input #"\n"))

(defn part1
  [args]
  (let [inputs (format-input args)]
    (->> inputs
         (map
           (fn [input]
             (as-> input _
               (frequencies _)
               (vals _)
               (into #{} _)
               [(if (get _ 2) 1 0) (if (get _ 3) 1 0)])))
         (reduce (fn [[a1 b1] [a2 b2]]
                   [(+ a1 a2) (+ b1 b2)]))
         (apply *))))

(defn part2
  [args])

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day2.txt" slurp trim trim-newline))]
    (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
