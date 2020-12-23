use io::stdin;
use std::io::{self, Read};

#[derive(Debug)]
struct Passenger {
    questions: String,
}

#[derive(Debug)]
struct PassengerGroup {
    passengers: Vec<Passenger>,
}

impl Passenger {
    pub fn new(questions: String) -> Self {
        Self { questions }
    }
}

impl PassengerGroup {
    pub fn new() -> Self {
        Self { passengers: vec![] }
    }

    fn add_passenger(&mut self, passenger: Passenger) {
        self.passengers.push(passenger);
    }

    fn get_unique_questions(&self) -> String {
        let mut unique_questions = String::new();

        for passenger in &self.passengers {
            for q in passenger.questions.chars() {
                if !unique_questions.contains(q) {
                    unique_questions.push(q);
                }
            }
        }

        unique_questions
    }

    fn get_common_questions(&self) -> String {
        let mut common_questions = String::new();

        if self.passengers.is_empty() {
            return common_questions;
        }

        let first_passenger_questions = &self.passengers.first().unwrap().questions;

        if self.passengers.len() == 1 {
            return first_passenger_questions.clone();
        }

        for first_passenger_question in first_passenger_questions.chars() {
            let mut is_common_qustion = true;

            for i in 1..self.passengers.len() {
                if !self.passengers[i]
                    .questions
                    .contains(first_passenger_question)
                {
                    is_common_qustion = false;
                    break;
                }
            }

            if is_common_qustion {
                common_questions.push(first_passenger_question);
            }
        }

        common_questions
    }
}

fn stdin_to_passenger_groups() -> Vec<PassengerGroup> {
    let mut input_str = String::new();
    stdin().read_to_string(&mut input_str);

    let mut passenger_groups: Vec<PassengerGroup> = vec![PassengerGroup::new()];

    for field_str in input_str.split('\n') {
        if field_str.trim().is_empty() {
            passenger_groups.push(PassengerGroup::new());
            continue;
        }

        let mut passenger_group = passenger_groups.last_mut().unwrap();
        passenger_group.add_passenger(Passenger::new(field_str.to_string()));
    }

    println!("{:?}", passenger_groups);
    passenger_groups
}

pub fn part1() {
    let passenger_groups = stdin_to_passenger_groups();

    let total_questions = passenger_groups
        .iter()
        .map(|pg| pg.get_unique_questions().chars().count())
        .sum::<usize>();

    println!("Total number of unique questions: {}", total_questions);
}

pub fn part2() {
    let passenger_groups = stdin_to_passenger_groups();

    let total_questions = passenger_groups
        .iter()
        .map(|pg| pg.get_common_questions().chars().count())
        .sum::<usize>();

    println!("Total number of common questions: {}", total_questions);
}
