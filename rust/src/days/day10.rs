use regex::Regex;

fn display(points: &Vec<(i64, i64, i64, i64)>) {
    // Find boundaries
    let (mut x_min, mut y_min, _, _) = points.first().unwrap();
    let (mut x_max, mut y_max, _, _) = points.first().unwrap();
    for &(x, y, _, _) in points {
        if x < x_min { x_min = x }
        if x > x_max { x_max = x }
        if y < y_min { y_min = y }
        if y > y_max { y_max = y }
    }

    let width = x_max - x_min + 1;
    let height = y_max - y_min + 1;
    let mut grid = vec![vec![false; width as usize]; height as usize];

    for (x, y) in points.iter().map(|(x, y, _, _)| { (x - x_min, y - y_min) }) {
        grid[y as usize][x as usize] = true;
    }

    for line in grid {
        for cell in line {
            print!("{}", if cell { "#" } else { "." });
        }
        println!("");
    }
    println!("");

    ()
}
fn looks_valid(points: &Vec<(i64, i64, i64, i64)>) -> bool {
    for i in 0..points.len() {
        let (x1, y1, _, _) = points[i];
        let close_match = points.iter()
            .map(|(x2, y2, _, _)| {
                (x1-x2).abs() + (y1-y2).abs() == 1 || (x1-x2)*(x1-x2) + (y1-y2)*(y1-y2) == 2
            })
            .fold(false, |a, b| { a || b });
        if !close_match {
            return false;
        }
    }
    true
}
fn move_points(points: Vec<(i64, i64, i64, i64)>) -> Vec<(i64, i64, i64, i64)> {
    points.iter()
        .map(|(x, y, dx, dy)| {(x + dx, y + dy, *dx, *dy)})
        .collect()
}

pub fn part1 (input: &str) -> String {
    let re = Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d+), *(-?\d+)>")
        .unwrap();
    let mut points: Vec<(i64, i64, i64, i64)> = re.captures_iter(input)
        .map(|cap| {
            let x: i64 = cap[1].parse().unwrap();
            let y: i64 = cap[2].parse().unwrap();
            let dx: i64 = cap[3].parse().unwrap();
            let dy: i64 = cap[4].parse().unwrap();
            (x, y, dx, dy)
        }).collect();

    let mut iteration = 0;
    while !looks_valid(&points){
        iteration += 1;
        points = move_points(points);
        // if iteration == 10 {
        //     break;
        // }
    }

    display(&points);

    println!("{}", iteration);
    format!("")
}

pub fn part2 (input: &str) -> String {
    format!("")
}

#[cfg(test)]
mod tests {
    #[test]
    fn day10_part1 () {
        assert_eq!(super::part1("position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>"), "0");
    }

    #[test]
    fn day10_part2 () {
        assert_eq!(0, 0);
    }
}
