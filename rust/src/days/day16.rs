use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Training {
    before_registers: [u32; 4],
    after_registers: [u32; 4],
    op: [u32; 4]
}

// Helper
fn parse_input (input: &str) -> Vec<Training> {
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
    // let mut set = vec![];

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

    return training_set;
}

fn is_valid_opcode(opcode: &str, a: u32, b: u32, c:u32, before_registers: [u32; 4], after_registers: [u32; 4]) -> bool {
    return match opcode {
        "addr" => after_registers[c as usize] == before_registers[a as usize] + before_registers[b as usize],
        "addi" => after_registers[c as usize] == before_registers[a as usize] + b,
        "mulr" => after_registers[c as usize] == before_registers[a as usize] * before_registers[b as usize],
        "muli" => after_registers[c as usize] == before_registers[a as usize] * b,
        "banr" => after_registers[c as usize] == before_registers[a as usize] & before_registers[b as usize],
        "bani" => after_registers[c as usize] == before_registers[a as usize] & b,
        "borr" => after_registers[c as usize] == before_registers[a as usize] | before_registers[b as usize],
        "bori" => after_registers[c as usize] == before_registers[a as usize] | b,
        "setr" => after_registers[c as usize] == before_registers[a as usize],
        "seti" => after_registers[c as usize] == a,
        "gtir" => false,
        "gtri" => false,
        "gtrr" => false,
        "eqir" => false,
        "eqri" => false,
        "eqrr" => false,
        _      => false
    };
}
pub fn part1 (input: &str) -> String {
    let training_set = parse_input(input);
    // println!("{:?}", training_set);
    for op in vec!["addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"] {
        println!("{} => {}", op, is_valid_opcode(op, 2, 1, 2, [3, 2, 1, 1], [3, 2, 2, 1]))
    }
    return String::from("");
}

pub fn part2 (input: &str) -> String {
    return String::from("");
}

#[cfg(test)]
mod tests {
    #[test]
    fn day16_part1 () {
        assert_eq!(0, 0);
    }

    #[test]
    fn day16_part2 () {
        assert_eq!(0, 0);
    }
}
