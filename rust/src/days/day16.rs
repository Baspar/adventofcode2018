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
fn number_valid_opcode(a: u32, b: u32, c: u32, before_registers: [u32; 4], after_registers: [u32; 4]) -> usize {
    vec!["addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"]
        .iter()
        .filter(|opcode| is_valid_opcode(opcode, a, b, c, before_registers, after_registers))
        .map(|_| 1)
        .fold(0, |a, b| a + b)

}
pub fn part1 (input: &str) -> String {
    let training_set = parse_input(input);
    let number_valid_opcode: Vec<usize> = training_set
        .iter()
        .map(|record| {
            let [_, a, b, c] = record.op;
            number_valid_opcode(a, b, c, record.before_registers, record.after_registers)
        })
        .filter(|number| *number >= 3)
        .collect();
    return format!("{}", number_valid_opcode.len());
}

pub fn part2 (_input: &str) -> String {
    return String::from("");
}

#[cfg(test)]
mod tests {
    #[test]
    fn day16_part1 () {
        assert_eq!(super::part1("Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]"), "1");
        assert_eq!(super::part1("Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 1, 1]"), "1");
    }

    #[test]
    fn day16_part2 () {
        assert_eq!(0, 0);
    }
}
