(ns adventofcode2018.day1
  (:require [clojure.string :refer [trim trim-newline split]]))

(defn- format-input
  [string]
  (as-> string _
    (split _ #" ")
    (map read-string _)))

(defn part1
  [args]
  (let [int-list (format-input args)]
    (reduce + int-list)))

(defn part2
  [args]
  (let [int-list (format-input args)
        infinite-int-list (-> int-list repeat flatten)]
    (loop [mem 0
           seen #{0}
           infinite-int-list infinite-int-list]
      (let [new-value (first infinite-int-list)
            new-mem (+ mem new-value)
            new-seen (conj seen new-mem)
            already-seen? (get seen new-mem)]
        (if (not= nil already-seen?)
          new-mem
          (recur new-mem new-seen (rest infinite-int-list)))))))

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day1.txt" slurp trim trim-newline))]
    (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
