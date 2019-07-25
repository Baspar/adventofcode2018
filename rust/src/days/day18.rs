use std::cmp::{min,max};
use std::collections::HashMap;

#[derive(Debug,Clone)]
enum Cell {
    Ground,
    Tree,
    Lumber
}

fn parse_input (input: &str) -> Vec<Vec<Cell>> {
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
fn get_neighbour(map: &Vec<Vec<Cell>>, c_y: usize, c_x: usize) -> (usize, usize) {
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
fn iteration(map: &mut Vec<Vec<Cell>>) {
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
fn display(map: &Vec<Vec<Cell>>) {
    for row in map {
        for cell in row {
            match cell {
                Cell::Ground => print!("."),
                Cell::Tree => print!("|"),
                Cell::Lumber => print!("#")
            }
        }
        println!("")
    }
}
fn count_all(map: &Vec<Vec<Cell>>) -> (usize, usize) {
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
fn hash(map: &Vec<Vec<Cell>>) -> String {
    let mut out = String::new();
    for row in map {
        for cell in row {
            out.push(match cell {
                Cell::Tree => '|',
                Cell::Lumber => '#',
                Cell::Ground => '.'
            })
        }
    }

    return out;
}

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
    let end = 1000000000;
    while i < end {
        let h = hash(&map);
        if i < end/2 && first_seen.contains_key(&h) {
            let previous_time = first_seen[&h];
            let loop_size = i - previous_time;
            while i + loop_size < end { i += loop_size }
        } else {
            first_seen.insert(h, i);
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
