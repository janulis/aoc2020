use crate::utils;
use utils::VecItem;

pub struct IntVecItem {
    pub value: i32,
}

impl VecItem for IntVecItem {
    fn parse(&mut self, s: &str) {
        match s.parse::<i32>() {
            Ok(value) => self.value = value,
            Err(_error) => println!("Could not parse string {}", s),
        }
    }
}

impl Default for IntVecItem {
    fn default() -> Self {
        IntVecItem { value: 0 }
    }
}

pub fn part1() {
    let numbers: Vec<IntVecItem> = utils::stdin_to_vec::<IntVecItem>();
    for i in 0..numbers.len() {
        let num1 = numbers[i].value;

        for j in (i + 1)..numbers.len() {
            let num2 = numbers[j].value;

            if (num1 + num2) == 2020 {
                println!("{}", num1 * num2);
                return;
            }
        }
    }
}

pub fn part2() {
    let numbers: Vec<IntVecItem> = utils::stdin_to_vec::<IntVecItem>();

    for i in 0..numbers.len() {
        let num1 = numbers[i].value;

        for j in (i + 1)..numbers.len() {
            let num2 = numbers[j].value;

            for k in (j + 1)..numbers.len() {
                let num3 = numbers[k].value;

                if (num1 + num2 + num3) == 2020 {
                    println!("{}", num1 * num2 * num3);
                    return;
                }
            }
        }
    }
}
