use crate::utils;
use utils::stdin_to_vec;

type SeatState = char;
type SeatLayoutRow = Vec<SeatState>;

const FLOOR: SeatState = '.';
const EMPTY_SEAT: SeatState = 'L';
const OCCUPIED_SEAT: SeatState = '#';

#[derive(Debug, Clone, PartialEq)]
struct SeatLayout {
    rows: Vec<SeatLayoutRow>,
}

impl SeatLayout {
    pub fn new(encoded_rows: &Vec<String>) -> Self {
        let mut rows: Vec<SeatLayoutRow> = vec![];

        for encoded_row in encoded_rows {
            let row: Vec<SeatState> = encoded_row.chars().collect();
            rows.push(row);
        }

        Self { rows }
    }

    pub fn is_seat_occupied(&self, row: usize, col: usize) -> bool {
        if row >= self.rows.len() || col >= self.rows[row].len() {
            return false;
        }

        self.rows[row][col] == OCCUPIED_SEAT
    }

    pub fn get_occupied_adjacent_immediate_seat_count(&self, row: usize, col: usize) -> usize {
        let adjacent_seat_positions = [
            (row.checked_sub(1), col.checked_sub(1)),
            (row.checked_sub(1), Some(col)),
            (row.checked_sub(1), col.checked_add(1)),
            (Some(row), col.checked_sub(1)),
            (Some(row), col.checked_add(1)),
            (row.checked_add(1), col.checked_sub(1)),
            (row.checked_add(1), Some(col)),
            (row.checked_add(1), col.checked_add(1)),
        ];

        let mut occupied_seat_count: usize = 0;
        for adjacent_seat_pos in &adjacent_seat_positions {
            if let (Some(row), Some(col)) = (adjacent_seat_pos.0, adjacent_seat_pos.1) {
                if self.is_seat_occupied(row, col) {
                    occupied_seat_count += 1;
                }
            }
        }

        occupied_seat_count
    }

    fn get_first_visible_seat_position(
        &self,
        row: usize,
        row_offet: i32,
        col: usize,
        col_offset: i32,
    ) -> (Option<usize>, Option<usize>) {
        let mut try_row = row as i32;
        let mut try_col = col as i32;

        loop {
            try_row += row_offet;
            try_col += col_offset;

            if try_row < 0
                || try_row as usize >= self.rows.len()
                || try_col < 0
                || try_col as usize >= self.rows[try_row as usize].len()
            {
                return (None, None);
            }

            if self.rows[try_row as usize][try_col as usize] != FLOOR {
                break;
            }
        }

        (Some(try_row as usize), Some(try_col as usize))
    }

    pub fn get_occupied_adjacent_first_visible_seat_count(&self, row: usize, col: usize) -> usize {
        let adjacent_seat_positions = [
            self.get_first_visible_seat_position(row, -1, col, -1),
            self.get_first_visible_seat_position(row, -1, col, 0),
            self.get_first_visible_seat_position(row, -1, col, 1),
            self.get_first_visible_seat_position(row, 0, col, -1),
            self.get_first_visible_seat_position(row, 0, col, 1),
            self.get_first_visible_seat_position(row, 1, col, -1),
            self.get_first_visible_seat_position(row, 1, col, 0),
            self.get_first_visible_seat_position(row, 1, col, 1),
        ];

        let mut occupied_seat_count: usize = 0;
        for adjacent_seat_pos in &adjacent_seat_positions {
            if let (Some(row), Some(col)) = (adjacent_seat_pos.0, adjacent_seat_pos.1) {
                if self.is_seat_occupied(row, col) {
                    occupied_seat_count += 1;
                }
            }
        }

        occupied_seat_count
    }

    pub fn get_occupied_seat_count(&self) -> usize {
        let mut occupied_seat_count = 0;

        for row in 0..self.rows.len() {
            for col in 0..self.rows[row].len() {
                if self.is_seat_occupied(row, col) {
                    occupied_seat_count += 1;
                }
            }
        }

        occupied_seat_count
    }

    pub fn simulate_seating_part1(&mut self) -> bool {
        let prev_layout = self.clone();

        for row in 0..self.rows.len() {
            for col in 0..self.rows[row].len() {
                match prev_layout.rows[row][col] {
                    EMPTY_SEAT => {
                        if prev_layout.get_occupied_adjacent_immediate_seat_count(row, col) == 0 {
                            self.rows[row][col] = OCCUPIED_SEAT;
                        }
                    }
                    OCCUPIED_SEAT => {
                        if prev_layout.get_occupied_adjacent_immediate_seat_count(row, col) >= 4 {
                            self.rows[row][col] = EMPTY_SEAT;
                        }
                    }
                    _ => (),
                }
            }
        }

        *self != prev_layout
    }

    pub fn simulate_seating_part2(&mut self) -> bool {
        let prev_layout = self.clone();

        for row in 0..self.rows.len() {
            for col in 0..self.rows[row].len() {
                match prev_layout.rows[row][col] {
                    EMPTY_SEAT => {
                        if prev_layout.get_occupied_adjacent_first_visible_seat_count(row, col) == 0
                        {
                            self.rows[row][col] = OCCUPIED_SEAT;
                        }
                    }
                    OCCUPIED_SEAT => {
                        if prev_layout.get_occupied_adjacent_first_visible_seat_count(row, col) >= 5
                        {
                            self.rows[row][col] = EMPTY_SEAT;
                        }
                    }
                    _ => (),
                }
            }
        }

        *self != prev_layout
    }

    pub fn print(&self) {
        println!();

        for row in &self.rows {
            for col in row {
                print!("{}", col);
            }
            println!();
        }

        println!("Occupied seats: {}", self.get_occupied_seat_count());
    }
}

pub fn part1() {
    let rows = &stdin_to_vec::<String>();
    let mut layout = SeatLayout::new(&rows);

    loop {
        let state_changed = layout.simulate_seating_part1();

        if !state_changed {
            break;
        }

        layout.print();
    }
}

pub fn part2() {
    let rows = &stdin_to_vec::<String>();
    let mut layout = SeatLayout::new(&rows);

    loop {
        let state_changed = layout.simulate_seating_part2();

        if !state_changed {
            break;
        }

        layout.print();
    }
}
