use crate::utils;
use std::{fmt::Debug, str::FromStr, string::ParseError};
use utils::stdin_to_vec;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    ACC,
    JMP,
    NOP,
}

impl Operation {
    pub fn from(op_string: &str) -> Operation {
        match op_string {
            "acc" => Operation::ACC,
            "jmp" => Operation::JMP,
            "nop" => Operation::NOP,
            _ => panic!("Unexpected operation: {}", op_string),
        }
    }
}
#[derive(Debug, Clone)]
struct Instruction {
    operation: Operation,
    argument: i32,
}

impl Instruction {
    pub fn parse(&mut self, instruction_line: &str) {
        let instruction_items: Vec<&str> = instruction_line.split(' ').collect();
        assert_eq!(instruction_items.len(), 2);

        self.operation = Operation::from(instruction_items[0]);
        self.argument = instruction_items[1].parse::<i32>().unwrap();
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instance = Self {
            operation: Operation::NOP,
            argument: 0,
        };

        instance.parse(s);

        Ok(instance)
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    accumulator: i32,
    instruction_index: i32,
    executed_instruction_indexes: Vec<i32>,
}

impl Program {
    pub fn new(instructions: &Vec<Instruction>) -> Self {
        Self {
            instructions: instructions.clone(),
            accumulator: 0,
            instruction_index: 0,
            executed_instruction_indexes: vec![],
        }
    }

    fn exec_instruction(&mut self) {
        assert!(
            self.instruction_index >= 0
                && (self.instruction_index as usize) < self.instructions.len()
        );

        let instruction = &self.instructions[self.instruction_index as usize];

        self.executed_instruction_indexes
            .push(self.instruction_index);

        match instruction.operation {
            Operation::NOP => self.instruction_index += 1,
            Operation::JMP => self.instruction_index += instruction.argument,
            Operation::ACC => {
                self.accumulator += instruction.argument;
                self.instruction_index += 1
            }
            _ => (),
        }

        println!(
            "Executed instruction: {:?}, accumulator: {}",
            instruction, self.accumulator
        );
    }

    pub fn is_loop_detected(&self) -> bool {
        self.executed_instruction_indexes
            .contains(&self.instruction_index)
    }

    pub fn is_terminated_normally(&self) -> bool {
        self.instruction_index >= (self.instructions.len() as i32)
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.instruction_index = 0;
        self.executed_instruction_indexes = vec![];
    }

    pub fn run(&mut self) {
        &self.reset();

        while !self.is_loop_detected() && !self.is_terminated_normally() {
            self.exec_instruction();
        }
    }
}

pub fn part1() {
    let instructions = stdin_to_vec::<Instruction>();

    let mut program = Program::new(&instructions);
    program.run();

    println!("Accumulator: {}", program.accumulator);
}

pub fn part2() {
    let instructions = &stdin_to_vec::<Instruction>();

    for i in 0..instructions.len() {
        let operation = &instructions[i].operation;

        let try_operation = match operation {
            Operation::NOP => Some(Operation::JMP),
            Operation::JMP => Some(Operation::NOP),
            _ => None,
        };

        if let Some(try_operation) = try_operation {
            let mut try_instructions = instructions.clone();
            try_instructions[i].operation = try_operation;

            let mut program = Program::new(&try_instructions);
            program.run();

            if program.is_terminated_normally() {
                println!(
                    "Program terminated normally after fixing instruction {} from {:?} to {:?}.\nAccumulator: {}",
                    i,
                    &instructions[i],
                    &try_instructions[i],
                    program.accumulator
                );
                break;
            }
        }
    }
}
