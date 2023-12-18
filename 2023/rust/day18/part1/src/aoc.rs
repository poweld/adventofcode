use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    row: i64,
    col: i64,
}

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
    fn coords(&self) -> Vec<Coord> {
        (0..self.rows()).flat_map(|row| {
            (0..self.cols()).map(move |col| {
                Coord { row: row as i64, col: col as i64 }
            })
        }).collect::<Vec<_>>()
    }
    fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        vec![
            Coord { row: coord.row - 1, col: coord.col },
            Coord { row: coord.row + 1, col: coord.col },
            Coord { row: coord.row, col: coord.col - 1 },
            Coord { row: coord.row, col: coord.col + 1 },
        ]
    }
    fn new(rows: usize, cols: usize, val: char) -> Self {
        Self(
            std::iter::once([val].repeat(cols))
                .cycle()
                .take(rows)
                .collect::<Vec<_>>()
        )
    }
    fn inside(&self, coord: &Coord) -> bool {
        if !self.is_in_bounds(&coord) || self.get(&coord) != Some(&'.') {
            return false;
        }
        return true;
        //// seed cheating
        // let cols = &self.0[coord.row as usize];
        // let walls_to_left_count = cols.iter()
        //     .take((coord.col) as usize)
        //     .filter(|c| c == &&'#')
        //     .collect::<Vec<_>>()
        //     .len();
        // dbg!(&walls_to_left_count);
        // walls_to_left_count % 2 == 1
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: usize,
    color: String,
}

#[derive(Debug)]
struct ParseResult {
    instructions: Vec<Instruction>,
}

fn parse(input: &str) -> ParseResult {
    let instructions: Vec<Instruction> = input.lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            let direction = match split.next().unwrap() {
                "U" => Direction::North,
                "R" => Direction::East,
                "D" => Direction::South,
                "L" => Direction::West,
                _ => panic!(),
            };
            let distance = split.next().unwrap().parse::<usize>().unwrap();
            let color = split.next().unwrap()
                .chars()
                .skip(2)
                .take(6)
                .collect::<String>();
            Instruction { direction, distance, color }
        })
        .collect();
    ParseResult { instructions }
}

fn instructions_to_plane(instructions: &Vec<Instruction>) -> Plane {
    let mut row: i64 = 0;
    let mut col: i64 = 0;
    let mut min_row: i64 = i64::MAX;
    let mut min_col: i64 = i64::MAX;
    let mut max_row: i64 = i64::MIN;
    let mut max_col: i64 = i64::MIN;
    let mut pre_coords: Vec<Coord> = vec![];
    for instruction in instructions {
        match instruction.direction {
            Direction::North => {
                for _ in 0..instruction.distance {
                    row -= 1;
                    pre_coords.push(Coord { row, col });
                }
            },
            Direction::East => {
                for _ in 0..instruction.distance {
                    col += 1;
                    pre_coords.push(Coord { row, col });
                }
            },
            Direction::South => {
                for _ in 0..instruction.distance {
                    row += 1;
                    pre_coords.push(Coord { row, col });
                }
            },
            Direction::West => {
                for _ in 0..instruction.distance {
                    col -= 1;
                    pre_coords.push(Coord { row, col });
                }
            },
        }
        min_row = std::cmp::min(row, min_row);
        min_col = std::cmp::min(col, min_col);
        max_row = std::cmp::max(row, max_row);
        max_col = std::cmp::max(col, max_col);
    }
    let rows = (max_row - min_row + 1) as usize;
    let cols = (max_col - min_col + 1) as usize;
    let mut plane = Plane::new(rows, cols, '.');
    for pre_coord in pre_coords {
        let coord = Coord { row: pre_coord.row - min_row, col: pre_coord.col - min_col };
        plane.set(&coord, '#');
    }
    plane
}

// Seed cheating
pub fn solve(input_path: &str, seed_row: i64, seed_col: i64) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult{ instructions } = parse(&input);
    let mut plane = instructions_to_plane(&instructions);
    println!("{plane}\n");

    //// seed cheating
    // let coord = Coord { row: 0, col: 0 };
    // let seed = plane.coords()
    //     .into_iter()
    //     .find(|coord| plane.inside(&coord))
    //     .unwrap();
    let seed = Coord { row: seed_row, col: seed_col };
    dbg!(&seed);

    let mut queue: VecDeque<Coord> = VecDeque::from([seed]);
    while !queue.is_empty() {
        let coord = queue.pop_front().unwrap();
        if plane.inside(&coord) {
            plane.set(&coord, 'X');
            for neighbor in plane.neighbors(&coord) {
                queue.push_back(neighbor);
            }
        }
    }

    println!("{plane}\n");

    plane.coords().iter()
        .filter(|coord| plane.get(&coord) != Some(&'.'))
        .collect::<Vec<_>>()
        .len()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt", 1, 1);
        println!("result: {result}");
        assert_eq!(result, 62.to_string());
    }
}
