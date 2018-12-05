(ns day5
  (:require [clojure.test :refer [deftest testing is]]
            [adventofcode2018.day5 :refer [part1 part2]]))

(deftest test-part-1
  (testing "FIXME, I fail."
    (is (= (part1 "aA") 0))
    (is (= (part1 "abBA") 0))
    (is (= (part1 "abAB") 4))
    (is (= (part1 "aabAAB") 6))
    (is (= (part1 "dabAcCaCBAcCcaDA") 10))))

(deftest test-part-2
  (testing "FIXME, I fail."
    (is (= 0 1))))
