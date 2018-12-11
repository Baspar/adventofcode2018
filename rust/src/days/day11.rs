// Helper
fn compute_power(serial: i64, x: i64, y: i64) -> i64 {
    let rack_id = (x+1) + 10;
    let mut power = rack_id * (y+1);
    power = power + serial;
    power = power * rack_id;
    power = power / 100;
    power = power % 10;
    power = power - 5;
    power
}

// Part1
pub fn part1 (input: &str) -> String {
    let serial: i64 = input.trim().parse().unwrap();
    let mut grid = vec![vec![0; 300]; 300];
    for i in 0..300 {
        for j in 0..300 {
            grid[i][j] = compute_power(serial, i as i64, j as i64);
        }
    }

    let mut max_power = grid[0][0];
    let mut max_coord = (0, 0);
    for x in 0..300-3 {
        for y in 0..300-3 {
            let mut power = 0;
            for dx in 0..3 {
                for dy in 0..3 {
                    power += grid[x+dx][y+dy];
                    if power > max_power {
                        max_power = power;
                        max_coord = (x+1, y+1);
                    }
                }
            }
        }
    }

    let (x, y) = max_coord;
    format!("{},{}", x, y)
}

// Part2
pub fn part2 (input: &str) -> String {
    return String::from(input);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day11_part1 () {
        assert_eq!(super::part1("42"), "21,61");
    }

    #[test]
    fn day11_part2 () {
        assert_eq!(0, 0);
    }
}
