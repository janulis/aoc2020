use io::stdin;
use std::io::{self, Read};
use std::{collections::HashMap, vec};

#[derive(Debug)]
struct Passport {
    required_field_keys: Vec<String>,
    fields: HashMap<String, String>,
}

impl Passport {
    pub fn new() -> Self {
        Self {
            required_field_keys: [
                "byr".to_string(),
                "iyr".to_string(),
                "eyr".to_string(),
                "hgt".to_string(),
                "hcl".to_string(),
                "ecl".to_string(),
                "pid".to_string(),
            ]
            .to_vec(),
            fields: HashMap::new(),
        }
    }

    pub fn is_valid_part1(&self) -> bool {
        for required_field_key in &self.required_field_keys {
            if !self.fields.contains_key(required_field_key) {
                return false;
            }
        }

        true
    }

    fn is_valid_year(&self, value: &str, min: u32, max: u32) -> bool {
        if value.chars().count() != 4 {
            return false;
        }

        if let Ok(year) = value.parse::<u32>() {
            year >= min && year <= max
        } else {
            false
        }
    }

    fn is_valid_height(&self, value: &str) -> bool {
        if value.ends_with("cm") {
            if let Ok(height) = value.trim_end_matches("cm").parse::<u32>() {
                return height >= 150 && height <= 193;
            } else {
                return false;
            }
        } else if value.ends_with("in") {
            if let Ok(height) = value.trim_end_matches("in").parse::<u32>() {
                return height >= 59 && height <= 76;
            } else {
                return false;
            }
        }

        false
    }

    fn is_valid_hair_color(&self, value: &str) -> bool {
        if value.chars().count() != 7 {
            return false;
        }

        let mut chars = value.chars();
        if let Some(c) = chars.next() {
            if c != '#' {
                return false;
            }
        }
        return chars.all(|c| c.is_numeric() || c.is_ascii_hexdigit());
    }

    fn is_valid_eye_color(&self, value: &str) -> bool {
        let eye_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        eye_colors.contains(&value)
    }

    fn is_valid_passport_id(&self, value: &str) -> bool {
        if value.chars().count() != 9 {
            return false;
        }

        return value.chars().all(char::is_numeric);
    }

    pub fn is_valid_part2(&self) -> bool {
        for required_field_key in &self.required_field_keys {
            if !self.fields.contains_key(required_field_key) {
                return false;
            }

            let key = required_field_key.as_str();
            let value = self.fields.get(key).unwrap().as_str();
            let value_valid = match key {
                "byr" => self.is_valid_year(value, 1920, 2002),
                "iyr" => self.is_valid_year(value, 2010, 2020),
                "eyr" => self.is_valid_year(value, 2020, 2030),
                "hgt" => self.is_valid_height(value),
                "hcl" => self.is_valid_hair_color(value),
                "ecl" => self.is_valid_eye_color(value),
                "pid" => self.is_valid_passport_id(value),
                _ => panic!("Unexpected value: {}", value),
            };

            if !value_valid {
                return false;
            }
        }

        true
    }
}

#[test]
fn test_is_valid_year() {
    let passport = Passport::new();
    assert!(passport.is_valid_year("2002", 1920, 2002));
    assert!(!passport.is_valid_year("2003", 1920, 2002));
    assert!(!passport.is_valid_year("1919", 1920, 2002));
    assert!(!passport.is_valid_year("20020", 1920, 2002));
}

#[test]
fn test_is_valid_height() {
    let passport = Passport::new();
    assert!(passport.is_valid_height("60in"));
    assert!(passport.is_valid_height("190cm"));
    assert!(!passport.is_valid_height("190in"));
    assert!(!passport.is_valid_height("190"));
}

#[test]
fn test_is_valid_hair_color() {
    let passport = Passport::new();
    assert!(passport.is_valid_hair_color("#123abc"));
    assert!(!passport.is_valid_hair_color("#123abz"));
    assert!(!passport.is_valid_hair_color("123abc"));
}

#[test]
fn test_is_valid_eye_color() {
    let passport = Passport::new();
    assert!(passport.is_valid_eye_color("brn"));
    assert!(!passport.is_valid_eye_color("wat"));
}

#[test]
fn test_is_valid_passport_id() {
    let passport = Passport::new();
    assert!(passport.is_valid_passport_id("000000001"));
    assert!(!passport.is_valid_passport_id("0123456789"));
}

fn stdin_to_passports() -> Vec<Passport> {
    let mut input_str = String::new();
    stdin().read_to_string(&mut input_str);

    let mut passports: Vec<Passport> = vec![Passport::new()];

    for field_str in input_str.split(|c| c == ' ' || c == '\n') {
        if field_str.trim().is_empty() {
            passports.push(Passport::new());
            continue;
        }

        let mut field_split = field_str.split(':');
        let key = field_split.next().unwrap();
        let value = field_split.next().unwrap();

        let mut passport = passports.last_mut().unwrap();
        passport.fields.insert(key.to_string(), value.to_string());
    }

    println!("{:?}", passports);
    passports
}

pub fn part1() {
    let valid_passport_count = stdin_to_passports()
        .iter()
        .filter(|passport| passport.is_valid_part1())
        .count();

    println!("Valid passport count: {}", valid_passport_count);
}

pub fn part2() {
    let valid_passport_count = stdin_to_passports()
        .iter()
        .filter(|passport| passport.is_valid_part2())
        .count();

    println!("Valid passport count: {}", valid_passport_count);
}
