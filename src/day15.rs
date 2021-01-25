use std::collections::HashMap;

use crate::utils::stdin_to_vec;

struct Game {
    turns: Vec<usize>,
    number_positions: HashMap<usize, usize>,
    number_spoken_count: HashMap<usize, usize>,
}

impl Game {
    pub fn new(starting_numbers: Vec<usize>) -> Self {
        println!("Starting numbers: {:?}", &starting_numbers);

        let mut number_positions = HashMap::<usize, usize>::new();
        let mut number_spoken_count = HashMap::<usize, usize>::new();

        for (num_turn, num) in starting_numbers.iter().enumerate() {
            number_positions.insert(*num, num_turn);
            number_spoken_count.insert(*num, 0);
        }        

        Self {
            turns: starting_numbers,
            number_positions,
            number_spoken_count,
        }
    }

    pub fn turn_count(&self) -> usize {
        self.turns.len()
    }

    pub fn next_turn(&mut self) -> usize {
        let last_spoken_num_pos = self.turns.len() - 1;
        let last_spoken_num = &self.turns[last_spoken_num_pos];

        let next_num = if *self.number_spoken_count.get(last_spoken_num).unwrap() > 0 {
            last_spoken_num_pos - *self.number_positions.get(last_spoken_num).unwrap()
        } else {
            0
        };

        self.number_positions
            .insert(*last_spoken_num, last_spoken_num_pos);

        self.turns.push(next_num);

        if let Some(number_spoken_count) = self.number_spoken_count.get_mut(&next_num) {
            *number_spoken_count += 1;
        } else {
            self.number_spoken_count.insert(next_num, 0);
        }

        next_num
    }

    pub fn get_last_turn_number(&self) -> usize {
        *self.turns.last().unwrap()
    }
}

pub fn get_starting_numbers() -> Vec<usize> {
    let input = stdin_to_vec::<String>();
    assert_eq!(input.len(), 1);

    input[0]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>()
}

pub fn part1() {
    let mut game = Game::new(get_starting_numbers());

    while game.turn_count() < 2020 {
        println!("Turn number: {}", game.next_turn());
    }
}

pub fn part2() {
    let mut game = Game::new(get_starting_numbers());

    while game.turn_count() < 30000000 {
        game.next_turn();
    }

    println!("Last turn number: {}", game.get_last_turn_number());
}
