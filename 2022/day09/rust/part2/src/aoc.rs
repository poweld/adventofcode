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

#[derive(Debug)]
struct Rope {
    knots: Vec<Coord>,
}
impl Rope {
    fn new(num_knots: usize) -> Self {
        let mut knots = vec![];
        for _ in 0..num_knots {
            knots.push(Coord::new(0, 0));
        }
        Self { knots }
    }
    fn move_to(&mut self, direction: &Direction) {
        self.knots.get_mut(0).unwrap().move_to(&direction);
        for knot_index in 1..self.knots.len() {
            let tail = self.knots.get(knot_index).unwrap();
            let head = self.knots.get(knot_index - 1).unwrap();
            let knot_direction = self.knot_direction(&head, &tail);
            if let Some(knot_direction) = knot_direction {
                let tail = self.knots.get_mut(knot_index).unwrap();
                tail.move_to(&knot_direction);
            }
        }
    }
    fn knot_direction(&self, head: &Coord, tail: &Coord) -> Option<Direction> {
        static MAX_DISTANCE: usize = 1;
        let (tail_x, tail_y) = (tail.x, tail.y);
        let (head_x, head_y) = (head.x, head.y);
        let cmp_x = head_x.cmp(&tail_x);
        let cmp_y = head_y.cmp(&tail_y);
        let diff_x = head_x.abs_diff(tail_x);
        let diff_y = head_y.abs_diff(tail_y);

        match cmp_x {
            Ordering::Less => {
                match cmp_y {
                    Ordering::Less if diff_x > MAX_DISTANCE || diff_y > MAX_DISTANCE => Some(Direction::DownLeft),
                    Ordering::Equal if diff_x > MAX_DISTANCE => Some(Direction::Left),
                    Ordering::Greater if diff_x > MAX_DISTANCE || diff_y > MAX_DISTANCE => Some(Direction::LeftUp),
                    _ => None,
                }
            },
            Ordering::Equal => {
                match cmp_y {
                    Ordering::Less if diff_y > MAX_DISTANCE => Some(Direction::Down),
                    Ordering::Equal => None,
                    Ordering::Greater if diff_y > MAX_DISTANCE => Some(Direction::Up),
                    _ => None,
                }
            },
            Ordering::Greater => {
                match cmp_y {
                    Ordering::Less if diff_x > MAX_DISTANCE || diff_y > MAX_DISTANCE => Some(Direction::RightDown),
                    Ordering::Equal if diff_x > MAX_DISTANCE => Some(Direction::Right),
                    Ordering::Greater if diff_x > MAX_DISTANCE || diff_y > MAX_DISTANCE => Some(Direction::UpRight),
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
            _ => panic!("invalid entry: {split:?}"),
        })
        .collect::<Vec<_>>()
}

pub fn solve(input_path: &str) -> Result<String, Box<dyn Error>> {
    let input: String = std::fs::read_to_string(input_path)?;

    let mut rope = Rope::new(10);
    let vectors = parse(input);
    let directions = vectors.iter()
        .flat_map(|vector| vector.into_iter());
    let tails = directions.map(|direction| {
        rope.move_to(&direction);
        rope.knots.last().unwrap().clone()
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
        let solution = 36.to_string();
        assert_eq!(result, solution)
    }
}
