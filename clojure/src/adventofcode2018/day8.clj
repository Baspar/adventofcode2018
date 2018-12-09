(ns adventofcode2018.day8
  (:require [clojure.string :refer [trim trim-newline split]]))

(defn part1
  [args]
  (let [values (as-> args <>
                (split <> #" ")
                (map read-string <>))]
    (loop [values values
           stack nil
           total-meta 0]
      (cond
        ;; Not more value to parse
        (empty? values) total-meta
        ;; Stack empty (first iteration)
        (empty? stack) (let [[nb-nodes nb-meta] (take 2 values)
                             new-values (drop 2 values)
                             new-stack (conj stack {:nb-nodes nb-nodes
                                                    :nb-meta nb-meta})]
                         (recur new-values new-stack total-meta))
        ;; Dig into child node
        (-> stack first :nb-nodes pos?) (let [[nb-nodes nb-meta] (take 2 values)
                                              new-values (drop 2 values)
                                              new-stack (conj (rest stack)
                                                              (-> stack
                                                                  first
                                                                  (update :nb-nodes dec))
                                                              {:nb-nodes nb-nodes
                                                               :nb-meta nb-meta})]
                                          (recur new-values new-stack total-meta))
        ;; Count meta
        (-> stack first :nb-meta pos?) (let [{:keys [nb-meta]} (first stack)
                                             new-stack (pop stack)
                                             metas (take nb-meta values)
                                             new-values (drop nb-meta values)
                                             new-total-meta (apply + total-meta metas)]
                                         (recur new-values new-stack new-total-meta))
        ;; Uknwon case
        :else "Unknown"))))

(defn part2
  [& args])

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day8.txt" slurp trim trim-newline))]
    (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
