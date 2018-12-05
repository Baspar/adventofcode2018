// Helper
fn collapse_len(characters: &Vec<char>) -> usize {
    let mut collapsed_polymer: Vec<char> = Vec::new();
    for &i in characters {
        // Push first letter onto the stack
        if collapsed_polymer.is_empty() {
            collapsed_polymer.push(i);
        } else {
            // Get last value
            let l = collapsed_polymer.last().unwrap();
            // If different case, pop, else push
            if *l != i && l.to_ascii_uppercase() == i.to_ascii_uppercase() {
                collapsed_polymer.pop();
            } else {
                collapsed_polymer.push(i);
            }
        }
    }
    collapsed_polymer.len()
}

// Part1
pub fn part1(input: &str) -> String {
    let characters = input.trim().chars().collect();
    format!("{}", collapse_len(&characters))
}

// Part2
fn collapse_len_but(exclude_letter: &char, input: &str) -> usize {
    let filtered_input = input.trim().chars().filter(|c| { c.to_ascii_uppercase() != *exclude_letter }).collect();
    collapse_len(&filtered_input)
}

pub fn part2(input: &str) -> String {
    let mut collapsed_polymers: Vec<usize> = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        .chars()
        .map(|letter| { collapse_len_but(&letter, input) })
        .collect();

    // Sort and get smallest polymer
    collapsed_polymers.sort();
    format!("{}", collapsed_polymers.first().unwrap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn day5_part1 () {
        assert_eq!(super::part1("aA"), "0");
        assert_eq!(super::part1("abBA"), "0");
        assert_eq!(super::part1("abAB"), "4");
        assert_eq!(super::part1("aabAAB"), "6");
        assert_eq!(super::part1("dabAcCaCBAcCcaDA"), "10");
    }

    #[test]
    fn day5_part2 () {
        assert_eq!(super::part2("dabAcCaCBAcCcaDA"), "4");
    }
}
