use std::{
    io::{self, BufRead},
    ops::{BitAnd, BitOr, BitXor, Shl, Shr},
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

pub fn check_bit<T: From<u8> + PartialEq + BitAnd<Output = T> + Shr<Output = T> + Copy>(
    num: T,
    position: u8,
) -> bool {
    (num >> T::from(position)) & T::from(1_u8) == T::from(1_u8)
}

pub fn toggle_bit<T: From<u8> + BitXor<Output = T> + Shl<Output = T> + Copy>(
    num: T,
    position: u8,
) -> T {
    num ^ (T::from(1_u8) << T::from(position))
}

pub fn set_bit<
    T: From<u8> + BitOr<Output = T> + BitAnd<Output = T> + BitXor<Output = T> + Shl<Output = T> + Copy,
>(
    num: T,
    position: u8,
    set: bool,
) -> T {
    if set {
        num | (T::from(1_u8) << T::from(position))
    } else {
        num & toggle_bit(num, position)
    }
}
