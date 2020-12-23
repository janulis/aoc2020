use std::{env, process};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn help() {
    println!(
        "Usage: ./aoc2020 <day_num> <part_num>
        where <day_num>: [1..9], <part_num>: [1..2]"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        help();
        return;
    }

    let day_num = &args[1].parse::<u8>().unwrap_or_else(|errror| {
        help();
        process::exit(1);
    });

    let part_num = &args[2].parse::<u8>().unwrap_or_else(|error| {
        help();
        process::exit(1);
    });

    match (day_num, part_num) {
        (1, 1) => day1::part1(),
        (1, 2) => day1::part2(),
        (2, 1) => day2::part1(),
        (2, 2) => day2::part2(),
        (3, 1) => day3::part1(),
        (3, 2) => day3::part2(),
        (4, 1) => day4::part1(),
        (4, 2) => day4::part2(),
        (5, 1) => day5::part1(),
        (5, 2) => day5::part2(),
        (6, 1) => day6::part1(),
        (6, 2) => day6::part2(),
        (7, 1) => day7::part1(),
        (7, 2) => day7::part2(),
        (8, 1) => day8::part1(),
        (8, 2) => day8::part2(),
        (9, 1) => day9::part1(),
        (9, 2) => day9::part2(),
        _ => help(),
    }
}
