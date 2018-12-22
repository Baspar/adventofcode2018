// Part1
pub fn part1 (input: &str) -> String {
    let n: usize = input.trim().parse().unwrap();
    let mut out = vec![3, 7];
    let mut i=0;
    let mut j=1;
    while (out.len() as usize) < n + 10 {
        let int_i = out[i];
        let int_j = out[j];
        let mult = int_i + int_j;

        for val in format!("{}", mult).chars().map(|c| c as usize - '0' as usize) {
            out.push(val);
        }

        i = (i + out[i] + 1) % out.len();
        j = (j + out[j] + 1) % out.len();
    }
    let mut res = out.split_off(n);
    res.truncate(10);
    res.iter().fold(String::new(), |a, b| format!("{}{}", a, b))
}

// Part2
pub fn part2 (input: &str) -> String {
    return String::from(input);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day14_part1 () {
        assert_eq!(super::part1("9"), "5158916779");
        assert_eq!(super::part1("5"), "0124515891");
        assert_eq!(super::part1("18"), "9251071085");
        assert_eq!(super::part1("2018"), "5941429882");
    }

    #[test]
    fn day14_part2 () {
        assert_eq!(0, 0);
    }
}
