(ns day1
  (:require [clojure.test :refer [deftest testing is]]
            [adventofcode2018.day1 :refer [part1 part2]]))

(deftest test-part-1
  (testing "FIXME, I fail."
    (is (= (part1 "+1\n+1\n+1") 3))
    (is (= (part1 "+1\n+1\n-2") 0))
    (is (= (part1 "-1\n-2\n-3") -6))))

(deftest test-part-2
  (testing "FIXME, I fail."
    (is (= (part2 "+1\n-1") 0))
    (is (= (part2 "+3\n+3\n+4\n-2\n-4") 10))
    (is (= (part2 "-6\n+3\n+8\n+5\n-6") 5))
    (is (= (part2 "+7\n+7\n-2\n-7\n-4") 14))))
