use std::{num::ParseIntError, str::FromStr};

use crate::utils::stdin_to_vec;
#[derive(Debug, Clone)]
enum Instruction {
    MoveNorth(usize),
    MoveSouth(usize),
    MoveEast(usize),
    MoveWest(usize),
    TurnLeft(usize),
    TurnRight(usize),
    MoveForward(usize),
}

impl Instruction {
    pub fn from(instruction_string: &str) -> Self {
        let action_code = &instruction_string[0..1];
        let action_value_string = &instruction_string[1..];
        let action_value: usize = action_value_string.parse().unwrap();

        match action_code {
            "N" => Instruction::MoveNorth(action_value),
            "S" => Instruction::MoveSouth(action_value),
            "E" => Instruction::MoveEast(action_value),
            "W" => Instruction::MoveWest(action_value),
            "L" => Instruction::TurnLeft(action_value),
            "R" => Instruction::TurnRight(action_value),
            "F" => Instruction::MoveForward(action_value),
            _ => panic!("Unexpected instruction: {}", instruction_string),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instruction::from(s))
    }
}

const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

#[derive(Debug)]
struct NavigationPart1 {
    instructions: Vec<Instruction>,
    position: [usize; 4],
    direction: usize,
}

impl NavigationPart1 {
    pub fn new(instructions: &Vec<Instruction>) -> Self {
        Self {
            instructions: instructions.clone(),
            position: [0, 0, 0, 0],
            direction: EAST,
        }
    }

    fn find_direction(&self, angle: i32) -> usize {
        let shift_count = angle / 90;
        let mut direction = self.direction as i32 + shift_count;

        if direction < 0 {
            direction += self.position.len() as i32;
        } else {
            direction %= self.position.len() as i32;
        }

        direction as usize
    }

    pub fn navigate(&mut self) {
        for instruction in &self.instructions {
            match instruction {
                Instruction::MoveNorth(action_value) => self.position[NORTH] += action_value,
                Instruction::MoveSouth(action_value) => self.position[SOUTH] += action_value,
                Instruction::MoveEast(action_value) => self.position[EAST] += action_value,
                Instruction::MoveWest(action_value) => self.position[WEST] += action_value,
                Instruction::TurnLeft(angle) => {
                    self.direction = self.find_direction(-(*angle as i32));
                }
                Instruction::TurnRight(angle) => {
                    self.direction = self.find_direction(*angle as i32);
                }
                Instruction::MoveForward(action_value) => {
                    self.position[self.direction] += action_value;
                }
            }
        }
    }

    pub fn get_distance(&self) -> usize {
        let west_east_dist = (self.position[WEST] as i32 - self.position[EAST] as i32).abs();
        let north_south_dist = (self.position[NORTH] as i32 - self.position[SOUTH] as i32).abs();

        (west_east_dist + north_south_dist) as usize
    }
}

type Position = [usize; 4];
type Direction = [usize; 4];

#[derive(Debug)]
struct NavigationPart2 {
    instructions: Vec<Instruction>,
    viewpoint_position: Position,
    ship_position: Position,
    direction: Direction,
}

impl NavigationPart2 {
    pub fn new(instructions: &Vec<Instruction>) -> Self {
        let mut instance = Self {
            instructions: instructions.clone(),
            viewpoint_position: [0, 0, 0, 0],
            ship_position: [0, 0, 0, 0],
            direction: [NORTH, EAST, SOUTH, WEST],
        };

        instance.viewpoint_position[NORTH] = 1;
        instance.viewpoint_position[EAST] = 10;

        instance
    }

    fn rotate_viewpoint(&self, angle: i32) -> Position {
        let shift_count = angle / 90;
        let mut viewpoint_position = self.viewpoint_position.clone();

        if shift_count < 0 {
            viewpoint_position.rotate_left(shift_count.abs() as usize);
        } else {
            viewpoint_position.rotate_right(shift_count.abs() as usize);
        }

        viewpoint_position
    }

    pub fn navigate(&mut self) {
        for instruction in &self.instructions {
            match instruction {
                Instruction::MoveNorth(action_value) => {
                    self.viewpoint_position[NORTH] += action_value
                }
                Instruction::MoveSouth(action_value) => {
                    self.viewpoint_position[SOUTH] += action_value
                }
                Instruction::MoveEast(action_value) => {
                    self.viewpoint_position[EAST] += action_value
                }
                Instruction::MoveWest(action_value) => {
                    self.viewpoint_position[WEST] += action_value
                }
                Instruction::TurnLeft(angle) => {
                    self.viewpoint_position = self.rotate_viewpoint(-(*angle as i32));
                }
                Instruction::TurnRight(angle) => {
                    self.viewpoint_position = self.rotate_viewpoint(*angle as i32);
                }
                Instruction::MoveForward(action_value) => {
                    for i in 0..self.ship_position.len() {
                        self.ship_position[i] += self.viewpoint_position[i] * action_value;
                    }
                }
            }
        }
    }

    pub fn get_distance(&self) -> usize {
        let west_east_dist =
            (self.ship_position[WEST] as i32 - self.ship_position[EAST] as i32).abs();

        let north_south_dist =
            (self.ship_position[NORTH] as i32 - self.ship_position[SOUTH] as i32).abs();

        (west_east_dist + north_south_dist) as usize
    }
}

pub fn part1() {
    let instructions = stdin_to_vec::<Instruction>();
    let mut navigation = NavigationPart1::new(&instructions);

    navigation.navigate();
    println!("Distance: {}", navigation.get_distance());
}

pub fn part2() {
    let instructions = stdin_to_vec::<Instruction>();
    let mut navigation = NavigationPart2::new(&instructions);

    navigation.navigate();
    println!("Distance: {}", navigation.get_distance());
}
