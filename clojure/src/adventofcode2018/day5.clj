(ns adventofcode2018.day5
  (:require [clojure.string :refer [trim trim-newline upper-case]]))

(defn part1
  [input]
  (let [units (map identity input)]
    (as-> units <>
      (reduce (fn [stack letter]
                (cond
                  (empty? stack) (list letter)
                  (= (first stack) letter) (conj stack letter)
                  (not= (upper-case (first stack)) (upper-case letter)) (conj stack letter)
                  :else (pop stack)))
              (list)
              <>)
      (reverse <>)
      (count <>))))

(defn part2
  [& args])

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day5.txt" slurp trim trim-newline))]
    (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
