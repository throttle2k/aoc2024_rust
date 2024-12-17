use core::panic;
use std::{collections::HashMap, fmt::Write};

use common::read_input;

#[derive(Debug)]
enum Operand {
    Literal(usize),
    Combo(usize),
}

impl Operand {
    fn value(&self, registers: &HashMap<char, usize>) -> usize {
        match self {
            Operand::Literal(n) => *n,
            Operand::Combo(n) => match n {
                n if *n <= 3 => *n,
                4 => *registers.get(&'a').unwrap(),
                5 => *registers.get(&'b').unwrap(),
                6 => *registers.get(&'c').unwrap(),
                n => unreachable!("Unknown combo operand {n}"),
            },
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Adv(Operand),
    Bxl(Operand),
    Bst(Operand),
    Jnz(Operand),
    Bxc(Operand),
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand),
}

impl From<&[&str]> for Instruction {
    fn from(value: &[&str]) -> Self {
        let (opcode, operand) = (
            value[0].parse::<usize>().unwrap(),
            value[1].parse::<usize>().unwrap(),
        );
        match opcode {
            0 => Self::Adv(Operand::Combo(operand)),
            1 => Self::Bxl(Operand::Literal(operand)),
            2 => Self::Bst(Operand::Combo(operand)),
            3 => Self::Jnz(Operand::Literal(operand)),
            4 => Self::Bxc(Operand::Literal(operand)),
            5 => Self::Out(Operand::Combo(operand)),
            6 => Self::Bdv(Operand::Combo(operand)),
            7 => Self::Cdv(Operand::Combo(operand)),
            n => panic!("Unknown opcode {n}"),
        }
    }
}

impl Instruction {
    fn apply<W: Write>(
        &self,
        pointer: usize,
        registers: &mut HashMap<char, usize>,
        output: &mut W,
    ) -> usize {
        match self {
            Instruction::Adv(operand) => {
                let numerator = registers.get(&'a').unwrap().clone();
                let denominator = 2_usize.pow(operand.value(registers) as u32);
                registers
                    .entry('a')
                    .and_modify(|value| *value = numerator / denominator);
                pointer + 1
            }
            Instruction::Bxl(operand) => {
                let operand = operand.value(registers).clone();
                registers.entry('b').and_modify(|value| *value ^= operand);
                pointer + 1
            }
            Instruction::Bst(operand) => {
                let operand = operand.value(registers).clone();
                registers
                    .entry('b')
                    .and_modify(|value| *value = operand % 8);
                pointer + 1
            }
            Instruction::Jnz(operand) => {
                if *registers.get(&'a').unwrap() == 0 {
                    pointer + 1
                } else {
                    let operand = operand.value(registers).clone();
                    operand / 2
                }
            }
            Instruction::Bxc(_operand) => {
                let reg_b = registers.get(&'b').unwrap().clone();
                let reg_c = registers.get(&'c').unwrap().clone();
                registers
                    .entry('b')
                    .and_modify(|value| *value = reg_b ^ reg_c);
                pointer + 1
            }
            Instruction::Out(operand) => {
                let operand = operand.value(registers).clone();
                write!(output, "{},", operand % 8).unwrap();
                pointer + 1
            }
            Instruction::Bdv(operand) => {
                let numerator = registers.get(&'a').unwrap().clone();
                let denominator = 2_usize.pow(operand.value(registers) as u32);
                registers
                    .entry('b')
                    .and_modify(|value| *value = numerator / denominator);
                pointer + 1
            }
            Instruction::Cdv(operand) => {
                let numerator = registers.get(&'a').unwrap().clone();
                let denominator = 2_usize.pow(operand.value(registers) as u32);
                registers
                    .entry('c')
                    .and_modify(|value| *value = numerator / denominator);
                pointer + 1
            }
        }
    }
}

#[derive(Debug)]
struct Computer {
    registers: HashMap<char, usize>,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
    output: String,
}

fn parse_register(input: (&str, &str)) -> (char, usize) {
    let (_, reg) = input.0.trim().split_once(' ').unwrap();
    let value = input.1.trim().parse::<usize>().unwrap();
    (reg.to_lowercase().chars().nth(0).unwrap(), value)
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let mut lines = value.trim().lines();
        let (reg_a, value_a) = parse_register(lines.next().unwrap().split_once(":").unwrap());
        let (reg_b, value_b) = parse_register(lines.next().unwrap().split_once(":").unwrap());
        let (reg_c, value_c) = parse_register(lines.next().unwrap().split_once(":").unwrap());
        lines.next();
        let (_, program) = lines.next().unwrap().split_once(" ").unwrap();
        let instructions = program
            .trim()
            .split(',')
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|i| i.into())
            .collect();
        let mut registers = HashMap::new();
        registers.insert(reg_a, value_a);
        registers.insert(reg_b, value_b);
        registers.insert(reg_c, value_c);
        Self {
            registers,
            instructions,
            instruction_pointer: 0,
            output: String::new(),
        }
    }
}

impl Computer {
    fn execute_program(&mut self) {
        while let Some(instruction) = self.instructions.get(self.instruction_pointer) {
            self.instruction_pointer = instruction.apply(
                self.instruction_pointer,
                &mut self.registers,
                &mut self.output,
            );
        }
        if self.output.ends_with(',') {
            self.output = self.output.strip_suffix(',').unwrap().to_string();
        }
    }
}

fn main() {
    let input = read_input("day17.txt");
    let mut computer = Computer::from(input.as_str());
    computer.execute_program();
    println!("Part 1 = {}", computer.output);
}

#[cfg(test)]
mod day17_tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = r#"Register A: 0
Register B: 0
Register C: 9

Program: 2,6"#;
        let mut computer = Computer::from(input);
        computer.execute_program();
        assert_eq!(*computer.registers.get(&'b').unwrap(), 1);
    }

    #[test]
    fn test_2() {
        let input = r#"Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4"#;
        let mut computer = Computer::from(input);
        computer.execute_program();
        assert_eq!(computer.output, "0,1,2".to_string());
    }

    #[test]
    fn test_3() {
        let input = r#"Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
        let mut computer = Computer::from(input);

        computer.execute_program();
        assert_eq!(computer.output, "4,2,5,6,7,7,7,7,3,1,0".to_string());
        assert_eq!(*computer.registers.get(&'a').unwrap(), 0);
    }

    #[test]
    fn test_4() {
        let input = r#"Register A: 0
Register B: 29
Register C: 0

Program: 1,7"#;
        let mut computer = Computer::from(input);

        computer.execute_program();
        assert_eq!(*computer.registers.get(&'b').unwrap(), 26);
    }

    #[test]
    fn test_5() {
        let input = r#"Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0"#;
        let mut computer = Computer::from(input);

        computer.execute_program();
        assert_eq!(*computer.registers.get(&'b').unwrap(), 44354);
    }

    #[test]
    fn part1() {
        let input = r#"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"#;
        let mut computer = Computer::from(input);
        computer.execute_program();
        assert_eq!(computer.output, "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}
