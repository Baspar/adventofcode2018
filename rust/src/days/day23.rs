use regex::Regex;

struct Bot {
    x: i32,
    y: i32,
    z: i32,
    radius: i32
}
impl Bot {
    fn new() -> Self {
        return Bot {
            x: 0,
            y: 0,
            z: 0,
            radius: 0
        };
    }
    fn manhattan_distance(&self, other: &Self) -> i32 {
        return
            (self.x - other.x).abs() +
            (self.y - other.y).abs() +
            (self.z - other.z).abs()
    }
}

fn parse_input(input: &str) -> Vec<Bot> {
    let mut out = Vec::new();
    let r = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)").unwrap();
    for line in input.lines() {
        let cap = r.captures(line).unwrap();
        let mut b = Bot::new();
        b.x = cap.get(1).unwrap().as_str().parse().unwrap();
        b.y = cap.get(2).unwrap().as_str().parse().unwrap();
        b.z = cap.get(3).unwrap().as_str().parse().unwrap();
        b.radius = cap.get(4).unwrap().as_str().parse().unwrap();
        out.push(b);
    }
    return out;
}

pub fn part1 (input: &str) -> String {
    let bots = parse_input(input);
    let max_bot = bots.iter()
        .max_by_key(|&b| b.radius)
        .unwrap();
    return format!("{}", bots
                   .iter()
                   .filter(|b| b.manhattan_distance(&max_bot) <= max_bot.radius)
                   .count());
}

pub fn part2 (input: &str) -> String {
    let _bots = parse_input(input);
    return format!("");
}

#[cfg(test)]
mod tests {
    #[test]
    fn day23_part1 () {
        assert_eq!(super::part1("pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
"), "7");
    }

    #[test]
    fn day23_part2 () {
        assert_eq!(0, 0);
    }
}
