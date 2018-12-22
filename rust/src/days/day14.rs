// Helpers
fn step(elf1: &mut usize, elf2: &mut usize, recipes: &mut Vec<usize>) -> u8{
    let mult = recipes[*elf1] + recipes[*elf2];

    if mult > 9 {
        recipes.push(mult / 10);
    }
    recipes.push(mult % 10);

    *elf1 = (*elf1 + recipes[*elf1] + 1) % recipes.len();
    *elf2 = (*elf2 + recipes[*elf2] + 1) % recipes.len();

    return if mult > 9 { 2 } else { 1 };
}
fn substr(recipes: &Vec<usize>, i: usize, j: usize) -> String {
    recipes[i..j].iter().fold(String::new(), |a, b| format!("{}{}", a, b))
}

// Part1
pub fn part1 (input: &str) -> String {
    let n: usize = input.trim().parse().unwrap();
    let mut recipes = vec![3, 7];
    let mut elf1=0;
    let mut elf2=1;

    while (recipes.len() as usize) < n + 10 {
        step(&mut elf1, &mut elf2, &mut recipes);
    }
    substr(&recipes, n, n+10)
}

// Part2
pub fn part2 (input: &str) -> String {
    let mut recipes = vec![3, 7];
    let mut elf1=0;
    let mut elf2=1;
    let n: usize = input.trim().parse().unwrap();
    let pow: usize = 10usize.pow(input.trim().len() as u32 - 1);

    let mut sub: Option<usize> = None;
    let mut i = input.len() - 1;
    loop {
        let nb_new_recipes = step(&mut elf1, &mut elf2, &mut recipes);
        for _ in 0..nb_new_recipes {
            if recipes.len() >= input.len() {
                sub = match sub {
                    Some(x) => Some(x % pow * 10 + recipes[i]),
                    None => Some(recipes[..input.len()].iter().fold(0, |a, b| 10*a+b))
                };

                i += 1;
                if sub.unwrap() == n { return format!("{}", i - input.len() + 1); }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn day14_part1 () {
        assert_eq!(super::part1("9"), "5158916779");
        assert_eq!(super::part1("5"), "0124515891");
        assert_eq!(super::part1("18"), "9251071085");
        assert_eq!(super::part1("2018"), "5941429882");
    }

    #[test]
    fn day14_part2 () {
        assert_eq!(super::part2("51589"), "9");
        assert_eq!(super::part2("01245"), "5");
        assert_eq!(super::part2("92510"), "18");
        assert_eq!(super::part2("59414"), "2018");
    }
}
