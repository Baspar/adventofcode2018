use std::collections::HashSet;
use std::collections::HashMap;

// Type
#[derive(Debug)]
enum ClosestTarget {
    Tie(usize),
    Target(usize, i32)
}

// Helper
fn get_boundaries(targets: &Vec<(i32, i32)>) -> (i32, i32, i32, i32) {
    let (mut min_x, mut min_y) = targets[0];
    let (mut max_x, mut max_y) = targets[0];
    for &(x, y) in targets {
        if x < min_x { min_x = x }
        if y < min_y { min_y = y }
        if x > max_x { max_x = x }
        if y > max_y { max_y = y }
    }
    (min_x, min_y, max_x, max_y)
}
fn parse_input(input: &str) -> (Vec<(i32, i32)>, i32, i32) {
    let mut targets = input.lines()
        .map(|line| {
            let coord_raw: Vec<_> = line.trim().split(", ").collect();
            let x = coord_raw[0].parse().unwrap();
            let y = coord_raw[1].parse().unwrap();
            (x, y)
        }).collect();
    let (min_x, min_y, max_x, max_y) = get_boundaries(&targets);
    targets = targets
        .iter()
        .map(|(x, y)| { (x - min_x, y - min_y) })
        .collect();
    (targets, max_x - min_x, max_y - min_y)
}
fn distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}
fn get_min_distance(targets: &Vec<(i32, i32)>, coord: (i32, i32)) -> ClosestTarget {
    let mut min_distance = -1;
    let mut min_target = 0;
    let mut is_tie = false;
    for (target_id, &target_coord) in targets.iter().enumerate() {
        let distance = distance(coord, target_coord);
        if min_distance == -1 || distance < min_distance {
            min_distance = distance;
            min_target = target_id;
            is_tie = false;
        } else if distance == min_distance {
            is_tie = true;
        }
    }

    if is_tie {
        ClosestTarget::Tie(min_target)
    } else {
        ClosestTarget::Target(min_target, min_distance)
    }
}

// Part1
pub fn part1(input: &str) -> String {
    // Parse
    let (targets, width, height) = parse_input(input);

    // Generate infinite zones and zone size
    let mut infinite_targets: HashSet<usize> = HashSet::new();
    let mut size_zone_target: HashMap<usize, i32> = HashMap::new();
    for x in 0..width {
        for y in 0..height {
            // Get best distance
            let min_distance = get_min_distance(&targets, (x, y));
            match min_distance {
                // If Tie, no action
                ClosestTarget::Tie(_) => {},
                // If target:
                ClosestTarget::Target(target_id, _) => {
                    // If on the border, consider as a infinite zone
                    if x == 0 || y == 0 || x == width || y == height {
                        infinite_targets.insert(target_id);
                    }
                    // Increment count
                    *(size_zone_target.entry(target_id).or_insert(0)) +=1;
                }
            }
        }
    }

    let mut size_largest_zone = -1;
    for (target_id, size) in size_zone_target {
        if !infinite_targets.contains(&target_id) {
            if size_largest_zone == -1 || size > size_largest_zone {
                size_largest_zone = size;
            }
        }
    }

    format!("{}", size_largest_zone)
}

// Part2
pub fn part2(input: &str) -> String {
    // Parse
    let (targets, width, height) = parse_input(input);

    // Generate infinite zones and zone size
    const MAX_DIST: i32 = 10000;
    let mut valid_cell: i32 = 0;
    for x in 0..width {
        for y in 0..height {
            let sum_distances = targets.iter()
                .map(|&target_coord| { distance(target_coord, (x, y)) })
                .fold(0, |a, b| { a + b });
            if sum_distances < MAX_DIST {
                valid_cell += 1;
            }
        }
    }

    format!("{}", valid_cell)
}

#[cfg(test)]
mod tests {
    #[test]
    fn day6_part1 () {
        assert_eq!(super::part1("1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"), "17");
    }

    #[test]
    fn day6_part2 () {
        assert_eq!(super::part2("1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"), "16");
    }
}
