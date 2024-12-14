// mod grid;
// use crate::grid::*;

// use std::collections::{HashSet, HashMap};

pub fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

fn parse(input: &str) -> Vec<String> {
  input.lines().map(|s| s.to_string()).collect()
}

type AOCResult = usize;
pub const PART1_EXPECTED_RESULT: AOCResult = 0;
pub fn part1(input: &str) -> AOCResult {
  1
}

pub const PART2_EXPECTED_RESULT: AOCResult = 0;
pub fn part2(input: &str) -> AOCResult {
  1
}
