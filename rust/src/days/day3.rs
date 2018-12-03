use regex::Regex;
use std::collections::HashMap;

// Helper
fn parse_input(input: &str) -> Vec<(i32, i32, i32, i32, i32)> {
    let regexp = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    regexp.captures_iter(input)
        .map(|cap| {
            let id = (&cap[1]).parse().unwrap();
            let x = (&cap[2]).parse().unwrap();
            let y = (&cap[3]).parse().unwrap();
            let width = (&cap[4]).parse().unwrap();
            let height = (&cap[5]).parse().unwrap();
            (id, x, y, width, height)
        })
    .collect()
}

// Part1
pub fn part1(input: &str) -> String {
    // Parse input
    let squares = parse_input(input);

    // Generate complete position list
    let inches_taken = squares.iter()
        .flat_map(|(_, x, y, width, height)| {
            let mut out: Vec<_> = vec![];
            for i in *x..(x+width) {
                for j in *y..(y+height) {
                    out.push(format!("{}_{}", i, j));
                }
            }
            out
        });

    // Compute position frequency
    let mut freq: HashMap<String, u32> = HashMap::new();
    for coord in inches_taken {
        let entry = freq.entry(coord).or_insert(0);
        *entry += 1;
    }

    // Compute position with no overlay
    let number: u32 = freq.values()
        .filter(|&i| { *i > 1 })
        .map(|_| { 1 })
        .sum();

    format!("{}", number)
}

// Part2
fn overlap((_, x1_a, y1_a, w_a, h_a): (i32, i32, i32, i32, i32), (_, x1_b, y1_b, w_b, h_b): (i32, i32, i32, i32, i32)) -> bool {
    let x2_a = x1_a + w_a - 1;
    let y2_a = y1_a + h_a - 1;
    let x2_b = x1_b + w_b - 1;
    let y2_b = y1_b + h_b - 1;

    // A totally on the left of B
    if x1_a < x1_b && x2_a < x1_b { return false }

    // A totally on the right of B
    if x1_a > x2_b && x2_a > x2_b { return false }

    // A totally above B
    if y1_a < y1_b && y2_a < y1_b { return false }

    // A totally below B
    if y1_a > y2_b && y2_a > y2_b { return false }

    // Contact zone
    true
}
pub fn part2(input: &str) -> String {
    // Parse input
    let squares = parse_input(input);

    // Check every pair of square to check for overlap
    for i in 0..squares.len() {
        let (id, _, _, _, _) = squares[i];
        let mut do_not_overlap = true;
        for j in 0..squares.len() {
            let square1 = squares[i];
            let square2 = squares[j];
            if i != j && overlap(square1, square2) {
                do_not_overlap = false;
            }
        }

        // If no overlap at all, that's the one!
        if do_not_overlap {
            return format!("{}", id)
        }
    }

    return String::from("");
}

#[cfg(test)]
mod tests {
    #[test]
    fn day3_part1 () {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        assert_eq!(super::part1(input), "4");
    }

    #[test]
    fn day3_part2 () {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        assert_eq!(super::part2(input), "3");
    }
}
