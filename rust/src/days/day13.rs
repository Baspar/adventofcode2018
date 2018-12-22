use std::cmp::max;
use num::complex::Complex;

// Structures
#[derive(Clone)]
enum Cell {
    Empty,    //
    Straight, // | or -
    Flip,     // /
    NegFlip,  // \
    Cross     // +
}
#[derive(Debug, Clone)]
struct Cart {
    pos: Complex<i64>,
    velocity: Complex<i64>,
    is_active: bool,
    turn_count: u8
}

// Helpers
fn generate_grid(input: &str) -> (Vec<Vec<Cell>>, Vec<Cart>) {
    let mut grid: Vec<Vec<Cell>> = Vec::new();
    let mut carts: Vec<Cart> = Vec::new();

    let width = input.lines()
        .map(|l| { l.len() })
        .fold(0, |a, b| { max(a, b) });

    for (x, line) in input.lines().enumerate() {
        grid.push(vec![Cell::Empty; width]);
        for (y, cell) in line.chars().enumerate() {
            // Cell
            grid[x][y] = match cell {
                '|'|'-' => Cell::Straight,
                '<'|'>'|'^'|'v' => Cell::Straight,
                '+' => Cell::Cross,
                '/' => Cell::NegFlip,
                '\\' => Cell::Flip,
                _ => Cell::Empty
            };

            // Cart
            match cell {
                '>' => {
                    carts.push(Cart{
                        pos: Complex::new(x as i64, y as i64),
                        turn_count: 0,
                        is_active: true,
                        velocity: Complex::new(0, 1)
                    })
                },
                '<' => {
                    carts.push(Cart{
                        pos: Complex::new(x as i64, y as i64),
                        turn_count: 0,
                        is_active: true,
                        velocity: Complex::new(0, -1)
                    })
                },
                'v' => {
                    carts.push(Cart{
                        pos: Complex::new(x as i64, y as i64),
                        turn_count: 0,
                        is_active: true,
                        velocity: Complex::new(1, 0)
                    })
                },
                '^' => {
                    carts.push(Cart{
                        pos: Complex::new(x as i64, y as i64),
                        turn_count: 0,
                        is_active: true,
                        velocity: Complex::new(-1, 0)
                    })
                },
                _ => {}
            }
        }
    }

    (grid, carts)
}
fn move_cart(carts: &mut Vec<Cart>, grid: &Vec<Vec<Cell>>, index: usize) {
    let cart = carts.get_mut(index).unwrap();
    if !cart.is_active { return; }

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
}
fn sort_carts(carts: &mut Vec<Cart>) {
    carts.sort_by_key(|x| x.pos.re);
    carts.sort_by_key(|x| x.pos.im);
}
fn get_collision(carts: &Vec<Cart>) -> Option<Complex<i64>> {
    for i in 0..carts.len() {
        for j in i+1..carts.len() {
            if carts[i].pos == carts[j].pos && carts[i].is_active && carts[j].is_active {
                return Some(carts[i].pos);
            }
        }
    }
    None
}
fn has_a_collision(carts: &Vec<Cart>) -> bool {
    match get_collision(carts) {
        Some(_) => true,
        _ => false
    }
}
fn remove_collision_carts(carts: &mut Vec<Cart>) {
    for i in 0..carts.len() {
        for j in i+1..carts.len() {
            if carts[i].pos == carts[j].pos && carts[i].is_active && carts[j].is_active {
                carts[i].is_active = false;
                carts[j].is_active = false;
                return;
            }
        }
    }
}

// Part1
pub fn part1 (input: &str) -> String {
    let (grid, mut carts) = generate_grid(input);
    loop {
        sort_carts(&mut carts);
        for i in 0..carts.len() {
            move_cart(&mut carts, &grid, i);
            if has_a_collision(&carts) {
                let cart_pos = get_collision(&carts).unwrap();
                return format!("{},{}", cart_pos.im , cart_pos.re);
            }
        }
    }
}

// Part2
pub fn part2 (input: &str) -> String {
    let (grid, mut carts) = generate_grid(input);
    let mut remaining_carts = carts.len();
    loop {
        sort_carts(&mut carts);
        for i in 0..carts.len() {
            move_cart(&mut carts, &grid, i);
            if has_a_collision(&carts) {
                remove_collision_carts(&mut carts);
                remaining_carts -= 2;
            }
        }
        if remaining_carts == 1 { break; }
    }
    let last_cart_pos = carts.iter()
        .filter(|c| c.is_active)
        .next()
        .unwrap().pos;
    format!("{},{}", last_cart_pos.im, last_cart_pos.re)
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
        assert_eq!(super::part2("/>-<\\
|   |
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/"), "6,4");
    }
}
