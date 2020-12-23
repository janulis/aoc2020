use crate::utils;

pub fn part1() {
    let numbers = utils::stdin_to_vec::<i32>();
    for i in 0..numbers.len() {
        let num1 = numbers[i];

        for j in (i + 1)..numbers.len() {
            let num2 = numbers[j];

            if (num1 + num2) == 2020 {
                println!("{}", num1 * num2);
                return;
            }
        }
    }
}

pub fn part2() {
    let numbers = utils::stdin_to_vec::<i32>();

    for i in 0..numbers.len() {
        let num1 = numbers[i];

        for j in (i + 1)..numbers.len() {
            let num2 = numbers[j];

            for k in (j + 1)..numbers.len() {
                let num3 = numbers[k];

                if (num1 + num2 + num3) == 2020 {
                    println!("{}", num1 * num2 * num3);
                    return;
                }
            }
        }
    }
}
