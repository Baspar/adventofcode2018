use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap,HashMap};

type Cell = (usize, usize);
type Map = Vec<Vec<Cell>>;
#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
    is_torch_equiped: bool,
    is_gear_equiped: bool,
    distance: usize
}

impl Position {
    fn new() -> Position {
        return Position {
            x: 0,
            y: 0,
            is_torch_equiped: true,
            is_gear_equiped: false,
            distance: 0
        }
    }
    fn key(&self) -> String {
        return format!("{}_{}_{}_{}", self.x, self.y, self.is_torch_equiped, self.is_gear_equiped);
    }
    fn is_valid(&self, map: &Map) -> bool {
        match map[self.y][self.x] {
            (_, 0) => if !self.is_torch_equiped && !self.is_gear_equiped { return false },
            (_, 1) => if self.is_torch_equiped { return false },
            (_, 2) => if self.is_gear_equiped { return false },
            _ => {}
        }
        return true
    }
    fn equip_none(&self, map: &Map) -> Option<Position> {
        if !self.is_torch_equiped && !self.is_gear_equiped {
            return None;
        }

        let mut clone = self.clone();
        clone.is_torch_equiped = false;
        clone.is_gear_equiped = false;
        clone.distance += 7;

        if clone.is_valid(map) {
            return Some(clone)
        }

        return None;
    }
    fn equip_torch(&self, map: &Map) -> Option<Position> {
        if self.is_torch_equiped {
            return None;
        }

        let mut clone = self.clone();
        clone.is_torch_equiped = true;
        clone.is_gear_equiped = false;
        clone.distance += 7;

        if clone.is_valid(map) {
            return Some(clone);
        }

        return None;
    }
    fn equip_gear(&self, map: &Map) -> Option<Position> {
        if self.is_gear_equiped {
            return None;
        }

        let mut clone = self.clone();
        clone.is_torch_equiped = false;
        clone.is_gear_equiped = true;
        clone.distance += 7;

        if clone.is_valid(map) {
            return Some(clone);
        }

        return None;
    }
    fn move_right (&self, map: &Map) -> Option<Position> {
        let mut clone = self.clone();
        clone.distance += 1;
        clone.x += 1;

        if clone.is_valid(map) {
            return Some(clone)
        }

        return None;
    }
    fn move_left (&self, map: &Map) -> Option<Position> {
        if self.x == 0 { return None }

        let mut clone = self.clone();
        clone.distance += 1;
        clone.x -= 1;

        if clone.is_valid(map) {
            return Some(clone)
        }

        return None;
    }
    fn move_up (&self, map: &Map) -> Option<Position> {
        if self.y == 0 { return None }

        let mut clone = self.clone();
        clone.distance += 1;
        clone.y -= 1;

        if clone.is_valid(map) {
            return Some(clone)
        }

        return None;
    }
    fn move_down (&self, map: &Map) -> Option<Position> {
        let mut clone = self.clone();
        clone.distance += 1;
        clone.y += 1;

        if clone.is_valid(map) {
            return Some(clone)
        }

        return None;
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}
impl Eq for Position { }
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

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

// fn display (map: &Map) {
//     for line in map {
//         for (_, cell) in line {
//             print!("{}", match cell {
//                 0 => '.',
//                 1 => '=',
//                 _ => '|'
//             })
//         }
//         println!("");
//     }
// }

fn risk_level (map: &Map) -> usize {
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
    let mut map: Map = Vec::new();
    for x in 0..=width {
        map.push(Vec::new());
        for y in 0..=height {
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
    // display(&map);
    return format!("{}", risk_level(&map));
}

pub fn part2 (input: &str) -> String {
    let (depth, width, height) = parse_input(input);
    let mut map: Map = Vec::new();
    let mut q = BinaryHeap::new();
    let mut seen = HashMap::new();

    for y in 0..=width*height {
        map.push(Vec::new());
        for x in 0..=width*height {
            let geo_index = if x == width && y == height {
                0
            } else {
                match (x, y) {
                (0, 0) => 0,
                (0, _) => y * 48271,
                (_, 0) => x * 16807,
                _ => {
                    let (g1, _) = map[y-1][x];
                    let (g2, _) = map[y][x-1];
                    g1 * g2
                }}
            };

            let erosion_level = (geo_index + depth) % 20183;
            let cell_type = erosion_level % 3;

            map[y].push((erosion_level, cell_type));
        }
    }

    // display(&map);

    q.push(Position::new());
    while let Some(pos) = q.pop() {
        let key = pos.key();
        if seen.get(&key).is_some() {
            continue
        }

        seen.insert(key, pos.distance);

        if pos.x == width && pos.y == height && pos.is_torch_equiped {
            return format!("{}", pos.distance)
        }

        if let Some(p) = pos.equip_none(&map) { q.push(p); }
        if let Some(p) = pos.equip_torch(&map) { q.push(p); }
        if let Some(p) = pos.equip_gear(&map) {  q.push(p); }
        if let Some(p) = pos.move_up(&map) { q.push(p); }
        if let Some(p) = pos.move_left(&map) { q.push(p); }
        if let Some(p) = pos.move_down(&map) { q.push(p); }
        if let Some(p) = pos.move_right(&map) { q.push(p); }
    }

    return format!("");
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
        assert_eq!(super::part2("depth: 510
                                target: 10,10"), "45");
    }
}
