use crate::utils;
use std::{fmt::Debug, str::FromStr, string::ParseError};
use utils::stdin_to_vec;

#[derive(Debug, Clone, PartialEq, Default)]
struct Bag {
    bag_type: String,
    contents: Vec<Bag>,
}

impl Bag {
    pub fn new(bag_type: String, contents: Vec<Bag>) -> Self {
        Self { bag_type, contents }
    }

    pub fn parse(&mut self, rule: &str) {
        let rule = rule
            .replace("bags", "")
            .replace("bag", "")
            .replace("contain", ",")
            .replace("no other", "0")
            .replace(".", "");

        let rule_items: Vec<&str> = rule.split(',').collect();
        self.bag_type = rule_items[0].trim().to_string();

        for i in 1..rule_items.len() {
            let rule_item = rule_items[i].trim();
            if rule_item.trim().is_empty() {
                continue;
            }

            let mut contained_bag_split = rule_item.splitn(2, ' ');
            let contained_bag_count = contained_bag_split
                .next()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            if contained_bag_count == 0 {
                continue;
            }

            let contained_bag_type = contained_bag_split
                .next()
                .unwrap()
                .parse::<String>()
                .unwrap();

            for _ in 0..contained_bag_count {
                self.contents
                    .push(Bag::new(contained_bag_type.to_string(), vec![]));
            }
        }
    }
}

impl FromStr for Bag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instance = Self {
            bag_type: String::new(),
            contents: vec![],
        };

        instance.parse(s);

        Ok(instance)
    }
}

fn find_parents(bags: &Vec<Bag>, contained_bag_type: &str, parent_bags: &mut Vec<Bag>) {
    bags.iter()
        .filter(|bag| {
            bag.contents
                .iter()
                .map(|contained_bag| contained_bag.bag_type.clone())
                .collect::<String>()
                .contains(contained_bag_type)
        })
        .for_each(|bag| {
            if !parent_bags.contains(bag) {
                parent_bags.push(bag.clone());
                find_parents(bags, bag.bag_type.as_str(), parent_bags);
            }
        });
}

fn find_contained_bag_count(bags: &Vec<Bag>, bag_type: &str, contained_bag_count: &mut usize) {
    bags.iter()
        .filter(|bag| bag.bag_type.eq(bag_type))
        .for_each(|bag| {
            *contained_bag_count += bag.contents.len();

            bag.contents.iter().for_each(|contained_bag| {
                find_contained_bag_count(
                    bags,
                    contained_bag.bag_type.as_str(),
                    contained_bag_count,
                );
            });
        });
}

pub fn part1() {
    let bags = stdin_to_vec::<Bag>();
    let contained_bag_type = "shiny gold";
    let mut parent_bags = vec![];

    find_parents(&bags, contained_bag_type, &mut parent_bags);

    println!("Parent bags: {}", parent_bags.len());
}

pub fn part2() {
    let bags = stdin_to_vec::<Bag>();
    let bag_type = "shiny gold";
    let mut contained_bag_count: usize = 0;

    find_contained_bag_count(&bags, bag_type, &mut contained_bag_count);

    println!("Contained bags: {}", contained_bag_count);
}
