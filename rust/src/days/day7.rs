use regex::Regex;
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<(String, String)> {
    let re = Regex::new(r"Step (.*) must be finished before step (.*) can begin.").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let from = String::from(&cap[1]);
            let to = String::from(&cap[2]);
            (from, to)
        })
    .collect()
}
pub fn part1(input: &str) -> String {
    let edges = parse_input(input);

    // Create graph
    let mut graph:HashMap<&String, Vec<&String>> = HashMap::new();
    for (to, from) in &edges {
        graph.entry(to).or_insert(vec![]);
        let i = graph.entry(from).or_insert(vec![]);
        (*i).push(to);
    }

    let mut out = String::from("");
    while !graph.is_empty() {
        let mut potential_node: Vec<&String> = graph.iter()
            .filter(|(_, v)| { v.len() == 0 })
            .map(|(k, _)| { *k })
            .collect();

        potential_node.sort();
        let next_node = potential_node.first().unwrap();

        out.push_str(next_node);
        graph.remove(next_node);
        for values in graph.values_mut() {
            *values = values.iter()
                .filter(|&x| { x != next_node })
                .map(|x| { *x })
                .collect();
        }
    }

    String::from(out)
}

pub fn part2(input: &str) -> String {
    String::from("0")
}

#[cfg(test)]
mod tests {
    #[test]
    fn day7_part1 () {
        assert_eq!(super::part1("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."), "CABDFE");
    }

    #[test]
    fn day7_part2 () {
        assert_eq!(super::part2("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."), "15");
    }
}
