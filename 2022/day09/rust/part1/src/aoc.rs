use std::error::Error;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Clone,Copy,Debug)]
struct Vector {
    direction: Direction,
    magnitude: usize,
}
impl Vector {
    fn new(direction: &Direction, magnitude: &usize) -> Self {
        Self { direction: *direction, magnitude: *magnitude }
    }
}
impl Iterator for Vector {
    type Item = Direction;
    fn next(&mut self) -> Option<Self::Item> {
        match self.magnitude {
            n if n > 0 => {
                self.magnitude -= 1;
                Some(self.direction)
            },
            _ => None,
        }
    }
}

#[derive(Clone,Copy,Debug)]
enum Direction {
    Up,
    UpRight,
    Right,
    RightDown,
    Down,
    DownLeft,
    Left,
    LeftUp,
}

#[derive(Copy,Clone,Debug,Eq,Hash,PartialEq)]
struct Coord {
    x: isize,
    y: isize,
}
impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    fn move_to(&mut self, direction: &Direction) {
        let n = 1;
        match direction {
            Direction::Up => {
                self.y += n;
            },
            Direction::UpRight => {
                self.x += n;
                self.y += n;
            },
            Direction::Right => {
                self.x += n;
            },
            Direction::RightDown => {
                self.x += n;
                self.y -= n;
            },
            Direction::Down => {
                self.y -= n;
            },
            Direction::DownLeft => {
                self.x -= n;
                self.y -= n;
            },
            Direction::Left => {
                self.x -= n;
            },
            Direction::LeftUp => {
                self.x -= n;
                self.y += n;
            },
        };
    }
}

#[derive(Copy,Clone,Debug)]
struct Rope {
    max_length: usize,
    head: Coord,
    tail: Coord,
}
impl Rope {
    fn new(max_length: &usize) -> Self {
        let head = Coord::new(0, 0);
        let tail = Coord::new(0, 0);
        Self {
            max_length: *max_length,
            head,
            tail
        }
    }
    fn move_to(&mut self, direction: &Direction) {
        self.head.move_to(&direction);
        let tail_direction = self.tail_direction();
        if let Some(tail_direction) = tail_direction {
            self.tail.move_to(&tail_direction);
        }
    }
    fn tail_direction(&mut self) -> Option<Direction> {
        let (tail_x, tail_y) = (self.tail.x, self.tail.y);
        let (head_x, head_y) = (self.head.x, self.head.y);
        let cmp_x = head_x.cmp(&tail_x);
        let cmp_y = head_y.cmp(&tail_y);
        let diff_x = head_x.abs_diff(tail_x);
        let diff_y = head_y.abs_diff(tail_y);

        match cmp_x {
            Ordering::Less => {
                match cmp_y {
                    Ordering::Less if diff_x >= self.max_length || diff_y >= self.max_length => Some(Direction::DownLeft),
                    Ordering::Equal if diff_x >= self.max_length => Some(Direction::Left),
                    Ordering::Greater if diff_x >= self.max_length || diff_y >= self.max_length => Some(Direction::LeftUp),
                    _ => None,
                }
            },
            Ordering::Equal => {
                match cmp_y {
                    Ordering::Less if diff_y >= self.max_length => Some(Direction::Down),
                    Ordering::Equal => None,
                    Ordering::Greater if diff_y >= self.max_length => Some(Direction::Up),
                    _ => None,
                }
            },
            Ordering::Greater => {
                match cmp_y {
                    Ordering::Less if diff_x >= self.max_length || diff_y >= self.max_length => Some(Direction::RightDown),
                    Ordering::Equal if diff_x >= self.max_length => Some(Direction::Right),
                    Ordering::Greater if diff_x >= self.max_length || diff_y >= self.max_length => Some(Direction::UpRight),
                    _ => None,
                }
            },
        }
    }
}

fn parse(input: String) -> Vec<Vector> {
    input.lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|split| match split[..] {
            ["U", n] => Vector::new(&Direction::Up, &usize::from_str_radix(n, 10).unwrap()),
            ["R", n] => Vector::new(&Direction::Right, &usize::from_str_radix(n, 10).unwrap()),
            ["D", n] => Vector::new(&Direction::Down, &usize::from_str_radix(n, 10).unwrap()),
            ["L", n] => Vector::new(&Direction::Left, &usize::from_str_radix(n, 10).unwrap()),
            _ => panic!(),
        })
        .collect::<Vec<_>>()
}

pub fn solve(input_path: &str) -> Result<String, Box<dyn Error>> {
    let input: String = std::fs::read_to_string(input_path)?;

    let mut rope = Rope::new(&2);
    let vectors = parse(input);
    let directions = vectors.iter()
        .flat_map(|vector| vector.into_iter());
    let tails = directions.map(|direction| {
        rope.move_to(&direction);
        rope.tail.clone()
    }).collect::<HashSet<_>>();
    let result = tails.len().to_string();

    Ok(result)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let result = solve("data/test_input.txt").expect("bad result");
        let solution = 13.to_string();
        assert_eq!(solution, result)
    }
}
