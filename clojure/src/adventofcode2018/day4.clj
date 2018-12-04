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

;; Part1
(defn- sort-date
  [_d1 _d2]
  (loop [d1 (_d1 :time)
         d2 (_d2 :time)]
    (cond
      (empty? d1) -1
      (< (first d1) (first d2)) -1
      (> (first d1) (first d2)) 1
      :else (recur (rest d1) (rest d2)))))
(defn- group-by-shift
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
(defn- find-best-minute
  [{:keys [guard ranges]}]
  (let [[froms tos] (apply mapv vector ranges)
        sorted-froms (sort froms)
        sorted-tos (sort tos)]
    (loop [froms sorted-froms
           tos sorted-tos
           current-count 0
           max-count 0
           max-at -1]
      (if (or (empty? tos) (empty? froms))
        {:guard guard
         :max-count max-count
         :max-at max-at}
        (let [take-from-tos? (< (first tos) (first froms))
              when-take-from-tos [froms (rest tos) (first tos) (dec current-count)]
              when-take-from-froms [(rest froms) tos (first froms) (inc current-count)]
              [next-froms next-tos next-max-at next-current-count] (if take-from-tos?
                                                                     when-take-from-tos
                                                                     when-take-from-froms)
              update-max? (< max-count next-current-count)]
          (recur next-froms
                 next-tos
                 next-current-count
                 (if update-max? next-current-count max-count)
                 (if update-max? next-max-at max-at)))))))
(defn part1
  [args]
  (let [best-shift (->> args
                        (format-input)
                        (sort sort-date)
                        (group-by-shift)
                        (map compute-sleep)
                        (group-by-guard)
                        (sort-by :sleep-time)
                        (last)
                        (find-best-minute))]
    (* (best-shift :guard)
       (best-shift :max-at))))

;; Part2
(defn part2
  [args])

(defn -main [& args]
  (let [input (or (first args)
                  (-> "../inputs/day4.txt" slurp trim trim-newline))]
    ;; (println "input:" input)
    (println "Part1")
    (println (part1 input))
    (println "Part2")
    (println (part2 input))))
