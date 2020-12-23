use std::{
    io::{self, BufRead},
    str::FromStr,
};

pub fn stdin_to_vec<T: FromStr>() -> Vec<T> {
    let mut list: Vec<T> = vec![];

    for line_result in io::stdin().lock().lines() {
        let line = line_result.unwrap();
        let item = line.parse::<T>();

        match item {
            Ok(value) => list.push(value),
            Err(_) => println!("Could not parse line: '{}'.", line),
        }
    }

    list
}
