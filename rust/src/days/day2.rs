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

// Part 2
fn distance_of_1(w1: &str, w2: &str) -> bool {
    let mut distance = 0;
    let mut _w1 = w1.chars();
    let mut _w2 = w2.chars();

    loop {
        match (_w1.next(), _w2.next()) {
            (Some(l1), Some(l2)) => { if l1 != l2 { distance += 1; } },
            _ => { break; }
        }
    }

    distance == 1
}
fn common_letters(w1: &str, w2: &str) -> String {
    let mut _w2 = w2.chars();
    w1.chars()
        .filter(|c| { *c == _w2.next().unwrap() })
        .collect()
}
pub fn part2 (input: &str) -> String {
    for w1 in input.lines() {
        for w2 in input.lines() {
            if distance_of_1(w1, w2) {
                return common_letters(&w1, &w2);
            }
        }
    };

    String::from("None found")
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day2_part1 () {
        let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        assert_eq!(super::part1(input), "12");
    }

    #[test]
    fn day2_part2 () {
        let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        assert_eq!(super::part2(input), "fgij");
    }
}
