(ns adventofcode2018.day7
  (:require [clojure.string :refer [trim trim-newline split join]]))

(defn- parse-input
  [input]
  (let [re #"Step (.*) must be finished before step (.*) can begin."]
    (as-> input <>
         (split <> #"\n")
         (map #(re-find re %) <>)
         (map (fn [[_ from to]] [from to]) <>))))
(defn part1
  [args]
  (let [edges (parse-input args)
        empty-graph (as-> edges <>
                      (apply concat <>)
                      (map (fn [x] [x '()]) <>)
                      (into {} <>))
        graph (reduce (fn [g [from to]]
                        (update g to conj from))
                      empty-graph
                      edges)]
    (loop [graph graph
           out []]
      (if (empty? graph)
        (join "" out)
        (let [next-node (as-> graph <>
                          (filter (fn [[_ v]] (zero? (count v))) <>)
                          (sort-by first <>)
                          (first <>)
                          (first <>))
              next-graph (as-> graph <>
                           (dissoc <> next-node)
                           (map (fn [[k v]] [k (filter #(not= next-node %) v)]) <>)
                           (into {} <>))]
          (recur next-graph (conj out next-node)))))))

(defn part2
  [& args])

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day7.txt" slurp trim trim-newline))]
    (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
