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
fn init_grid(input: &str) -> Vec<Vec<i64>> {
    let serial: i64 = input.trim().parse().unwrap();
    let mut grid = vec![vec![0; 300]; 300];
    for i in 0..300 {
        for j in 0..300 {
            grid[i][j] = compute_power(serial, i as i64, j as i64);
        }
    };

    grid
}
fn init_partial_sum_grid(input: &str) -> Vec<Vec<i64>> {
    let serial: i64 = input.trim().parse().unwrap();
    let mut grid = vec![vec![0; 300]; 300];

    for i in 0..300 {
        for j in 0..300 {
            grid[i][j] = compute_power(serial, i as i64, j as i64);
            grid[i][j] += if i == 0 && j == 0 {
                0
            } else if i == 0 {
                grid[i][j-1]
            } else if j == 0 {
                grid[i-1][j]
            } else {
                grid[i-1][j] + grid[i][j-1] - grid[i-1][j-1]
            }
        }
    }

    grid
}

// Part1
pub fn part1 (input: &str) -> String {
    let grid = init_grid(input);

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
    let grid = init_partial_sum_grid(input);

    let mut max_size = 1;
    let mut max_x = 1;
    let mut max_y = 1;
    let mut max_power = grid[0][0];

    for size in 1..301 {
        for x in 0..300-size+1 {
            for y in 0..300-size+1 {
                let power = if x == 0 && y == 0 {
                    grid[x+size-1][y+size-1]
                } else if x == 0 {
                    grid[x+size-1][y+size-1] - grid[x+size-1][y-1]
                } else if y == 0 {
                    grid[x+size-1][y+size-1] - grid[x-1][y+size-1]
                } else {
                    grid[x+size-1][y+size-1] - grid[x-1][y+size-1] - grid[x+size-1][y-1] + grid[x-1][y-1]
                };

                if power > max_power {
                    max_size = size;
                    max_x = x+1;
                    max_y = y+1;
                    max_power = power;
                }
            }
        }
    }

    format!("{},{},{}", max_x, max_y, max_size)
}

#[cfg(test)]
mod tests {
    #[test]
    fn day11_part1 () {
        assert_eq!(super::part1("42"), "21,61");
    }

    #[test]
    fn day11_part2 () {
        assert_eq!(super::part2("18"), "90,269,16");
        assert_eq!(super::part2("42"), "232,251,12");
    }
}
