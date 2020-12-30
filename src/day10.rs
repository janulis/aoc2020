use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use crate::utils;

const MAX_JOLT_DIFF: i64 = 3;

fn get_sorted_jolts() -> Vec<i64> {
    let mut jolts = utils::stdin_to_vec::<i64>();

    jolts.sort();

    let outlet_jolt_num = 0;
    let device_adapter_jolt_num = jolts.last().unwrap() + MAX_JOLT_DIFF;

    jolts.insert(0, outlet_jolt_num);
    jolts.push(device_adapter_jolt_num);

    jolts
}

pub fn part1() {
    let jolts = get_sorted_jolts();
    let mut jolts_diff_distribution = HashMap::<i64, i64>::new();

    for i in 0..jolts.len() - 1 {
        let diff = jolts[i + 1] - jolts[i];

        let diff_count = jolts_diff_distribution.entry(diff).or_insert(0);
        *diff_count += 1;
    }

    println!("Jolts diff distribution, {:?}", jolts_diff_distribution);
    let result = jolts_diff_distribution[&1] * jolts_diff_distribution[&3];
    println!("Result: {}", result);
}

fn get_vec_hash(vec: &Vec<i64>) -> u64 {
    let mut hasher = DefaultHasher::new();
    Hash::hash_slice(&vec, &mut hasher);
    hasher.finish()
}

fn find_arrangements(jolts: &Vec<i64>, arrangements: &mut HashMap<u64, Vec<i64>>) {
    if arrangements.is_empty() {
        arrangements.insert(get_vec_hash(jolts), jolts.clone());
    }

    for i in 1..jolts.len() - 1 {
        if (jolts[i + 1] - jolts[i - 1]) <= MAX_JOLT_DIFF {
            let mut arrangement = jolts.clone();
            arrangement.remove(i);

            let arrangement_hash = get_vec_hash(&arrangement);

            if !arrangements.contains_key(&arrangement_hash) {
                find_arrangements(&arrangement, arrangements);
                arrangements.insert(arrangement_hash, arrangement);
            }
        }
    }
}

fn find_arrangement_count(jolts: &Vec<i64>) -> usize {
    println!("{:?}", jolts);
    let mut arrangement_count = 1;

    let mut i = 0;
    while i < jolts.len() {
        let mut valid_sequence_len = 0;

        for j in (i + 1)..jolts.len() {
            let jolt_diff = jolts[j] - jolts[i];

            if jolt_diff > MAX_JOLT_DIFF || j == (jolts.len() - 1) {
                let mut arrangements: HashMap<u64, Vec<i64>> = HashMap::new();
                find_arrangements(&jolts[i..=j].to_vec(), &mut arrangements);

                arrangement_count *= arrangements.len();
                break;
            }

            valid_sequence_len += 1;
        }

        i += if valid_sequence_len > 0 {
            valid_sequence_len
        } else {
            1
        };
    }

    arrangement_count
}

pub fn part2() {
    let jolts = get_sorted_jolts();
    println!("Arrangement count: {}", find_arrangement_count(&jolts));
}
