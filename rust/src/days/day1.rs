use std::collections::HashSet;

fn read_input (input: &str) -> Vec<i32> {
    input.split(" ").map(|s: &str| { s.parse().unwrap() }).collect()
}

pub fn part1 (input: &str) -> String {
    let mut tot = 0;
    for i in read_input(input) {
        tot += i;
    }
    return tot.to_string();
}

pub fn part2 (input: &str) -> String {
    let mut tot = 0;
    let mut seen = HashSet::new();

    let list = read_input(input);
    seen.insert(tot);

    loop {
        for i in &list {
            tot += i;
            if seen.contains(&tot) {
                return tot.to_string()
            };

            seen.insert(tot);
        }
    }
}
