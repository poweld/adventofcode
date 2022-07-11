use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_input(maybe_input_filename: Option<String>) -> std::io::Lines<BufReader<File>> {
    let default_filename = "input.txt".to_string();
    let input_filename = maybe_input_filename.unwrap_or(default_filename);
    let input_file = File::open(input_filename).unwrap();
    let reader = BufReader::new(input_file);

    return reader.lines()
}
