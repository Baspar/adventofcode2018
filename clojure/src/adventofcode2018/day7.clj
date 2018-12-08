(ns adventofcode2018.day7
  (:require [clojure.string :refer [trim trim-newline split join]]))

;; Helper
(defn- parse-input
  [input]
  (let [re #"Step (.*) must be finished before step (.*) can begin."]
    (as-> input <>
      (split <> #"\n")
      (map #(re-find re %) <>)
      (map (fn [[_ from to]] [(first from) (first to)]) <>))))

;; Part1
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
        (let [next-task (as-> graph <>
                          (filter (fn [[_ v]] (zero? (count v))) <>)
                          (sort-by first <>)
                          (first <>)
                          (first <>))
              next-graph (as-> graph <>
                           (dissoc <> next-task)
                           (map (fn [[k v]] [k (filter #(not= next-task %) v)]) <>)
                           (into {} <>))]
          (recur next-graph (conj out next-task)))))))

;; Part2
(defn part2
  [max-workers min-time args]
  (let [edges (parse-input args)
        ;; Initialize graph with node only
        empty-graph (->> edges
                         (apply concat)
                         (map (fn [x] [x '()]))
                         (into {}))
        ;; Add edge to the graph
        graph (reduce (fn [g [from to]]
                        (update g to conj from))
                      empty-graph
                      edges)
        char->time (fn [c] (+ (int c) 1 min-time (- (int \A))))]
    (loop [graph graph
           total-time 0
           workers []]
      (let [;; Total worker - busy workers
            number-available-workers (- max-workers (count workers))
            ;; Find tasks without dependencies, and take them up to the number of worker available
            tasks-to-be-taken (->> graph
                                   (filter (fn [[_ v]] (zero? (count v))))
                                   (sort-by key)
                                   (map key)
                                   (take number-available-workers)
                                   (into #{}))]
        (cond
          ;; If graph fully discovered, and no more working worker, return
          (and (empty? graph) (empty? workers)) total-time
          ;; If cannot take any task (Worker busy, or no task without dependencies), run workers
          ;; Look for the next worker to be done, add his time, and let him finish his job
          (empty? tasks-to-be-taken) (let [;; When is the next worker gonna be over ?
                                           min-time (->> workers (map :t) (apply min))
                                           tasks-running (->> workers
                                                              (keep (fn [{:keys [c t]}]
                                                                      (when (= min-time t) c)))
                                                              (into #{}))
                                           ;; Remove edges from next worker(s) to be done
                                           new-graph (as-> graph <>
                                                       (map (fn [[k v]] [k (filter #(not (get tasks-running %)) v)]) <>)
                                                       (into {} <>))
                                           ;; Bump time
                                           new-total-time (+ min-time total-time)
                                           ;; Remove worker done, and impact time on other workers
                                           new-workers (->> workers
                                                            (map (fn [{:keys [c t]}] {:c c
                                                                                      :t (- t min-time)}))
                                                            (filter #(-> % (get :t) zero? not)))]
                                       (recur new-graph
                                              new-total-time
                                              new-workers))
          ;; Else, take new tasks up to number of worker available
          :else (let [;; Remove node only from the graph.
                      ;; Edge will be removed when task is completed
                      new-graph (reduce (fn [m task] (dissoc m task))
                                        graph
                                        tasks-to-be-taken)
                      ;; Old worker + task to be taken formatted (Time + character)
                      new-workers (->> tasks-to-be-taken
                                       (map (fn [c] {:c c
                                                     :t (char->time c)}))
                                       (into workers))]
                  (recur new-graph
                         total-time
                         new-workers)))))))

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day7.txt" slurp trim trim-newline))]
    (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 5 60 input))))
