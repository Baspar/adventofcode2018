pub fn part1 (input: &str) -> String {
    let mut collapsed_polymer: Vec<char> = Vec::new();
    for i in input.trim().chars() {
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
    format!("{:?}", collapsed_polymer.len())
}

pub fn part2 (input: &str) -> String {
    return String::from(input);
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
