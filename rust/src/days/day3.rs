use regex::Regex;
use std::collections::HashMap;

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

pub fn part1(input: &str) -> String {
    // Parse i,put
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

pub fn part2(input: &str) -> String {
    return String::from("");
}

#[cfg(test)]
mod tests {
    #[test]
    fn day3_part1 () {
        assert_eq!(0, 0);
    }

    #[test]
    fn day3_part2 () {
        assert_eq!(0, 0);
    }
}
