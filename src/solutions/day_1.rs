use std::collections::HashSet;

use aoc_2020::read_input;

pub fn solution_1() {
    let input = read_input(1);

    if let Some((x, y)) = two_sum(&input, 2020) {
        println!("{}", x * y);
    }
}

pub fn solution_2() {
    let input = read_input(1);

    for line in input.lines() {
        if let Ok(number) = line.parse::<i32>() {
            let counterpart = 2020 - number;

            if let Some((x, y)) = two_sum(&input, counterpart) {
                println!("{}", x * y * number);
                break;
            }
        }
    }
}

fn two_sum(input: &String, target: i32) -> Option<(i32, i32)> {
    let mut set: HashSet<i32> = HashSet::new();

    for line in input.lines() {
        if let Ok(number) = line.parse::<i32>() {
            let counterpart = target - number;

            if set.contains(&counterpart) {
                return Some((number, counterpart));
            }

            set.insert(number);
        }
    }

    return None;
}
