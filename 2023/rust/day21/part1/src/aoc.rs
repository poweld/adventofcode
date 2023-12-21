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
    // fn set(&mut self, coord: &Coord, val: char) {
    //     if self.is_in_bounds(coord) {
    //         let row = coord.row as usize;
    //         let col = coord.col as usize;
    //         if let Some(existing) = self.0.get_mut(row).and_then(|cols| cols.get_mut(col)) {
    //             *existing = val;
    //         }
    //     } else {
    //         panic!("attempted to set out of bound coord: {coord:?}");
    //     }
    // }
    fn coords(&self) -> Vec<Coord> {
        (0..self.rows()).flat_map(|row| {
            (0..self.cols()).map(move |col| {
                Coord { row: row as i64, col: col as i64 }
            })
        }).collect::<Vec<_>>()
    }
    fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        let neighbors = [
            Coord { row: coord.row - 1, col: coord.col },
            Coord { row: coord.row + 1, col: coord.col },
            Coord { row: coord.row, col: coord.col - 1 },
            Coord { row: coord.row, col: coord.col + 1 },
        ];
        neighbors.into_iter()
            .filter(|coord| self.is_in_bounds(coord))
            .filter(|coord| self.get(&coord).unwrap() != &'#')
            .collect()
    }
    // fn score(&self) -> u64 {
    //     self.coords().into_iter()
    //         .fold(0u64, |acc, coord| {
    //             match self.get(&coord) {
    //                 Some(c) => match c {
    //                     '.' | '|' | '/' | '-' | '\\' => acc,
    //                     _ => acc + 1,
    //                 },
    //                 _ => panic!("invalid coord"),
    //             }
    //         })
    // }
    fn get_start(&self) -> Coord {
        *self.coords().iter().find(|coord| self.get(&coord).unwrap() == &'S').unwrap()
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
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { plane } = parse(&input);
    println!("{plane}\n");
    let start = plane.get_start();
    dbg!(&start);
    let mut evens: u64 = 0;
    let mut odds: u64 = 0;
    let mut seen: HashSet<Coord> = HashSet::new();
    let mut frontier: Vec<Coord> = plane.neighbors(&start);
    let mut steps = 0;
    let steps_desired = 6;
    while !frontier.is_empty() && steps < steps_desired {
        steps += 1;
        if steps % 2 == 0 {
            evens += frontier.len() as u64;
        } else {
            odds += frontier.len() as u64;
        }

        let mut new_frontier = vec![];
        for coord in &frontier {
            for neighbor in plane.neighbors(&coord) {
                if !seen.contains(&neighbor) {
                    new_frontier.push(neighbor);
                    seen.insert(neighbor);
                }
            }
        }
        dbg!(&steps, &evens, &odds, &frontier);
    }
    
    match steps_desired {
        even if even % 2 == 0 => evens.to_string(),
        odd => odds.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 46.to_string());
    }
}
