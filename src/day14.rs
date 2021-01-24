use crate::utils::{self, stdin_to_vec};
use regex::Regex;
use std::{collections::HashMap, str::FromStr, string::ParseError, vec};

#[derive(Clone, Debug)]
enum Instruction {
    Mask(String),
    Mem(u64, u64),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        let mem_regex = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
        let mask_regex = Regex::new(r"mask = (?P<mask>[01X]{36})").unwrap();

        if let Some(captures) = mem_regex.captures(input_str) {
            return Ok(Instruction::Mem(
                captures["address"].parse::<u64>().unwrap(),
                captures["value"].parse::<u64>().unwrap(),
            ));
        } else if let Some(captures) = mask_regex.captures(input_str) {
            return Ok(Instruction::Mask(captures["mask"].to_string()));
        } else {
            panic!("Unsupported input: {}", input_str);
        }
    }
}

struct Program {
    instructions: Vec<Instruction>,
    mem: HashMap<u64, u64>,
}

impl Program {
    fn new(instructions: &Vec<Instruction>) -> Self {
        Self {
            instructions: instructions.clone(),
            mem: HashMap::new(),
        }
    }

    fn apply_mask_part1(&self, value: u64, mask: &str) -> u64 {
        let mut masked_value = value;

        for (bit_pos, mask_bit) in mask.chars().rev().enumerate() {
            if mask_bit != 'X' {
                masked_value = utils::set_bit(masked_value, bit_pos as u8, mask_bit == '1');
            }
        }

        masked_value
    }

    fn run_part1(&mut self) -> u64 {
        let mut current_mask = "";

        for instruction in &self.instructions {
            match instruction {
                Instruction::Mask(mask) => current_mask = mask,
                Instruction::Mem(address, value) => {
                    self.mem
                        .insert(*address, self.apply_mask_part1(*value, current_mask));
                }
            }
        }

        self.mem.values().sum::<u64>()
    }

    fn apply_mask_part2(&self, address: u64, mask: &str) -> Vec<u64> {
        let mut masked_address = address;
        let mut x_bit_positions: Vec<u8> = vec![];

        for (bit_pos, mask_bit) in mask.chars().rev().enumerate() {
            if mask_bit == 'X' {
                x_bit_positions.push(bit_pos as u8);
            } else if mask_bit == '1' {
                masked_address = utils::set_bit(masked_address, bit_pos as u8, true);
            }
        }

        let address_combination_count = 2_u16.pow(x_bit_positions.len() as u32);
        let mut addresses = Vec::<u64>::with_capacity(address_combination_count as usize);

        for num in 0..address_combination_count {
            let mut address = masked_address;

            for (bit_pos, x_bit_pos) in x_bit_positions.iter().enumerate() {
                address = utils::set_bit(
                    address,
                    *x_bit_pos as u8,
                    utils::check_bit(num, bit_pos as u8),
                );
            }

            addresses.push(address);
        }

        addresses
    }

    fn run_part2(&mut self) -> u64 {
        let mut current_mask = "";

        for instruction in &self.instructions {
            match instruction {
                Instruction::Mask(mask) => current_mask = mask,
                Instruction::Mem(address, value) => {
                    for addr in &self.apply_mask_part2(*address, current_mask) {
                        self.mem.insert(*addr, *value);
                    }
                }
            }
        }

        self.mem.values().sum::<u64>()
    }
}

pub fn part1() {
    let instructions = stdin_to_vec::<Instruction>();
    println!("{:?}", instructions);

    let mut program = Program::new(&instructions);

    println!("Result: {:?}", program.run_part1());
}

pub fn part2() {
    let instructions = stdin_to_vec::<Instruction>();
    println!("{:?}", instructions);

    let mut program = Program::new(&instructions);

    println!("Result: {:?}", program.run_part2());
}
