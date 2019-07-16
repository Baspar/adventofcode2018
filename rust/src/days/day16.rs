use regex::Regex;
use std::collections::{HashMap,HashSet};

#[derive(Debug, Copy, Clone)]
struct Training {
    before_registers: [u32; 4],
    after_registers: [u32; 4],
    op: [u32; 4]
}

const OP_CODES: [&str; 16] = ["addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"];

// Helper
fn parse_input (input: &str) -> (Vec<Training>, Vec<[u32; 4]>) {
    let before_regexp = Regex::new(r"Before: *\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    let op_regexp = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    let after_regexp = Regex::new(r"After: *\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();

    let mut ongoing_training = Training {
        before_registers: [0; 4],
        after_registers: [0; 4],
        op: [0; 4]
    };
    let mut has_ongoing_training = false;
    let mut training_set: Vec<Training> = vec![];
    let mut operations: Vec<[u32; 4]> = vec![];

    for line in input.lines() {
        let matches = [
            before_regexp.captures(line),
            op_regexp.captures(line),
            after_regexp.captures(line)
        ];
        match matches {
            [Some(registers), _, _] => {
                has_ongoing_training = true;
                ongoing_training.before_registers = [
                    registers.get(1).unwrap().as_str().parse().unwrap(),
                    registers.get(2).unwrap().as_str().parse().unwrap(),
                    registers.get(3).unwrap().as_str().parse().unwrap(),
                    registers.get(4).unwrap().as_str().parse().unwrap()
                ];
            }
            [_, Some(op), _] => {
                if has_ongoing_training {
                    ongoing_training.op = [
                        op.get(1).unwrap().as_str().parse().unwrap(),
                        op.get(2).unwrap().as_str().parse().unwrap(),
                        op.get(3).unwrap().as_str().parse().unwrap(),
                        op.get(4).unwrap().as_str().parse().unwrap()
                    ];
                } else {
                    operations.push([
                        op.get(1).unwrap().as_str().parse().unwrap(),
                        op.get(2).unwrap().as_str().parse().unwrap(),
                        op.get(3).unwrap().as_str().parse().unwrap(),
                        op.get(4).unwrap().as_str().parse().unwrap()
                    ])
                }
            },
            [_, _, Some(registers)] => {
                has_ongoing_training = false;
                ongoing_training.after_registers = [
                    registers.get(1).unwrap().as_str().parse().unwrap(),
                    registers.get(2).unwrap().as_str().parse().unwrap(),
                    registers.get(3).unwrap().as_str().parse().unwrap(),
                    registers.get(4).unwrap().as_str().parse().unwrap()
                ];
                training_set.push(ongoing_training);
            },
            _ => {}
        }
    }

    return (training_set, operations);
}

fn is_valid_opcode(opcode: &str, a_: u32, b_: u32, c_: u32, before_registers: [u32; 4], after_registers: [u32; 4]) -> bool {
    let a = a_ as usize;
    let b = b_ as usize;
    let c = c_ as usize;
    return match opcode {
        "addr" => after_registers[c] == before_registers[a] + before_registers[b],
        "addi" => after_registers[c] == before_registers[a] + b_,
        "mulr" => after_registers[c] == before_registers[a] * before_registers[b],
        "muli" => after_registers[c] == before_registers[a] * b_,
        "banr" => after_registers[c] == before_registers[a] & before_registers[b],
        "bani" => after_registers[c] == before_registers[a] & b_,
        "borr" => after_registers[c] == before_registers[a] | before_registers[b],
        "bori" => after_registers[c] == before_registers[a] | b_,
        "setr" => after_registers[c] == before_registers[a],
        "seti" => after_registers[c] == a_,
        "gtir" => after_registers[c] == if                  a_ >  before_registers[b] { 1 } else { 0 },
        "gtri" => after_registers[c] == if before_registers[a] >  b_                  { 1 } else { 0 },
        "gtrr" => after_registers[c] == if before_registers[a] >  before_registers[b] { 1 } else { 0 },
        "eqir" => after_registers[c] == if                  a_ == before_registers[b] { 1 } else { 0 },
        "eqri" => after_registers[c] == if before_registers[a] == b_                  { 1 } else { 0 },
        "eqrr" => after_registers[c] == if before_registers[a] == before_registers[b] { 1 } else { 0 },
        _      => false
    };
}
fn get_valid_opcodes(a: u32, b: u32, c: u32, before_registers: [u32; 4], after_registers: [u32; 4]) -> HashSet<String> {
    OP_CODES
        .clone()
        .iter()
        .filter(|opcode| is_valid_opcode(opcode, a, b, c, before_registers, after_registers))
        .map(|&x| String::from(x))
        .collect()
}
pub fn part1 (input: &str) -> String {
    let (training_set, _) = parse_input(input);
    let number_valid_opcode: Vec<HashSet<String>> = training_set
        .iter()
        .map(|record| {
            let [_, a, b, c] = record.op;
            get_valid_opcodes(a, b, c, record.before_registers, record.after_registers)
        })
        .filter(|valid_opcode| valid_opcode.len() >= 3)
        .collect();
    return format!("{}", number_valid_opcode.len());
}

fn apply_opcode(opcode: &str, a_: u32, b_: u32, c_: u32, registers: &mut [u32; 4]) {
    let a = a_ as usize;
    let b = b_ as usize;
    let c = c_ as usize;
    match opcode {
        "addr" => registers[c] = registers[a] + registers[b],
        "addi" => registers[c] = registers[a] + b_,
        "mulr" => registers[c] = registers[a] * registers[b],
        "muli" => registers[c] = registers[a] * b_,
        "banr" => registers[c] = registers[a] & registers[b],
        "bani" => registers[c] = registers[a] & b_,
        "borr" => registers[c] = registers[a] | registers[b],
        "bori" => registers[c] = registers[a] | b_,
        "setr" => registers[c] = registers[a],
        "seti" => registers[c] = a_,
        "gtir" => registers[c] = if           a_ >  registers[b] { 1 } else { 0 },
        "gtri" => registers[c] = if registers[a] >  b_           { 1 } else { 0 },
        "gtrr" => registers[c] = if registers[a] >  registers[b] { 1 } else { 0 },
        "eqir" => registers[c] = if           a_ == registers[b] { 1 } else { 0 },
        "eqri" => registers[c] = if registers[a] == b_           { 1 } else { 0 },
        "eqrr" => registers[c] = if registers[a] == registers[b] { 1 } else { 0 },
        _      => {}
    };
}
fn deduce_opcodes(training_set: Vec<Training>) -> HashMap<u32, String> {
    let mut possible_opcodes: HashMap<u32, HashSet<String>> = HashMap::new();
    for training_data in training_set {
        let [opcode, a, b, c] = training_data.op;
        let before_registers = training_data.before_registers;
        let after_registers = training_data.after_registers;
        let valid_opcodes = get_valid_opcodes(a, b, c, before_registers, after_registers);

        possible_opcodes.insert(
            opcode,
            if !possible_opcodes.contains_key(&opcode) {
                valid_opcodes
            } else {
                possible_opcodes[&opcode]
                .intersection(&valid_opcodes)
                .cloned()
                .collect()
            }
        );
    }

    let mut out: HashMap<u32, String> = HashMap::new();
    for _ in 0..OP_CODES.len() {

        let mut values_to_remove = Vec::new();
        let mut opcodes_to_remove = Vec::new();

        for (opcode, values) in possible_opcodes.iter() {
            if values.len() == 1 {
                let value = values.iter().next().unwrap();
                out.insert(opcode.clone(), value.clone());
                opcodes_to_remove.push(opcode.clone());
                values_to_remove.push(value.clone());
                break;
            }
        }

        for opcode in opcodes_to_remove.iter() {
            possible_opcodes.remove(opcode);
        }

        for values in possible_opcodes.values_mut() {
            for value in values_to_remove.iter() {
                values.remove(value);
            }
        }
    }

    return out;
}

pub fn part2 (input: &str) -> String {
    let (training_set, operations) = parse_input(input);
    let opcode_mapping = deduce_opcodes(training_set);
    let mut register = [0u32; 4];
    for [opcode, a, b, c] in operations {
        apply_opcode(opcode_mapping.get(&opcode).unwrap(), a, b, c, &mut register);
    }

    return format!("{}", register[0]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn day16_part1 () {
        assert_eq!(super::part1("Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]"), "1");
        assert_eq!(super::part1("Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [7, 6, 5, 4]"), "0");
    }

    #[test]
    fn day16_part2 () {
        assert_eq!(0, 0);
    }
}
