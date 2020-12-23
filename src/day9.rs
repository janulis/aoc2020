use io::stdin;
use std::{
    io::{self, Read},
    vec,
};

fn stdin_to_ints() -> Vec<i64> {
    let mut input_str = String::new();
    stdin().read_to_string(&mut input_str);

    let mut numbers = vec![];

    for field_str in input_str.split('\n') {
        let number = field_str.parse::<i64>().unwrap();
        numbers.push(number);
    }

    numbers
}

fn find_first_invalid_number(numbers: &Vec<i64>, preamble_len: usize) -> Option<i64> {
    for i in preamble_len..numbers.len() {
        let num_to_validate = numbers[i];
        let mut is_valid_num = false;

        let preamble_start = i - preamble_len;
        let preamble_end = i;

        for j in preamble_start..preamble_end {
            let preamble_num1 = numbers[j];
            let preamble_num2 = num_to_validate - preamble_num1;

            if (&numbers[(j + 1)..preamble_end]).contains(&preamble_num2) {
                is_valid_num = true;
                break;
            }
        }

        if !is_valid_num {
            return Some(num_to_validate);
        }
    }

    None
}

pub fn part1() {
    let numbers = stdin_to_ints();
    let preamble_len = 25;

    if let Some(invalid_number) = find_first_invalid_number(&numbers, preamble_len) {
        println!("Number {} is invalid", invalid_number);
    }
}

pub fn part2() {
    let numbers = stdin_to_ints();
    let preamble_len = 25;

    if let Some(invalid_number) = find_first_invalid_number(&numbers, preamble_len) {
        for i in 0..numbers.len() {
            let mut range_number_sum = numbers[i];
            let mut range_min_number = numbers[i];
            let mut range_max_number = numbers[i];

            for j in (i + 1)..numbers.len() {
                let number = numbers[j];                
                range_number_sum += number;

                if number < range_min_number {
                    range_min_number = number;
                } else if number > range_max_number {
                    range_max_number = number;
                }

                if range_number_sum == invalid_number {
                    let result = range_min_number + range_max_number;
                    println!("Encryption weakness: {}", result);
                    return;
                } else if range_number_sum > invalid_number {
                    break;
                }
            }
        }
    }
}
