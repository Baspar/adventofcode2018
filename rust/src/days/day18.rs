use std::cmp::{min,max};
use std::collections::HashMap;

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
enum Cell {
    Ground,
    Tree,
    Lumber
}
type Map = Vec<Vec<Cell>>;

fn parse_input (input: &str) -> Map {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for cell in line.chars() {
            match cell {
                '#' => row.push(Cell::Lumber),
                '.' => row.push(Cell::Ground),
                '|' => row.push(Cell::Tree),
                _ => {}
            }
        }
        map.push(row);
    }
    return map;
}
fn get_neighbour(map: &Map, c_y: usize, c_x: usize) -> (usize, usize) {
    let mut nb_tree = 0;
    let mut nb_lumber = 0;
    for y in (max(c_y, 1) - 1)..min(c_y + 2, map.len()) {
        for x in (max(c_x, 1) - 1)..min(c_x + 2, map[y].len()) {
            if x != c_x || y != c_y {
                match map[y][x] {
                    Cell::Tree => nb_tree += 1,
                    Cell::Lumber => nb_lumber += 1,
                    _ => {}
                }
            }
        }
    }

    return ( nb_tree, nb_lumber )
}
fn iteration(map: &mut Map) {
    let old_map = map.clone();

    for y in 0..old_map.len() {
        for x in 0..old_map[y].len() {
            let ( nb_tree, nb_lumber ) = get_neighbour(&old_map, x, y);
            match old_map[x][y] {
                Cell::Ground => if nb_tree >= 3 { map[x][y] = Cell::Tree },
                Cell::Tree => if nb_lumber >= 3 { map[x][y] = Cell::Lumber },
                Cell::Lumber =>  if nb_lumber == 0 || nb_tree == 0 { map[x][y] = Cell::Ground },
            }
        }
    }
}
fn count_all(map: &Map) -> (usize, usize) {
    let mut nb_tree = 0;
    let mut nb_lumber = 0;
    for row in map {
        for cell in row {
            match cell {
                Cell::Tree => nb_tree += 1,
                Cell::Lumber => nb_lumber += 1,
                _ => {}
            }
        }
    }
    return ( nb_tree, nb_lumber )
}
// fn display(map: &Map) {
//     for row in map {
//         for cell in row {
//             match cell {
//                 Cell::Ground => print!("."),
//                 Cell::Tree => print!("|"),
//                 Cell::Lumber => print!("#")
//             }
//         }
//         println!("")
//     }
// }

pub fn part1 (input: &str) -> String {
    let mut map = parse_input(input);
    for _ in 0..10 {
        // println!("\n===== Day {} =====\n", i);
        // display(&map);
        iteration(&mut map);
    }
    let ( nb_tree, nb_lumber ) = count_all(&map);
    return format!("{}", nb_lumber * nb_tree);
}

pub fn part2 (input: &str) -> String {
    let mut map = parse_input(input);
    let mut i = 0;
    let mut first_seen = HashMap::new();
    let end = 1_000_000_000;
    while i < end {
        if i < end/2 && first_seen.contains_key(&map) {
            let previous_time = first_seen[&map];
            let loop_size = i - previous_time;
            while i + loop_size < end { i += loop_size }
        } else {
            first_seen.insert(map.clone(), i);
            iteration(&mut map);
            i += 1;
        }
    }

    let ( nb_tree, nb_lumber ) = count_all(&map);
    return format!("{}", nb_lumber * nb_tree);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day18_part1 () {
        assert_eq!(super::part1(".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."), "1147");
    }

    #[test]
    fn day18_part2 () {
        assert_eq!(0, 0);
    }
}
