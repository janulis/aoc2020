use crate::utils;
use std::{fmt::Debug, str::FromStr, string::ParseError};
use utils::stdin_to_vec;

pub struct SlopePattern {
    pub pattern: Vec<char>,
}

impl FromStr for SlopePattern {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pattern: s.chars().collect(),
        })
    }
}

impl Debug for SlopePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formattedPattern = String::new();

        for c in &self.pattern {
            formattedPattern.push(c.clone());
        }

        write!(f, "{}", formattedPattern)
    }
}

impl Clone for SlopePattern {
    fn clone(&self) -> Self {
        Self {
            pattern: self.pattern.clone(),
        }
    }
}

fn print_patterns(patterns: &Vec<SlopePattern>) {
    println!("\nSlope patterns:");
    for p in patterns {
        println!("{:?}", p);
    }
    println!();
}

fn count_trees(
    patterns_template: &Vec<SlopePattern>,
    shift_right_by: usize,
    shift_down_by: usize,
) -> usize {
    let mut patterns = patterns_template.clone();

    let mut horizontal_pos: usize = shift_right_by;
    let mut vertical_pos = shift_down_by;
    let mut tree_count: usize = 0;

    let patterns_len = patterns.len();
    while vertical_pos < patterns_len {
        let p = &mut patterns[vertical_pos];

        while p.pattern.len() < (horizontal_pos + 1) {
            p.pattern
                .append(&mut patterns_template[vertical_pos].pattern.clone());
        }

        if p.pattern[horizontal_pos] == '#' {
            tree_count += 1;
            p.pattern[horizontal_pos] = 'X';
        } else {
            p.pattern[horizontal_pos] = 'O';
        }

        horizontal_pos += shift_right_by;
        vertical_pos += shift_down_by;
    }

    print_patterns(&patterns);

    println!(
        "Right: {}, Down: {}, Trees: {}",
        shift_right_by, shift_down_by, tree_count
    );

    tree_count
}

pub fn part1() {
    let patterns_template: Vec<SlopePattern> = stdin_to_vec::<SlopePattern>();
    count_trees(&patterns_template, 3, 1);
}

pub fn part2() {
    let patterns_template: Vec<SlopePattern> = stdin_to_vec::<SlopePattern>();
    let traverse_rules = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut mult_result = 1;
    for rule in traverse_rules.iter() {
        let tree_count = count_trees(&patterns_template, rule.0, rule.1);
        mult_result *= tree_count;
    }

    println!("Multiplication result: {}", mult_result);
}
