use std::env;
use std::process;
use regex::Regex;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check given day
    if args.len() <= 1 {
        println!("Please provide a day");
        process::exit(1)
    }

    // Check if given day is valid
    let day: &String = &args[1];
    let day_regex = Regex::new(r"^day([1-9]|1[0-9]|2[0-5])$").unwrap();
    if !day_regex.is_match(day) {
        println!("Please provide a valid day");
        process::exit(1)
    }

    // Use Args as input for day
    let input = if args.len() > 2 {
        &args[2]
    } else {
        ""
    };

    println!("Part1:");
    let result_part1 = days::part1(day, input);
    println!("{}", result_part1);

    println!("Part2:");
    let result_part2 = days::part2(day, input);
    println!("{}", result_part2);
}
