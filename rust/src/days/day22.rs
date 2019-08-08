use regex::Regex;

type Cell = (usize, usize);

fn parse_input (input: &str) -> (usize, usize, usize) {
    let mut lines = input.lines();
    let l1 = lines.next().unwrap();
    let l2 = lines.next().unwrap();

    let depth: usize = Regex::new(r"\d+").unwrap()
        .captures(l1).unwrap()
        .get(0).unwrap()
        .as_str().parse().unwrap();
    let width: usize = Regex::new(r"(\d+),(\d+)").unwrap()
        .captures(l2).unwrap()
        .get(1).unwrap()
        .as_str().parse().unwrap();
    let height: usize = Regex::new(r"(\d+),(\d+)").unwrap()
        .captures(l2).unwrap()
        .get(2).unwrap()
        .as_str().parse().unwrap();

    return (depth, width, height)
}

fn display (map: &Vec<Vec<Cell>>) {
    for line in map {
        for (_, cell) in line {
            print!("{}", match cell {
                0 => '.',
                1 => '=',
                _ => '|'
            })
        }
        println!("");
    }
}

fn risk_level (map: &Vec<Vec<Cell>>) -> usize {
    let mut level = 0;
    for line in map {
        for (_, cell) in line {
            level += cell;
        }
    }

    let (_, target_type) = map.last().unwrap().last().unwrap();

    return level - target_type;
}

pub fn part1 (input: &str) -> String {
    let (depth, width, height) = parse_input(input);
    let mut map: Vec<Vec<Cell>> = Vec::new();
    for x in 0..width+1 {
        map.push(Vec::new());
        for y in 0..height+1 {
            let geo_index = match (x, y) {
                (0, 0) => 0,
                (0, _) => y * 48271,
                (_, 0) => x * 16807,
                _ => {
                    let (g1, _) = map[x][y-1];
                    let (g2, _) = map[x-1][y];
                    g1 * g2
                }
            };

            let erosion_level = (geo_index + depth) % 20183;
            let cell_type = erosion_level % 3;

            map[x].push((erosion_level, cell_type));
        }
    }
    display(&map);
    return format!("{}", risk_level(&map));
}

pub fn part2 (input: &str) -> String {
    return String::from(input);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day22_part1 () {
        assert_eq!(super::part1("depth: 510
                                target: 10,10"), "114");
    }

    #[test]
    fn day22_part2 () {
        assert_eq!(0, 0);
    }
}
