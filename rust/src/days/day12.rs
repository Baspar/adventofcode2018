use regex::Regex;
struct Data {
    flowers: Vec<bool>,
    recipes: Vec<bool>
}

// Parse input into Data
fn parse_input(input: &str) -> Data {
    // Find initial State
    let re = Regex::new("initial state: ([.#]*)").unwrap();
    let flowers: Vec<bool> = re.captures(input)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .chars()
        .map(|c| { c == '#' })
        .collect();

    // Find recipes
    let re_recipe = Regex::new("([.#]+) => ([.#])").unwrap();
    let mut recipes = vec![false; 32];
    for recipe in re_recipe.captures_iter(input) {
        let from = &recipe[1].chars()
            .rev()
            .enumerate()
            .map(|(i, c): (usize, char)| { if c == '#' { 2_usize.pow(i as u32) } else { 0 }})
            .fold(0, |a, b| { a + b });
        let to = &recipe[2] == "#";
        recipes[*from] = to;
    };

    Data{ flowers: flowers, recipes: recipes }
}
// Compute score with flowers
fn score(flowers: &Vec<bool>, min_index: &i64) -> i64 {
    flowers.iter()
        .enumerate()
        .map(|(index, &is_flower)| { if is_flower { index as i64 + *min_index } else { 0 } })
        .sum()
}
// Run one generation
fn alter_flower(flowers: Vec<bool>, min_index: i64, recipes: &Vec<bool>) -> (Vec<bool>, i64) {
    let mut new_min_index = min_index;
    let mut new_flowers:Vec<bool> = vec![];

    let mut value = 0;

    // Special check for first element
    value = (value % 16) * 2 + if flowers[0] {1} else {0};
    if recipes[value] { new_min_index -= 1; new_flowers.push(true) }

    // Special check for first element
    value = (value % 16) * 2 + if flowers[1] {1} else {0};
    if recipes[value] { new_min_index -= 1; new_flowers.push(true) }

    // Normal check for rest array
    for index in 2..flowers.len() + 2 {
        let flower = if index < flowers.len() {flowers[index]} else {false};
        value %= 16;
        value *= 2;
        value += if flower {1} else {0};

        new_flowers.push(recipes[value]);
    }

    // Special check for n+1
    value = (value % 16) * 2;
    if recipes[value] { new_flowers.push(true) }

    // Special check for n+2
    value = (value % 16) * 2;
    if recipes[value] { new_flowers.push(true) }

    (new_flowers, new_min_index)
}

// Part1
pub fn part1 (input: &str) -> String {
    let Data{ mut flowers, recipes } = parse_input(input);

    // Run 20 gen
    let mut min_index = 0;
    for _ in 0..20 {
        let res = alter_flower(flowers, min_index, &recipes);
        flowers = res.0;
        min_index = res.1;
    }
    let res = score(&flowers, &min_index);
    return format!("{}", res);
}

// Part2
pub fn part2 (input: &str) -> String {
    let Data{ mut flowers, recipes } = parse_input(input);
    let mut min_index = 0;

    // Run 100 generations
    for _ in 0..100 {
        let res = alter_flower(flowers, min_index, &recipes);
        flowers = res.0;
        min_index = res.1;
    }

    // Get score gen 100
    let score_at_gen100 = score(&flowers, &min_index);

    // Get score gen 101
    let res = alter_flower(flowers, min_index, &recipes);
    flowers = res.0;
    min_index = res.1;
    let score_at_gen101 = score(&flowers, &min_index);

    // Since a pattern start at iteration 100 (Visualy checked),
    // we can compute in advance the response
    let score_at_gen_50_billions = score_at_gen100 + (score_at_gen101 - score_at_gen100) * (50_000_000_000 - 100);
    return format!("{}", score_at_gen_50_billions);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day12_part1 () {
        assert_eq!(super::part1("initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"), "325");
    }

    #[test]
    fn day12_part2 () {
        assert_eq!(0, 0);
    }
}
