use core::panic;
use io::stdin;
use std::io::{self, Read};

#[derive(Debug)]
struct BoardingPass {
    seat_code: String,
    num_rows: u8,
    num_cols: u8,
    row: u8,
    col: u8,
}

impl BoardingPass {
    pub fn new(seat_code: String) -> Self {
        let mut instance = Self {
            seat_code: String::new(),
            num_rows: 128,
            num_cols: 8,
            row: 0,
            col: 0,
        };

        instance.parse(seat_code);

        instance
    }

    fn partition(&self, min: u8, max: u8, lower_half: bool) -> (u8, u8) {
        let mid = min + (max - min) / 2;

        if lower_half {
            (min, mid)
        } else {
            (mid + 1, max)
        }
    }

    fn parse(&mut self, seat_code: String) {
        self.seat_code = seat_code;

        let mut row_bounds = (0, self.num_rows - 1);
        let mut col_bounds = (0, self.num_cols - 1);

        for c in self.seat_code.chars() {
            match c {
                'F' => row_bounds = self.partition(row_bounds.0, row_bounds.1, true),
                'B' => row_bounds = self.partition(row_bounds.0, row_bounds.1, false),
                'L' => col_bounds = self.partition(col_bounds.0, col_bounds.1, true),
                'R' => col_bounds = self.partition(col_bounds.0, col_bounds.1, false),
                _ => panic!("Unexpected seat code character {}", c),
            }
        }

        assert_eq!(row_bounds.0, row_bounds.1);
        self.row = row_bounds.0;

        assert_eq!(col_bounds.0, col_bounds.1);
        self.col = col_bounds.0;
    }

    pub fn get_seat_id(&self) -> usize {
        (self.row as usize) * (self.num_cols as usize) + (self.col as usize)
    }
}

#[test]
fn test_parse() {
    let boarding_pass = BoardingPass::new("BFFFBBFRRR".to_string());
    assert_eq!(boarding_pass.row, 70);
    assert_eq!(boarding_pass.col, 7);
    assert_eq!(boarding_pass.get_seat_id(), 567);

    let boarding_pass = BoardingPass::new("FFFBBBFRRR".to_string());
    assert_eq!(boarding_pass.row, 14);
    assert_eq!(boarding_pass.col, 7);
    assert_eq!(boarding_pass.get_seat_id(), 119);

    let boarding_pass = BoardingPass::new("BBFFBBFRLL".to_string());
    assert_eq!(boarding_pass.row, 102);
    assert_eq!(boarding_pass.col, 4);
    assert_eq!(boarding_pass.get_seat_id(), 820);
}

fn stdin_to_boarding_passes() -> Vec<BoardingPass> {
    let mut input_str = String::new();
    stdin().read_to_string(&mut input_str);

    let mut boarding_passes: Vec<BoardingPass> = vec![];

    for seat_code in input_str.split('\n') {
        boarding_passes.push(BoardingPass::new(seat_code.to_string()));
    }

    boarding_passes
}

pub fn part1() {
    let boarding_passes = stdin_to_boarding_passes();
    // println!("{:?}", boarding_passes);

    let mut max_seat_id = 0;
    for boarding_pass in boarding_passes {
        let seat_id = boarding_pass.get_seat_id();

        if seat_id > max_seat_id {
            max_seat_id = seat_id;
        }
    }

    println!("Max boarding pass seat id: {}", max_seat_id);
}

pub fn part2() {
    let boarding_passes = stdin_to_boarding_passes();

    let mut seat_ids: Vec<usize> = vec![];
    for boarding_pass in boarding_passes {
        if boarding_pass.row == 0 || boarding_pass.row == (boarding_pass.num_rows - 1) {
            println!("Skipping {:?}", boarding_pass);
            continue;
        }

        seat_ids.push(boarding_pass.get_seat_id());
    }

    seat_ids.sort();

    for i in 0..seat_ids.len() - 1 {
        if seat_ids[i + 1] == (seat_ids[i] + 2) {
            println!("Found seat: {}", seat_ids[i] + 1);
        }
    }
}
