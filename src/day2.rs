use crate::utils;
use std::{fmt::Debug, str::FromStr, string::ParseError};
use utils::stdin_to_vec;

pub struct Password {
    pub policy_number1: usize,
    pub plolicy_number2: usize,
    pub policy_letter: char,
    pub password: String,
}

impl FromStr for Password {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let minus_pos = chars.position(|c| c == '-').unwrap();
        let policy_number1 = String::from(&s[..minus_pos]).parse::<usize>().unwrap();

        chars = s.chars();
        let space_pos = chars.position(|c| c == ' ').unwrap();
        let plolicy_number2 = String::from(&s[minus_pos + 1..space_pos])
            .parse::<usize>()
            .unwrap();

        let policy_letter = chars.next().unwrap();

        assert_eq!(Some(':'), chars.next());
        assert_eq!(Some(' '), chars.next());

        let password = chars.collect();

        Ok(Self {
            policy_number1,
            plolicy_number2,
            policy_letter,
            password,
        })
    }
}

impl Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(policy_number1={}, policy_number2={}, policy_letter={}, password={})",
            self.policy_number1, self.plolicy_number2, self.policy_letter, self.password
        )
    }
}

fn is_password_valid_part1(p: &Password) -> bool {
    let mut letter_count = 0;
    for c in p.password.chars() {
        if c == p.policy_letter {
            letter_count += 1;
        }
    }

    letter_count >= p.policy_number1 && letter_count <= p.plolicy_number2
}

pub fn part1() {
    let passwords: Vec<Password> = stdin_to_vec::<Password>();

    let mut valid_password_count = 0;
    for password in &passwords {
        if is_password_valid_part1(password) {
            println!("Password is valid: {:?}", password);
            valid_password_count += 1;
        } else {
            println!("Password is NOT valid: {:?}", password);
        }
    }

    println!("Valid password count: {}", valid_password_count);
}

fn is_password_valid_part2(p: &Password) -> bool {
    let password_letters: Vec<char> = p.password.chars().collect();
    let letter1: char = password_letters[p.policy_number1 - 1];
    let letter2: char = password_letters[p.plolicy_number2 - 1];

    (letter1 == p.policy_letter || letter2 == p.policy_letter) && letter1 != letter2
}

pub fn part2() {
    let passwords: Vec<Password> = utils::stdin_to_vec::<Password>();

    let mut valid_password_count = 0;
    for password in &passwords {
        if is_password_valid_part2(password) {
            println!("Password is valid: {:?}", password);
            valid_password_count += 1;
        } else {
            println!("Password is NOT valid: {:?}", password);
        }
    }

    println!("Valid password count: {}", valid_password_count);
}
