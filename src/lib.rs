use std::fs;

pub fn read_input(day: usize) -> String {
    let input_path = format!("inputs/day_{}.txt", day);

    match fs::read_to_string(&input_path) {
        Ok(input) => input,
        Err(err) => panic!("Could not read the input file: {:?}", err),
    }
}
