use std::cmp::Ordering;
use std::collections::HashMap;

fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

fn main() {
  let input = read_to_string("input/input.txt");
  println!("Part1: {}", part1(&input));
  println!("Part2: {}", part2(&input));
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
  input.lines()
    .map(|line| {
      let mut line_nums = line.split_whitespace()
        .map(|num_str| num_str.parse::<u32>().unwrap());
      (line_nums.next().unwrap(), line_nums.next().unwrap())
    })
    .unzip()
}

fn delta((val1, val2): (u32, u32)) -> u32 {
  match val1.cmp(&val2) {
    Ordering::Less => val2 - val1,
    Ordering::Greater => val1 - val2,
    Ordering::Equal => 0,
  }
}

fn part1(input: &str) -> u32 {
  let (mut list1, mut list2) = parse(&input);
  list1.sort();
  list2.sort();
  list1.into_iter()
    .zip(list2.into_iter())
    .fold(0, |acc, values| acc + delta(values))
}

fn part2(input: &str) -> u32 {
  let (list1, list2) = parse(&input);
    let mut counter: HashMap<u32, u32> = HashMap::new();
    for list2_val in list2 {
      counter.entry(list2_val)
        .and_modify(|count| *count += 1)
        .or_insert(1);
    }
    list1.into_iter()
      .map(|element| {
        let occurrences = counter.get(&element).unwrap_or(&0);
        element * occurrences
      })
      .sum()
}


#[cfg(test)]
mod tests {
  use super::*;

  fn get_test_input() -> String {
    read_to_string("input/test_input.txt")
  }

  #[test]
  fn part1_1() {
    let input = get_test_input();
    assert_eq!(part1(&input), 11);
  }

  #[test]
  fn part2_1() {
    let input = get_test_input();
    assert_eq!(part2(&input), 31);
  }
}
