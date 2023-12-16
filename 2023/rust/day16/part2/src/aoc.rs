use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    row: i64,
    col: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct CoordDir(Coord, Direction);

#[derive(Debug, Clone)]
struct Plane(Vec<Vec<char>>);
impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self.0.iter().map(|chars| chars.iter().collect::<String>()).collect::<Vec<_>>().join("\n");
        write!(f, "{}", s)
    }
}
impl Plane {
    fn rows(&self) -> usize {
        self.0.len()
    }
    fn cols(&self) -> usize {
        self.0[0].len()
    }
    fn is_in_bounds(&self, coord: &Coord) -> bool {
        coord.row >= 0 &&
        coord.col >= 0 &&
        (coord.row as usize) < self.rows() &&
        (coord.col as usize) < self.cols()
    }
    fn get(&self, coord: &Coord) -> Option<&char> {
        let row = coord.row as usize;
        let col = coord.col as usize;
        if self.is_in_bounds(coord) {
            self.0.get(row).and_then(|cols| cols.get(col))
        } else {
            None
        }
    }
    fn set(&mut self, coord: &Coord, val: char) {
        if self.is_in_bounds(coord) {
            let row = coord.row as usize;
            let col = coord.col as usize;
            if let Some(existing) = self.0.get_mut(row).and_then(|cols| cols.get_mut(col)) {
                *existing = val;
            }
        } else {
            panic!("attempted to set out of bound coord: {coord:?}");
        }
    }
    fn visit_by(&mut self, runner: &Runner) {
        match self.get(&runner.coord) {
            Some(c) => {
                match c {
                    '.' => self.set(&runner.coord, runner.direction.to_char()),
                    '^' | '>' | 'v' | '<' => self.set(&runner.coord, '2'),
                    c => {
                        let digit = c.to_digit(10);
                        if let Some(digit) = digit {
                            if let Some(new_digit) = char::from_digit(digit + 1, 10) {
                                self.set(&runner.coord, new_digit);
                            }
                        }
                    },
                }
            },
            None => (),
        }
    }
    fn coords(&self) -> Vec<Coord> {
        (0..self.rows()).flat_map(|row| {
            (0..self.cols()).map(move |col| {
                Coord { row: row as i64, col: col as i64 }
            })
        }).collect::<Vec<_>>()
    }
    fn score(&self) -> u64 {
        self.coords().into_iter()
            .fold(0u64, |acc, coord| {
                match self.get(&coord) {
                    Some(c) => match c {
                        '.' | '|' | '/' | '-' | '\\' => acc,
                        _ => acc + 1,
                    },
                    _ => panic!("invalid coord"),
                }
            })
    }
}

#[derive(Debug)]
struct ParseResult {
    plane: Plane,
}

fn parse(input: &str) -> ParseResult {
    let plane_data: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    ParseResult { plane: Plane(plane_data) }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Self::North => '^',
            Self::East => '>',
            Self::South => 'v',
            Self::West => '<',
        }
    }
    fn reflect(self, mirror: char) -> Vec<Self> {
        match mirror {
            '/' => match self {
                Self::North => vec![Self::East],
                Self::East => vec![Self::North],
                Self::South => vec![Self::West],
                Self::West => vec![Self::South],
            },
            '\\' => self.reflect('/').into_iter().map(|dir| dir.opposite()).collect(),
            '-' => match self {
                Self::East | Self::West => vec![self],
                Self::North | Self::South => {
                    vec![Self::East, Self::West]
                }
            },
            '|' => match self {
                Self::North | Self::South => vec![self],
                Self::East | Self::West => {
                    vec![Self::North, Self::South]
                }
            },
            _ => vec![self],
        }
    }
}

#[derive(Debug, Clone)]
struct Runner {
    coord: Coord,
    direction: Direction,
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { plane } = parse(&input);
    let northmost_row_coorddirs = (0..plane.cols()).map(|x| x as i64).map(|col| CoordDir(Coord { row: 0, col }, Direction::South));
    let southmost_row_coorddirs = (0..plane.cols()).map(|x| x as i64).map(|col| CoordDir(Coord { row: (plane.rows() as i64) - 1, col }, Direction::North));
    let westmost_col_coorddirs = (0..plane.rows()).map(|x| x as i64).map(|row| CoordDir(Coord { row, col: 0 }, Direction::East));
    let eastmost_col_coorddirs = (0..plane.rows()).map(|x| x as i64).map(|row| CoordDir(Coord { row, col: (plane.cols() as i64) - 1 }, Direction::West));
    let init_coords = northmost_row_coorddirs.chain(southmost_row_coorddirs).chain(westmost_col_coorddirs).chain(eastmost_col_coorddirs);
    init_coords.map(|init_coorddir| {
        let mut plane = plane.clone();
        let init_coord = init_coorddir.0;
        let init_direction = init_coorddir.1;
        let mut runners = vec![Runner { coord: init_coord.clone(), direction: init_direction }];
        let mut seen_from_direction: HashSet<CoordDir> = HashSet::new();
        let mut visited: HashSet<Coord> = HashSet::new();

        while runners.len() > 0 {
            // dbg!(&runners);
            let runner = runners.pop().unwrap();
            let coorddir = CoordDir(runner.coord, runner.direction);
            if seen_from_direction.contains(&coorddir) {
                continue;
            }
            seen_from_direction.insert(coorddir);
            plane.visit_by(&runner);
            // println!("\n{plane}\n");
            let at_location = plane.get(&runner.coord);
            if let Some(at_location) = at_location {
                visited.insert(runner.coord);
                let new_directions = runner.clone().direction.reflect(*at_location);
                // dbg!(&new_directions);
                for new_direction in new_directions {
                    let new_coord = match new_direction {
                        Direction::North => Coord { row: runner.coord.row - 1, col: runner.coord.col },
                        Direction::East => Coord { row: runner.coord.row, col: runner.coord.col + 1 },
                        Direction::South => Coord { row: runner.coord.row + 1, col: runner.coord.col },
                        Direction::West => Coord { row: runner.coord.row, col: runner.coord.col - 1 },
                    };
                    let mut new_runner = runner.clone();
                    new_runner.coord = new_coord;
                    new_runner.direction = new_direction;
                    runners.push(new_runner);
                }
            }
        }

        visited.len()
    })
    .max().unwrap()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 51.to_string());
    }
}
