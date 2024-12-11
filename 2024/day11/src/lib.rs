use std::collections::HashMap;

pub fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

fn parse(input: &str) -> Vec<u64> {
  input.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>()
}

fn blink(rock: &u64) -> Vec<u64> {
  match rock {
    0 => vec![1],
    even if even.to_string().len() % 2 == 0 => {
      let even = even.to_string();
      let split_index = even.len() / 2;
      let left = even[..split_index].parse().unwrap();
      let right = even[split_index..].parse().unwrap();
      vec![left, right]
    },
    s => vec![s * 2024],
  }
}

pub fn part1(input: &str) -> u64 {
  let mut rocks = parse(&input);
  for _ in 0..25 {
    rocks = rocks.into_iter().flat_map(|rock| blink(&rock))
    .collect::<Vec<_>>()
  }
  rocks.len().try_into().unwrap()
}

pub fn part2(input: &str) -> u64 {
  let rocks = parse(&input);
  let mut rock_counter = HashMap::new();
  for rock in rocks {
    rock_counter.entry(rock)
      .and_modify(|count| *count += 1)
      .or_insert(1);
  }
  for _ in 0..75 {
    let mut new_rock_counter = HashMap::new();
    for (rock, count) in rock_counter.iter() {
      let mut update_count = |rock| {
        new_rock_counter.entry(rock)
          .and_modify(|counter| *counter += *count)
          .or_insert(*count);
      };
      match rock {
        0 => {
          let new_rock = 1;
          update_count(new_rock);
        },
        even if even.to_string().len() % 2 == 0 => {
          let even = even.to_string();
          let split_index = even.len() / 2;
          let new_rock_left = even[..split_index].parse().unwrap();
          let new_rock_right = even[split_index..].parse().unwrap();
          update_count(new_rock_left);
          update_count(new_rock_right);
        },
        rock => {
          let new_rock = rock * 2024;
          update_count(new_rock);
        },
      }
    }
    rock_counter = new_rock_counter;
  }
  rock_counter.values().sum()
}
