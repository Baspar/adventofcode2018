use std::collections::HashSet;

// Helper
fn read_input (input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|s: &str| { s.parse().unwrap() })
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    let mut tot = 0;
    for i in read_input(input) {
        tot += i;
    }
    return tot.to_string();
}

// Part2
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

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day1_part1 () {
        assert_eq!(super::part1("+1\n+1\n+1"), "3");
        assert_eq!(super::part1("+1\n+1\n-2"), "0");
        assert_eq!(super::part1("-1\n-2\n-3"), "-6");
    }

    #[test]
    fn day1_part2 () {
        assert_eq!(super::part2("+1\n-1"), "0");
        assert_eq!(super::part2("+3\n+3\n+4\n-2\n-4"), "10");
        assert_eq!(super::part2("-6\n+3\n+8\n+5\n-6"), "5");
        assert_eq!(super::part2("+7\n+7\n-2\n-7\n-4"), "14");
    }
}
