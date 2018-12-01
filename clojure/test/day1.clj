(ns day1
  (:require [clojure.test :refer [deftest testing is]]
            [adventofcode2018.day1 :refer [part1 part2]]))

(deftest test-part-1
  (testing "FIXME, I fail."
    (is (= (part1 "+1 +1 +1") 3))
    (is (= (part1 "+1 +1 -2") 0))
    (is (= (part1 "-1 -2 -3") -6))))

(deftest test-part-2
  (testing "FIXME, I fail."
    (is (= (part2 "+1 -1") 0))
    (is (= (part2 "+3 +3 +4 -2 -4") 10))
    (is (= (part2 "-6 +3 +8 +5 -6") 5))
    (is (= (part2 "+7 +7 -2 -7 -4") 14))))
