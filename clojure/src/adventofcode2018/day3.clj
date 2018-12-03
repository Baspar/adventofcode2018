(ns adventofcode2018.day3
  (:require [clojure.string :refer [trim trim-newline split]]))

(defn- format-input
  [input]
  (as-> input _
       (split _ #"\n")
       (map #(re-find #"\d+ @ (\d+),(\d+): (\d+)x(\d+)" %) _)
       (map #(let [[_match x y width height] %]
               {:x (read-string x)
                :y (read-string y)
                :width (read-string width)
                :height (read-string height)}) _)))

(defn- generate-coords
  [{:keys [x y width height]}]
  (for [_x (range x (+ x width))
        _y (range y (+ y height))]
    (str _x "_" _y)))

(defn part1
  [args]
  (as-> args _
    (format-input _)
    (map generate-coords _)
    (mapcat identity _)
    (frequencies _)
    (vals _)
    (filter #(< 1 %) _)
    (count _)))

(defn part2
  [& args])

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day3.txt" slurp trim trim-newline))]
    ;; (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
