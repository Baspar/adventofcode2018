use regex::Regex;
use std::io::{stdin,Read};

fn apply_opcode(registers: &mut [usize; 6], opcode: &(&str, usize, usize, usize)) {
    match *opcode {
        ("addr", a, b, c) => registers[c] = registers[a] + registers[b],
        ("addi", a, b, c) => registers[c] = registers[a] + b,
        ("mulr", a, b, c) => registers[c] = registers[a] * registers[b],
        ("muli", a, b, c) => registers[c] = registers[a] * b,
        ("banr", a, b, c) => registers[c] = registers[a] & registers[b],
        ("bani", a, b, c) => registers[c] = registers[a] & b,
        ("borr", a, b, c) => registers[c] = registers[a] | registers[b],
        ("bori", a, b, c) => registers[c] = registers[a] | b,
        ("setr", a, _, c) => registers[c] = registers[a],
        ("seti", a, _, c) => registers[c] = a,
        ("gtir", a, b, c) => registers[c] = if            a >  registers[b] { 1 } else { 0 },
        ("gtri", a, b, c) => registers[c] = if registers[a] >  b            { 1 } else { 0 },
        ("gtrr", a, b, c) => registers[c] = if registers[a] >  registers[b] { 1 } else { 0 },
        ("eqir", a, b, c) => registers[c] = if            a == registers[b] { 1 } else { 0 },
        ("eqri", a, b, c) => registers[c] = if registers[a] == b            { 1 } else { 0 },
        ("eqrr", a, b, c) => registers[c] = if registers[a] == registers[b] { 1 } else { 0 },
        _ => {}
    };
}

fn parse_input (input: &str) -> (usize, Vec<(&str, usize, usize, usize)>) {
    let mut lines = input.lines();
    let ip_line = lines.next().unwrap();
    let ip = Regex::new(r"\d+").unwrap()
        .captures(ip_line).unwrap()
        .get(0).unwrap()
        .as_str().parse().unwrap();

    let instructions = lines
        .map(|line: &str| {
            let mut words = line.split_whitespace();

            let op_code = words.next().unwrap();
            let a: usize = words.next().unwrap().parse().unwrap();
            let b: usize = words.next().unwrap().parse().unwrap();
            let c: usize = words.next().unwrap().parse().unwrap();

            return (op_code, a, b, c);
        })
        .collect();

    return (ip, instructions)
}

fn display(instruction_number: &usize, registers: &[usize; 6], opcodes: &Vec<(&str, usize, usize, usize)>) {
    for (id, opcode) in opcodes.iter().enumerate() {
        if id == *instruction_number {
            print!("==> ");
        } else {
            if id < 10 { print!(" {}  ", id) } else { print!("{}  ", id) };
        }
        println!("{:?}", opcode);
    }
    println!("{:?}", registers);
}

pub fn part1 (input: &str) -> String {
    let mut registers = [0usize; 6];
    let (ip, instructions) = parse_input(input);
    let mut instruction_number = 0;
    while instruction_number < instructions.len() {
        apply_opcode(&mut registers, &instructions[instruction_number]);
        registers[ip] += 1;
        instruction_number = registers[ip];
    }
    return format!("{}", registers[0]);
}

pub fn part2 (input: &str) -> String {
    let r3: u64 = 10551330;
    let out: u64 = (1..r3+1)
        .filter(|x| r3 % x == 0)
        .sum();
    return format!("{}", out);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day19_part1 () {
        assert_eq!(super::part1("#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"), "7");
    }

    #[test]
    fn day19_part2 () {
        assert_eq!(0, 0);
    }
}
