use regex::Regex;

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

pub fn part1 (input: &str) -> String {
    let mut registers = [0usize; 6];
    let (ip, instructions) = parse_input(input);
    let mut instruction_number = 0;
    while instruction_number != 28 {
        apply_opcode(&mut registers, &instructions[instruction_number]);
        registers[ip] += 1;
        instruction_number = registers[ip];
    }
    return format!("{}", registers[5]);
}

pub fn part2 (input: &str) -> String {
    return String::from(input);
}

// #ip 2
// [r0, r1, op, r2, r3, r4]
//
//  [0]: r4 = 123
//  [1]: r4 = r4 & 456
//  [2]: if r4 == 72 { r4 = 1 } else { r4 = 0 }
//  [3]: op += r4
//  [4]: GOTO [1]
//
//  [5]: r4 = 0
//  [6]: r2 = r4 | 65536
//  [7]: r4 = 7586220
//  [8]: r1 = r2 & 255
//  [9]: r4 += r1
// [10]: r4 = r4 & 16777215
// [11]: r4 = r4 * 65899
// [12]: r4 = r4 & 16777215
//
// [13]: if r2 < 256 { r1 = 1 } else { r1 = 0 }
// [14]: op += r1
// [15]: GOTO [18]
// [16]: GOTO [28]
//
// [17]: r1 = 0
// [18]: r3 = r3 + 1
// [19]: r3 = r3 * 256
//
// [20]: if r3 > r2 { r3 = 1 } else { r3 = 0 }
// [21]: op += r3
// [22]: GOTO [25]
// [23]: GOTO [26]
//
// [24]: r1 = r1 + 1
// [25]: GOTO [18]
//
// [26]: r2 = r1
// [27]: GOTO [8]
//
// [28]: if r4 == r0 { r1 = 1 } else { r1 = 0 }
// [29]: GOTO [31]
// [30]: GOTO [6]

#[cfg(test)]
mod tests {
    #[test]
    fn day21_part1 () {
        assert_eq!(0, 0);
    }

    #[test]
    fn day21_part2 () {
        assert_eq!(0, 0);
    }
}
