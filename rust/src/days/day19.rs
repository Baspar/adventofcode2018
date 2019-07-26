use regex::Regex;

enum Instruction {
    Addr(usize, usize, usize),
    Addi(usize, usize, usize),
    Mulr(usize, usize, usize),
    Muli(usize, usize, usize),
    Banr(usize, usize, usize),
    Bani(usize, usize, usize),
    Borr(usize, usize, usize),
    Bori(usize, usize, usize),
    Setr(usize, usize, usize),
    Seti(usize, usize, usize),
    Gtir(usize, usize, usize),
    Gtri(usize, usize, usize),
    Gtrr(usize, usize, usize),
    Eqir(usize, usize, usize),
    Eqri(usize, usize, usize),
    Eqrr(usize, usize, usize),
    Error()
}


fn apply_opcode(registers: &mut [usize; 6], opcode: &Instruction) {
    match *opcode {
        Instruction::Addr(a, b, c) => registers[c] = registers[a] + registers[b],
        Instruction::Addi(a, b, c) => registers[c] = registers[a] + b,
        Instruction::Mulr(a, b, c) => registers[c] = registers[a] * registers[b],
        Instruction::Muli(a, b, c) => registers[c] = registers[a] * b,
        Instruction::Banr(a, b, c) => registers[c] = registers[a] & registers[b],
        Instruction::Bani(a, b, c) => registers[c] = registers[a] & b,
        Instruction::Borr(a, b, c) => registers[c] = registers[a] | registers[b],
        Instruction::Bori(a, b, c) => registers[c] = registers[a] | b,
        Instruction::Setr(a, _, c) => registers[c] = registers[a],
        Instruction::Seti(a, _, c) => registers[c] = a,
        Instruction::Gtir(a, b, c) => registers[c] = if            a >  registers[b] { 1 } else { 0 },
        Instruction::Gtri(a, b, c) => registers[c] = if registers[a] >  b            { 1 } else { 0 },
        Instruction::Gtrr(a, b, c) => registers[c] = if registers[a] >  registers[b] { 1 } else { 0 },
        Instruction::Eqir(a, b, c) => registers[c] = if            a == registers[b] { 1 } else { 0 },
        Instruction::Eqri(a, b, c) => registers[c] = if registers[a] == b            { 1 } else { 0 },
        Instruction::Eqrr(a, b, c) => registers[c] = if registers[a] == registers[b] { 1 } else { 0 },
        Instruction::Error() => {}
    };
}

fn parse_input (input: &str) -> (usize, Vec<Instruction>) {
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

            return match op_code {
                "addr" => Instruction::Addr(a, b, c),
                "addi" => Instruction::Addi(a, b, c),
                "mulr" => Instruction::Mulr(a, b, c),
                "muli" => Instruction::Muli(a, b, c),
                "banr" => Instruction::Banr(a, b, c),
                "bani" => Instruction::Bani(a, b, c),
                "borr" => Instruction::Borr(a, b, c),
                "bori" => Instruction::Bori(a, b, c),
                "setr" => Instruction::Setr(a, b, c),
                "seti" => Instruction::Seti(a, b, c),
                "gtir" => Instruction::Gtir(a, b, c),
                "gtri" => Instruction::Gtri(a, b, c),
                "gtrr" => Instruction::Gtrr(a, b, c),
                "eqir" => Instruction::Eqir(a, b, c),
                "eqri" => Instruction::Eqri(a, b, c),
                "eqrr" => Instruction::Eqrr(a, b, c),
                _      => Instruction::Error()
            }
        })
        .collect();

    return (ip, instructions)
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
    return String::from(input);
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
