use std::ops::BitXor;

use advent_of_code::get_challenge_input_as_str;

#[derive(Debug)]
struct ComboOperand(u64);

impl ComboOperand {
    pub fn get_value(&self, computer: &Computer) -> u64 {
        match self.0 {
            0 | 1 | 2 | 3 => self.0,
            4 => computer.register_a,
            5 => computer.register_b,
            6 => computer.register_c,
            7 => panic!("7 does not appear in valid programs"),
            _ => panic!("Invalid operand"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    ADV(ComboOperand),
    BXL(u64),
    BST(ComboOperand),
    JNZ(u64),
    BXC(()),
    OUT(ComboOperand),
    BDV(ComboOperand),
    CDV(ComboOperand),
}

#[derive(Debug, Clone)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    pub instruction_ptr: u64,

    program: Vec<u64>,
}

impl Computer {
    pub fn execute_instruction(&mut self) -> Option<u64> {
        use Instruction::*;

        let operand = self.program[(self.instruction_ptr + 1) as usize];

        let instruction = match self.program[self.instruction_ptr as usize] {
            0 => ADV(ComboOperand(operand)),
            1 => BXL(operand),
            2 => BST(ComboOperand(operand)),
            3 => JNZ(operand),
            4 => BXC(()),
            5 => OUT(ComboOperand(operand)),
            6 => BDV(ComboOperand(operand)),
            7 => CDV(ComboOperand(operand)),
            _ => panic!("Unknown instruction"),
        };

        match instruction {
            ADV(operand) => {
                let numerator = self.register_a;
                let denominator = 2_u64.pow(operand.get_value(self) as u32);

                self.register_a = numerator / denominator;
                self.instruction_ptr += 2;
            }
            BXL(operand) => {
                self.register_b = self.register_b.bitxor(operand);
                self.instruction_ptr += 2;
            }
            BST(operand) => {
                self.register_b = operand.get_value(self) % 8;
                self.instruction_ptr += 2;
            }
            JNZ(operand) => {
                if self.register_a != 0 {
                    self.instruction_ptr = operand;
                } else {
                    self.instruction_ptr += 2;
                }
            }
            BXC(_) => {
                self.register_b = self.register_b.bitxor(self.register_c);
                self.instruction_ptr += 2;
            }
            OUT(operand) => {
                let output = operand.get_value(self) % 8;
                self.instruction_ptr += 2;
                return Some(output);
            }
            BDV(operand) => {
                let numerator = self.register_a;
                let denominator = 2_u64.pow(operand.get_value(self) as u32);

                self.register_b = numerator / denominator;
                self.instruction_ptr += 2;
            }
            CDV(operand) => {
                let numerator = self.register_a;
                let denominator = 2_u64.pow(operand.get_value(self) as u32);

                self.register_c = numerator / denominator;
                self.instruction_ptr += 2;
            }
        }

        None
    }
}

fn parse_data(data: &str) -> Computer {
    let mut register_a: u64 = 0;
    let mut register_b: u64 = 0;
    let mut register_c: u64 = 0;
    let mut program: Vec<u64> = Vec::new();

    for line in data.lines() {
        if line.starts_with("Register A: ") {
            register_a = line
                .split_at("Register A: ".len())
                .1
                .parse::<u64>()
                .unwrap();
        } else if line.starts_with("Register B: ") {
            register_b = line
                .split_at("Register B: ".len())
                .1
                .parse::<u64>()
                .unwrap();
        } else if line.starts_with("Register C: ") {
            register_c = line
                .split_at("Register C: ".len())
                .1
                .parse::<u64>()
                .unwrap();
        } else if line.starts_with("Program: ") {
            program = line
                .split_at("Program: ".len())
                .1
                .split(',')
                .map(|c| c.parse::<u64>().unwrap())
                .collect();
        }
    }

    Computer {
        register_a,
        register_b,
        register_c,
        program,
        instruction_ptr: 0,
    }
}

#[allow(unused)]
fn part_one(computer: &mut Computer) -> String {
    let mut all_output: Vec<u64> = Vec::new();

    while computer.instruction_ptr < computer.program.len() as u64 {
        if let Some(output) = computer.execute_instruction() {
            all_output.push(output);
        }
    }

    all_output
        .iter()
        .map(|o| format!("{o}"))
        .collect::<Vec<String>>()
        .join(",")
}

#[allow(unused)]
fn part_two(original_computer: &mut Computer) -> u64 {
    let mut register_a_value: u64 = 0;
    let mut new_output: Vec<u64> = Vec::new();
    let mut computer = original_computer.clone();

    loop {
        computer = original_computer.clone();
        computer.register_a = register_a_value;
        new_output.clear();

        while computer.instruction_ptr < computer.program.len() as u64 {
            if let Some(output) = computer.execute_instruction() {
                new_output.push(output);
            }
        }

        if new_output != computer.program {
            register_a_value += 1;

            println!("{:?}", new_output);
            std::thread::sleep(std::time::Duration::from_millis(250));

            // if register_a_value % 1000 == 0 {
            //     println!("Trying {register_a_value}...")
            // }
        } else {
            break;
        }
    }

    println!("{:?} {:?}", new_output, computer.program);

    register_a_value
}

pub fn main() {
    if let Ok(data) = get_challenge_input_as_str(2024, 17) {
        let mut computer = parse_data(&data);

        let output = part_one(&mut computer);

        println!("part one: {output}");

        let mut computer = parse_data(&data);

        let output = part_two(&mut computer);

        println!("part two: {output}");
    }
}

mod tests {
    #![allow(unused)]
    use super::*;

    const EXAMPLE_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE_TWO_INPUT: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_parse() {
        let data = parse_data(EXAMPLE_INPUT);

        assert_eq!(data.register_a, 729);
        assert_eq!(data.register_b, 0);
        assert_eq!(data.register_c, 0);
        assert_eq!(data.program.len(), 6);
    }

    #[test]
    fn test_part_one() {
        let mut data = parse_data(EXAMPLE_INPUT);
        let output = part_one(&mut data);

        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_two() {
        let mut data = parse_data(EXAMPLE_TWO_INPUT);
        let output = part_two(&mut data);

        assert_eq!(output, 117440);
    }
}
