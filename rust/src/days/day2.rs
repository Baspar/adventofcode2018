fn count_2_3_occurences (s: &str) -> (i32, i32) {
    let mut has_2_occurences = 0;
    let mut has_3_occurences = 0;
    let mut occurences = [0; 26];

    // Count occurences
    for i in s.chars() {
        let index = i as usize - 'a' as usize;
        occurences[index] += 1;
    }

    // Check frequencies of 2 or 3
    for occurence in occurences.iter() {
        match *occurence {
            2 => has_2_occurences = 1,
            3 => has_3_occurences = 1,
            _ => ()
        }
    }

    (has_2_occurences, has_3_occurences)
}
pub fn part1 (input: &str) -> String {
    let occurences: Vec<(i32, i32)> = input
        .lines()
        .map(|s: &str| { count_2_3_occurences(s) })
        .collect();

    let mut occurences_2 = 0;
    let mut occurences_3 = 0;

    for (occ_2, occ_3) in occurences {
        occurences_2 += occ_2;
        occurences_3 += occ_3;
    }

    let result = occurences_2 * occurences_3;

    format!("{}", result)
}

pub fn part2 (input: &str) -> String {
    return String::from(input);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day2_part1 () {
        assert_eq!(0, 0);
    }

    #[test]
    fn day2_part2 () {
        assert_eq!(0, 0);
    }
}
