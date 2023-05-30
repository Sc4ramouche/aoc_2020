pub mod solutions;

use solutions::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a day, i.e.: ./aoc_2020 day_7");
    }

    let day = &args[1];

    match day.as_str() {
        "day_1" => day_1::solution_1(),
        "day_1_2" => day_1::solution_2(),
        "day_2" => day_2::solution_1(),
        "day_2_2" => day_2::solution_2(),
        _ => println!("No day matched"),
    }
}
