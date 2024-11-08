fn main() {
  let input = std::fs::read_to_string("data/input.txt").expect("should have input file"); 
  println!("Part 1: {}", pt1(&input));
  println!("Part 2: {}", pt2(&input));
}

fn pt1(input: &str) -> isize {
  let mut level = 0;
  for c in input.chars() {
    match c {
      '(' => level += 1,
      ')' => level -= 1,
      _ => (),
    }
  }
  level
}

fn pt2(input: &str) -> isize {
  let mut level = 0;
  for (index, c) in input.chars().enumerate() {
    match c {
      '(' => level += 1,
      ')' => level -= 1,
      _ => (),
    }
    if level < 0 {
      // Entered basement
      return (index + 1) as isize;
    }
  }
  panic!("Should have found the \"basement\" before completion")
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_pt1_1() {
    let input = "(())";
    let result = pt1(input);
    assert_eq!(result, 0);
  }

  #[test]
  fn test_pt1_2() {
    let input = "()()";
    let result = pt1(input);
    assert_eq!(result, 0);
  }

  #[test]
  fn test_pt1_3() {
    let input = ")())())";
    let result = pt1(input);
    assert_eq!(result, -3);
  }

  #[test]
  fn test_pt1_3() {
    let input = ")())())";
    let result = pt1(input);
    assert_eq!(result, -3);
  }

  #[test]
  fn test_pt2_1() {
    let input = ")";
    let result = pt2(input);
    assert_eq!(result, 1);
  }

  #[test]
  fn test_pt2_2() {
    let input = "()())";
    let result = pt2(input);
    assert_eq!(result, 5);
  }
}
