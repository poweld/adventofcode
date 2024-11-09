use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
  let input = std::fs::read_to_string("data/input.txt").expect("should have input file");
  println!("Part 1: {}", pt1(&input));
  println!("Part 2: {}", pt2(&input));
}


#[derive(Debug)]
pub struct Present {
  pub length: usize,
  pub width: usize,
  pub height: usize,
}

impl Present {
  pub fn paper_needed(&self) -> usize {
    let side_areas = [
      self.length * self.width,
      self.width * self.height,
      self.height * self.length,
    ];
    let surface_area = side_areas.iter().sum::<usize>() * 2;
    let smallest_side = side_areas.iter().min().unwrap();
    surface_area + smallest_side
  }
  pub fn ribbon_needed(&self) -> usize {
    let mut dimensions = vec![self.length, self.width, self.height];
    dimensions.sort();
    let wrap_ribbon = (dimensions[0] + dimensions[1]) * 2;
    let bow_ribbon = self.length * self.width * self.height;
    wrap_ribbon + bow_ribbon
  }
}

impl FromStr for Present {
  type Err = ParseIntError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split('x');
    Ok(Present {
      length: split.next().expect("existing length").parse::<usize>().expect("numeric length"),
      width: split.next().expect("existing width").parse::<usize>().expect("numeric width"),
      height: split.next().expect("existing height").parse::<usize>().expect("numeric height"),
    })
  }
}

fn pt1(input: &str) -> usize {
  input.lines().fold(0, |acc, line| {
    let present = Present::from_str(line);
    acc + present.map_or(0, |present| present.paper_needed())
  })
}

fn pt2(input: &str) -> usize {
  input.lines().fold(0, |acc, line| {
    let present = Present::from_str(line);
    acc + present.map_or(0, |present| present.ribbon_needed())
  })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_pt1_1() {
    let input = "2x3x4";
    let result = pt1(input);
    assert_eq!(result, 58);
  }

  #[test]
  fn test_pt2_1() {
    let input = "2x3x4";
    let result = pt2(input);
    assert_eq!(result, 34);
  }
}
