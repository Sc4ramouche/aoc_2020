use aoc_2020::read_input;

pub fn solution_1() {
    let input = read_input(2);
    let mut valid_passwords_count = 0;

    for line in input.lines() {
        let mut iter = line.split(" ");

        let rule = iter.next().unwrap();
        let (min, max) = get_min_max(rule);

        let char_with_colon = iter.next().unwrap();
        let rule_char = char_with_colon.chars().next().unwrap();

        let password = iter.next().unwrap();

        let mut count = 0;
        for char in password.chars() {
            if char == rule_char {
                count += 1;
            }
        }

        if count >= min && count <= max {
            valid_passwords_count += 1;
        }
    }

    println!("There are {} valid passwords", valid_passwords_count);
}

pub fn solution_2() {
    println!("Day 2 part two");
}

pub fn get_min_max(rule: &str) -> (i32, i32) {
    let min_max: Vec<&str> = rule.split("-").take(2).collect();
    let min = min_max[0].parse::<i32>().unwrap();
    let max = min_max[1].parse::<i32>().unwrap();
    (min, max)
}
