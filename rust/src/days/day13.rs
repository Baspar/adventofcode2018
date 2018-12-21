use std::cmp::max;
use num::complex::Complex;
use std::collections::VecDeque;

// Structures
#[derive(Clone)]
enum Cell {
    Empty,    //
    Straight, // | or -
    Flip,     // /
    NegFlip,  // \
    Cross,    // +
    Unknown   // If there was a cart
}
#[derive(Debug)]
struct Cart {
    pos: Complex<i64>,
    velocity: Complex<i64>,
    turn_count: u8
}

// Helpers
fn generate_grid(input: &str) -> (Vec<Vec<Cell>>, VecDeque<Cart>) {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    let mut carts: VecDeque<Cart> = VecDeque::new();

    let width = input.lines()
        .map(|l| { l.len() })
        .fold(0, |a, b| { max(a, b) });

    for (x, line) in input.lines().enumerate() {
        grid.push(vec![Cell::Empty; width]);
        for (y, cell) in line.chars().enumerate() {
            grid[x][y] = Cell::Empty;
            match cell {
                // Cell
                '|'|'-' => { grid[x][y] = Cell::Straight; },
                '+' => { grid[x][y] = Cell::Cross; },
                '/' => { grid[x][y] = Cell::NegFlip; },
                '\\' => { grid[x][y] = Cell::Flip; },

                // Cart
                '>' => {
                    grid[x][y] = Cell::Unknown;
                    carts.push_back(Cart{
                        pos: Complex::new(x as i64, y as i64),
                        turn_count: 0,
                        velocity: Complex::new(0, 1)
                    })
                },
                '<' => {
                    grid[x][y] = Cell::Unknown;
                    carts.push_back(Cart{
                        pos: Complex::new(x as i64, y as i64),
                        turn_count: 0,
                        velocity: Complex::new(0, -1)
                    })
                },
                'v' => {
                    grid[x][y] = Cell::Unknown;
                    carts.push_back(Cart{
                        pos: Complex::new(x as i64, y as i64),
                        turn_count: 0,
                        velocity: Complex::new(1, 0)
                    })
                },
                '^' => {
                    grid[x][y] = Cell::Unknown;
                    carts.push_back(Cart{
                        pos: Complex::new(x as i64, y as i64),
                        turn_count: 0,
                        velocity: Complex::new(-1, 0)
                    })
                },
                _ => { grid[x][y] = Cell::Empty; }
            }
        }
    }

    (grid, carts)
}
fn move_one_cart(carts: &mut VecDeque<Cart>, grid: &Vec<Vec<Cell>>) {
    let mut cart = carts.pop_front().unwrap();
    cart.pos += cart.velocity;
    let x = cart.pos.re;
    let y = cart.pos.im;

    cart.velocity = match grid[x as usize][y as usize] {
        Cell::Straight => cart.velocity,
        Cell::Flip => cart.velocity.conj() * Complex::i(),
        Cell::NegFlip => - cart.velocity.conj() * Complex::i(),
        Cell::Cross => {
            cart.turn_count += 1;
            cart.turn_count %= 3;
            match cart.turn_count {
                0 => cart.velocity * -Complex::i(), // Right
                1 => cart.velocity * Complex::i(), // Left
                2 => cart.velocity, // Straight
                _ => cart.velocity
            }
        },
        _ => cart.velocity
    };

    carts.push_back(cart);
}
fn get_collision(carts: &VecDeque<Cart>) -> Option<(i64, i64)> {
    for i in 0..carts.len() {
        for j in i+1..carts.len() {
            if carts[i].pos == carts[j].pos {
                return Some((carts[i].pos.im, carts[i].pos.re));
            }
        }
    }
    None
}
fn has_a_collision(carts: &VecDeque<Cart>) -> bool {
    match get_collision(carts) {
        Some(_) => true,
        _ => false
    }
}

// Part1
pub fn part1 (input: &str) -> String {
    let (grid, mut carts) = generate_grid(input);
    while !has_a_collision(&carts) {
        move_one_cart(&mut carts, &grid);
    }
    let (x, y) = get_collision(&carts).unwrap();
    format!("{},{}", x, y)
}

// Part2
pub fn part2 (_input: &str) -> String {
    format!("")
}

#[cfg(test)]
mod tests {
    #[test]
    fn day13_part1 () {
        assert_eq!(super::part1("/->-\\
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   "), "7,3");
    }

    #[test]
    fn day13_part2 () {
        assert_eq!(0, 0);
    }
}
