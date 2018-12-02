(ns adventofcode2018.day2
  (:require [clojure.string :refer [trim trim-newline split join]]))

;; Helper
(defn- format-input
  [input]
  (split input #"\n"))

;; Part1
(defn get-2-3-freq [input]
  (as-> input _
    (frequencies _)
    (vals _)
    (into #{} _)
    [(if (get _ 2) 1 0) (if (get _ 3) 1 0)]))
(defn sum-2-3-freq
  [f1 f2]
  (map + f1 f2))
(defn part1
  [args]
  (->> args
       (format-input)
       (map get-2-3-freq)
       (reduce sum-2-3-freq)
       (apply *)))

;; Part2
(defn- distance-of-1?
  [[w1 w2]]
  (->> [w1 w2]
       (apply map #(if (= %1 %2) 0 1))
       (reduce +)
       (= 1)))
(defn generate-combination
  [inputs]
  (for [w1 inputs
        w2 inputs]
    [w1 w2]))
(defn get-common-letters
  [words]
  (->> words
       (apply map #(when (= %1 %2) %1))
       (filter identity)
       (join "")))
(defn part2
  [args]
  (->> args
       (format-input)
       (generate-combination)
       (filter distance-of-1?)
       (first)
       (get-common-letters)))

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day2.txt" slurp trim trim-newline))]
    (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
