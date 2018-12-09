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
  [args]
  (let [values (as-> args <>
                 (split <> #" ")
                 (map read-string <>))]
    (loop [values values
           stack nil
           children-values nil]
      (cond
        ;; Not more value to parse
        (empty? values) (-> children-values first first)
        ;; Stack empty (first iteration)
        (empty? stack) (let [[nb-nodes nb-meta] (take 2 values)
                             new-values (drop 2 values)
                             new-stack (conj stack {:nb-nodes nb-nodes
                                                    :nb-meta nb-meta})]
                         (recur new-values new-stack (conj children-values [])))
        ;; Dig into child node
        (-> stack first :nb-nodes pos?) (let [[nb-nodes nb-meta] (take 2 values)]
                                          (if (zero? nb-nodes)
                                            (let [new-values (->> values (drop 2) (drop nb-meta))
                                                  new-stack (conj (rest stack)
                                                                  (-> stack
                                                                      first
                                                                      (update :nb-nodes dec)))
                                                  metas (->> values (drop 2) (take nb-meta))
                                                  node-value (reduce + metas)
                                                  new-children-values (conj (rest children-values)
                                                                            (vec (conj (first children-values) node-value)))]
                                              (recur new-values new-stack new-children-values))
                                            (let [new-values (drop 2 values)
                                                  new-stack (conj (rest stack)
                                                                  (-> stack
                                                                      first
                                                                      (update :nb-nodes dec))
                                                                  {:nb-nodes nb-nodes
                                                                   :nb-meta nb-meta})
                                                  new-children-values (conj children-values [])]
                                              (recur new-values new-stack new-children-values))))
        ;; Sum child
        (-> stack first :nb-meta pos?) (let [{:keys [nb-meta]} (first stack)
                                             new-stack (pop stack)
                                             metas (take nb-meta values)
                                             new-values (drop nb-meta values)
                                             used-children-value (first children-values)
                                             node-value (->> metas
                                                             (map dec)
                                                             (map #(get used-children-value % 0))
                                                             (reduce +))
                                             new-children-values (as-> children-values <>
                                                                   (pop <>)
                                                                   (conj (rest <>) (vec (conj (first <>) node-value))))]
                                         (recur new-values new-stack new-children-values))
        ;; Uknwon case
        :else "Unknown"))))

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day8.txt" slurp trim trim-newline))]
    (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
