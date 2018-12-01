(ns adventofcode2018.day10
  (:require [clojure.string :refer [trim trim-newline]]))

(defn part1
  [& args])

(defn part2
  [& args])

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day10.txt" slurp trim trim-newline))]
    (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
