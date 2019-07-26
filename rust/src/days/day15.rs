// Types
enum Cell {
    Wall,
    Empty,
    Goblin(usize),
    Elf(usize)
}
#[derive(Debug)]
struct Character {
    is_goblin: bool,
    is_alive: bool,
    x: usize,
    y: usize
}
type Grid = Vec<Vec<Cell>>;
struct World {
    goblins: Vec<Character> ,
    elfs: Vec<Character>,
    grid: Grid
}

// Helpers
fn print(g: &Grid) {
    for row in g {
        for cell in row {
            match cell {
                Cell::Wall => print!( "#"),
                Cell::Empty => print!( " "),
                Cell::Goblin(_) => print!( "G"),
                Cell::Elf(_) => print!( "E")
            }
        }
        println!("");
    }
}
fn parse_input(input: &str) -> World {
    let mut goblins: Vec<Character> = Vec::new();
    let mut elfs: Vec<Character> = Vec::new();
    let mut grid: Grid = Vec::new();

    let mut x = 0;
    for line in input.lines() {
        let mut y = 0;
        let mut row: Vec<Cell> = Vec::new();
        for c in line.chars() {
            match c {
                '#' => row.push(Cell::Wall),
                '.' => row.push(Cell::Empty),
                'G' => {
                    let goblin = Character{is_goblin: true, is_alive: true, x: x, y: y};
                    row.push(Cell::Goblin(goblins.len()));
                    goblins.push(goblin);
                },
                'E' => {
                    let elf = Character{is_goblin: false, is_alive: true, x: x, y: y};
                    row.push(Cell::Elf(elfs.len()));
                    elfs.push(elf);
                },
                _ => {}
            }
            y += 1;
        }
        grid.push(row);
        x += 1;
    }

    World{
        grid: grid,
        elfs: elfs,
        goblins: goblins
    }

}
fn sort_characters(world: &mut World) {
    world.goblins.sort_by_key(|g| g.y);
    world.goblins.sort_by_key(|g| g.x);
    world.elfs.sort_by_key(|g| g.y);
    world.elfs.sort_by_key(|g| g.x);
}

// Part1
pub fn part1 (input: &str) -> String {
    let mut world: World = parse_input(input);
    sort_characters(&mut world);
    print(&world.grid);
    println!("{:?}", world.elfs);
    format!("")
}

// Part2
pub fn part2 (_input: &str) -> String {
    format!("")
}

#[cfg(test)]
mod tests {
    #[test]
    fn day15_part1 () {
        assert_eq!(0, 0);
    }

    #[test]
    fn day15_part2 () {
        assert_eq!(0, 0);
    }
}
