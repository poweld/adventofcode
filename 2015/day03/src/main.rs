use std::collections::HashSet;

fn main() {
  let input = std::fs::read_to_string("data/input.txt").expect("should have input file");
  println!("Part 1: {}", pt1(&input));
  println!("Part 2: {}", pt2(&input));
}


#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Position {
  pub x: isize,
  pub y: isize,
}

impl Position {
  pub fn up(&self) -> Position {
    Position {
      x: self.x,
      y: self.y + 1,
    }
  }
  pub fn right(&self) -> Position {
    Position {
      x: self.x + 1,
      y: self.y,
    }
  }
  pub fn down(&self) -> Position {
    Position {
      x: self.x,
      y: self.y - 1,
    }
  }
  pub fn left(&self) -> Position {
    Position {
      x: self.x - 1,
      y: self.y,
    }
  }
}

fn pt1(input: &str) -> usize {
  let mut position = Position { x: 0, y: 0 };
  let mut visited = HashSet::new();
  visited.insert(position.clone());
  for c in input.chars() {
    match c {
      '^' => position = position.up(),
      '>' => position = position.right(),
      'v' => position = position.down(),
      '<' => position = position.left(),
      _ => (),
    }
    visited.insert(position.clone());
  }
  visited.len()
}

fn pt2(input: &str) -> usize {
  let mut santa_position = Position { x: 0, y: 0 };
  let mut robo_santa_position = Position { x: 0, y: 0 };
  let mut visited = HashSet::new();
  visited.insert(santa_position.clone());
  let mut santa_turn = true;
  for c in input.chars() {
    if santa_turn {
      match c {
        '^' => santa_position = santa_position.up(),
        '>' => santa_position = santa_position.right(),
        'v' => santa_position = santa_position.down(),
        '<' => santa_position = santa_position.left(),
        _ => (),
      }
      visited.insert(santa_position.clone());
    } else {
      match c {
        '^' => robo_santa_position = robo_santa_position.up(),
        '>' => robo_santa_position = robo_santa_position.right(),
        'v' => robo_santa_position = robo_santa_position.down(),
        '<' => robo_santa_position = robo_santa_position.left(),
        _ => (),
      }
      visited.insert(robo_santa_position.clone());
    }
    santa_turn = !santa_turn;
  }
  visited.len()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_pt1_1() {
    let input = ">";
    let result = pt1(input);
    assert_eq!(result, 2);
  }

  #[test]
  fn test_pt1_2() {
    let input = "^>v<";
    let result = pt1(input);
    assert_eq!(result, 4);
  }

  #[test]
  fn test_pt1_3() {
    let input = "^v^v^v^v^v";
    let result = pt1(input);
    assert_eq!(result, 2);
  }

  #[test]
  fn test_pt2_1() {
    let input = "^v";
    let result = pt2(input);
    assert_eq!(result, 3);
  }

  #[test]
  fn test_pt2_2() {
    let input = "^>v<";
    let result = pt2(input);
    assert_eq!(result, 3);
  }

  #[test]
  fn test_pt2_3() {
    let input = "^v^v^v^v^v";
    let result = pt2(input);
    assert_eq!(result, 11);
  }
}
