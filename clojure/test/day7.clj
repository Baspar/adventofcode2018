(ns day7
  (:require [clojure.test :refer [deftest testing is]]
            [adventofcode2018.day7 :refer [part1 part2]]))

(deftest test-part-1
  (testing "FIXME, I fail."
    (is (= (part1 "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.") "CABDFE"))))

(deftest test-part-2
  (testing "FIXME, I fail."
    (is (= (part2 2 0 "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.") 15))))
