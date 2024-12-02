use std::cmp::Ordering;
use std::ops::Range;

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

fn parse(input: &str) -> Vec<Vec<u32>> {
  input.lines()
    .map(|line| {
      line.split_whitespace()
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>()
}

fn delta(val1: &u32, val2: &u32) -> (u32, Ordering) {
  match val1.cmp(&val2) {
    Ordering::Less => (val2 - val1, Ordering::Less),
    Ordering::Greater => (val1 - val2, Ordering::Greater),
    Ordering::Equal => (0, Ordering::Equal),
  }
}

static SAFE_DELTA_RANGE: Range<u32> = 1..4;

fn is_safe(report: &Vec<u32>, ileft: usize, iright: usize, allow_failure: bool, passed_set_ordering: &Option<Ordering>) -> bool {
  if iright < report.len() {
    let (delta, ordering) = delta(&report[ileft], &report[iright]);
    let set_ordering = passed_set_ordering.unwrap_or(ordering);
    (
      ordering == set_ordering &&
      SAFE_DELTA_RANGE.contains(&delta) &&
      is_safe(&report, iright, iright + 1, allow_failure, &Some(set_ordering))
    ) ||
    (
      allow_failure &&
      (
        is_safe(&report, ileft, iright + 1, false, &passed_set_ordering) ||
        (
          ileft == 0 &&
          is_safe(&report, iright, iright + 1, false, &passed_set_ordering)
        )
      )
    )
  } else {
    true
  }
}

fn part1(input: &str) -> u32 {
  let reports = parse(&input);
  reports.into_iter()
    .map(|report| is_safe(&report, 0, 1, false, &None))
    .fold(0, |acc, is_safe| if is_safe { acc + 1 } else { acc })
}

fn part2(input: &str) -> u32 {
  let reports = parse(&input);
  reports.into_iter()
    .map(|report| is_safe(&report, 0, 1, true, &None))
    .fold(0, |acc, is_safe| if is_safe { acc + 1 } else { acc })
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
    assert_eq!(part1(&input), 2);
  }

  #[test]
  fn part2_1() {
    let input = get_test_input();
    assert_eq!(part2(&input), 14);
  }
}
