use regex::Regex;
use std::cmp::{max, min};
use std::collections::LinkedList;

#[derive(Clone, Copy)]
enum Cell {
    StillWater,
    FlowingWater,
    Wall,
    Empty
}

fn parse_input (input: &str) -> (Vec<Vec<Cell>>, usize, usize) {
    let regexp = Regex::new(r"([xy])=(\d+), ([xy])=(\d+)\.\.(\d+)").unwrap();
    enum Line {
        X(usize, usize, usize),
        Y(usize, usize, usize)
    }

    // Read and generate lines
    let mut lines = Vec::new();
    for line in input.lines() {
        let cap = regexp.captures(line).unwrap();
        let dimension = cap.get(1).unwrap().as_str();
        if dimension == "x" {
            let x = cap.get(2).unwrap().as_str().parse().unwrap();
            let y1 = cap.get(4).unwrap().as_str().parse().unwrap();
            let y2 = cap.get(5).unwrap().as_str().parse().unwrap();
            lines.push(Line::X(x, y1, y2));
        } else {
            let y = cap.get(2).unwrap().as_str().parse().unwrap();
            let x1 = cap.get(4).unwrap().as_str().parse().unwrap();
            let x2 = cap.get(5).unwrap().as_str().parse().unwrap();
            lines.push(Line::Y(y, x1, x2));
        }
    }

    // Find min/max x/y
    let (min_x, min_y, max_x, max_y) = lines
        .iter()
        .map(|line| match line {
            Line::X(x, y1, y2) => (x, min(y1, y2), x, max(y1, y2)),
            Line::Y(y, x1, x2) => (min(x1, x2), y, max(x1, x2), y)
        })
        .fold((500, 999999, 500, 0), |
              (min_x1, min_y1, max_x1, max_y1), (&min_x2, &min_y2, &max_x2, &max_y2)
              | (
                  min(min_x1, min_x2),
                  min(min_y1, min_y2),
                  max(max_x1, max_x2),
                  max(max_y1, max_y2)
              ));

    let delta_x = (max_x + 1) - (min_x - 1);
    let delta_y = max_y;

    // Initialize map
    let mut map = Vec::new();
    for _ in 0..(delta_y + 1) {
        map.push(vec![Cell::Empty; delta_x + 1]);
    }

    // Mark line on the map
    for line in lines {
        match line {
            Line::X(x, y1, y2) => {
                for y in y1..(y2+1) {
                    map[y][x - (min_x - 1)] = Cell::Wall;
                }
            },
            Line::Y(y, x1, x2) => {
                for x in x1..(x2+1) {
                    map[y][x - (min_x - 1)] = Cell::Wall;
                }
            }
        }
    }

    return (map, min_x, min_y);
}
fn flow(map: &Vec<Vec<Cell>>, y: usize, x: usize) -> (usize, usize) {
    let mut min_x = x;
    let mut max_x = x;

    while min_x > 0 {
        // If cell not accessible, stop
        match map[y][min_x - 1] {
            Cell::Empty => {},
            Cell::FlowingWater => {},
            _ => break
        }

        // Move
        min_x -= 1;

        // Check if Cell below is empty
        match map[y + 1][min_x] {
            Cell::Empty | Cell::FlowingWater => break,
            _ => {}
        }
    }

    while max_x < map[y].len() - 1 {
        // If cell not accessible, stop
        match map[y][max_x + 1] {
            Cell::Empty => {},
            Cell::FlowingWater => {},
            _ => break
        }

        // Move
        max_x += 1;

        // Check if Cell below is empty
        match map[y + 1][max_x] {
            Cell::Empty | Cell::FlowingWater => break,
            _ => {}
        }
    }

    return (min_x, max_x);
}
fn iteration(map: &mut Vec<Vec<Cell>>, sources: &mut LinkedList<(usize, usize)>) {
    let (source_y, source_x) = sources.pop_front().unwrap();

    if source_y < map.len() - 1 {
        match map[source_y + 1][source_x] {
            Cell::Empty => {
                map[source_y + 1][source_x] = Cell::FlowingWater;
                sources.push_back((source_y + 1, source_x));
            },
            Cell::FlowingWater => return,
            _ => {
                let (left_x, right_x) = flow(&map, source_y, source_x);
                let left_type = map[source_y + 1][left_x].clone();
                let right_type = map[source_y + 1][right_x].clone();
                for x in left_x..(right_x + 1) {
                    map[source_y][x] = match (left_type, right_type) {
                        (Cell::Empty, _)
                        | (_, Cell::Empty)
                        | (_, Cell::FlowingWater)
                        | (Cell::FlowingWater, _) => Cell::FlowingWater,
                        _ => Cell::StillWater
                    }
                }

                match (left_type, right_type) {
                    (Cell::Empty, Cell::Empty) => {
                        sources.push_back((source_y, left_x));
                        sources.push_back((source_y, right_x));
                    },
                    (Cell::Empty, _) => sources.push_back((source_y, left_x)),
                    (_, Cell::Empty) => sources.push_back((source_y, right_x)),
                    (_, Cell::FlowingWater) => {},
                    (Cell::FlowingWater, _) => {},
                    (_, _) => if source_y != map.len() - 1 { sources.push_back((source_y - 1, source_x)) }
                }

            }
        }
    }
}
fn display(map: &Vec<Vec<Cell>>) {
    for row in map.iter() {
        for cell in row.iter() {
            match cell {
                Cell::Empty => print!(" "),
                Cell::Wall => print!("█"),
                Cell::FlowingWater => print!("░"),
                Cell::StillWater => print!("▒")
            }
        }
        println!("");
    }
}

pub fn part1 (input: &str) -> String {
    let (mut map, min_x, min_y) = parse_input(input);
    let mut sources = LinkedList::new();
    sources.push_back((0, 500 - (min_x - 1)));
    map[0][500 - (min_x - 1)] = Cell::FlowingWater;
    while !sources.is_empty() {
        iteration(&mut map, &mut sources);
    }

    // display(&map);

    let mut water_amount = 0;
    for y in min_y..map.len() {
        for cell in map[y].iter() {
            match cell {
                Cell::FlowingWater | Cell::StillWater => water_amount += 1,
                _ => {}
            }
        }
    }

    return format!("{}", water_amount);
}

pub fn part2 (input: &str) -> String {
    let (mut map, min_x, min_y) = parse_input(input);
    let mut sources = LinkedList::new();
    sources.push_back((0, 500 - (min_x - 1)));
    map[0][500 - (min_x - 1)] = Cell::FlowingWater;
    while !sources.is_empty() {
        iteration(&mut map, &mut sources);
    }

    // display(&map);

    let mut water_amount = 0;
    for y in min_y..map.len() {
        for cell in map[y].iter() {
            match cell {
                Cell::StillWater => water_amount += 1,
                _ => {}
            }
        }
    }

    return format!("{}", water_amount);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day17_part1 () {
        assert_eq!(super::part1("x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"), "57");
    }

    #[test]
    fn day17_part2 () {
        assert_eq!(super::part2("x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"), "29");
    }
}
