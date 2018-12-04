(ns adventofcode2018.day4
  (:require [clojure.string :refer [trim trim-newline split join]]))

;; Helper
(defn- format-input
  [input]
  (as-> input _
    (split _ #"\n")
    (map #(re-find #"\[(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2})\].*(asleep|up|#(\d+))" %) _)
    (map (fn [[_ year month day hour minute instruction guard]]
           (let [trim-zero (fn [x] (let [o (clojure.string/replace x #"^0*" "")]
                                     (if (empty? o) "0" o)))
                 year (-> year trim-zero read-string)
                 month (-> month trim-zero read-string)
                 day (-> day trim-zero read-string)
                 hour (-> hour trim-zero read-string)
                 minute (-> minute trim-zero read-string)
                 m {:time [year month day hour minute]
                    :day [year month day]}]
             (cond
               (= instruction "up") (assoc m :instruction :wake-up)
               (= instruction "asleep") (assoc m :instruction :sleep)
               :else (-> m
                         (assoc :instruction :start-shift)
                         (assoc :guard (read-string guard)))))) _)))
(defn- sort-date
  [_d1 _d2]
  (loop [d1 (_d1 :time)
         d2 (_d2 :time)]
    (cond
      (empty? d1) -1
      (< (first d1) (first d2)) -1
      (> (first d1) (first d2)) 1
      :else (recur (rest d1) (rest d2)))))
(defn- fold-by-shift
  [data]
  (as-> data _
       (partition-by :guard _)
       (mapcat (fn [a b]
                 (let [a-guard? ((first a) :guard)
                       b-guard? ((first b) :guard)]
                   (when (and a-guard? (not b-guard?))
                     (map #(assoc % :guard a-guard?) b))))
               _
               (rest _))
       (partition-all 2 _)))
(defn- compute-sleep
  [[from to]]
  (let [hour-from (-> from :time (get 3))
        min-from (-> from :time (get 4))
        hour-to (-> to :time (get 3))
        min-to (-> to :time (get 4))
        sleep-time (- (+ (* 60 hour-to) min-to)
                      (+ (* 60 hour-from) min-from))]
    {:guard (from :guard)
     :sleep-time sleep-time
     :range [min-from min-to]}))
(defn- group-by-guard
  [shifts]
  (->> shifts
       (group-by :guard)
       (map (fn [[guard shifts]]
              (let [total-sleep-time (->> shifts
                                          (map :sleep-time)
                                          (reduce +))]
                {:guard guard
                 :sleep-time total-sleep-time
                 :ranges (mapv :range shifts)})))))
(defn- find-all-minutes
  [{:keys [guard ranges]}]
  (let [all-minutes (->> ranges
                         (mapcat (fn [[from to]] (range from to)))
                         (frequencies))]
    {:guard guard
     :all-minutes all-minutes}))

;; Part1
(defn- find-best-minute
  [{:keys [guard all-minutes]}]
  (let [[best-minute _number-of-time] (->> all-minutes
                                           (sort-by second)
                                           (last))]
    {:guard guard
     :best-minute best-minute}))
(defn part1
  [args]
  (let [best-shift (->> args
                        (format-input)
                        (sort sort-date)
                        (fold-by-shift)
                        (map compute-sleep)
                        (group-by-guard)
                        (sort-by :sleep-time)
                        (last)
                        (find-all-minutes)
                        (find-best-minute))]
    (* (best-shift :guard)
       (best-shift :best-minute))))

;; Part2
(defn- find-most-freq-sleepy-minute
  [guards-sleep-freq]
  (->> guards-sleep-freq
       (mapcat (fn [{:keys [guard all-minutes]}]
                 (map (fn [[minute freq]] {:guard guard
                                           :minute minute
                                           :freq freq})
                      all-minutes)))
       (sort-by :freq)
       (last)))
(defn part2
  [args]
  (let [best-shift (->> args
                        (format-input)
                        (sort sort-date)
                        (fold-by-shift)
                        (map compute-sleep)
                        (group-by-guard)
                        (map find-all-minutes)
                        (find-most-freq-sleepy-minute)
                        ;; (last)
                        )]
    (* (best-shift :guard)
       (best-shift :minute))
    ))

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day4.txt" slurp trim trim-newline))]
    ;; (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
