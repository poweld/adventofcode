use std::cmp;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
struct ParseResult {
    pipe_matrix: PipeMatrix,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn opposite(&self) -> &Self {
        match self {
            Self::North => &Self::South,
            Self::East => &Self:: West,
            Self::South => &Self::North,
            Self::West => &Self::East,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coord {
    row: usize,
    col: usize,
}

static WESTWARD_CONNECTIONS: [Pipe; 3] = [Pipe::BendNorthEast, Pipe::Horizontal, Pipe::BendSouthEast];
static EASTWARD_CONNECTIONS: [Pipe; 3] = [Pipe::BendNorthWest, Pipe::Horizontal, Pipe::BendSouthWest];
static NORTHWARD_CONECTIONS: [Pipe; 3] = [Pipe::BendSouthWest, Pipe::Vertical, Pipe::BendSouthEast];
static SOUTHWARD_CONNECTIONS: [Pipe; 3] = [Pipe::BendNorthWest, Pipe::Vertical, Pipe::BendNorthEast];

#[derive(Debug, Clone)]
struct PipeMatrix(Vec<Vec<Pipe>>);
impl PipeMatrix {
    fn rows(&self) -> usize {
        self.0.len()
    }
    fn cols(&self) -> usize {
        self.0[0].len()
    }
    fn get(&self, coord: &Coord) -> Option<&Pipe> {
        self.0.get(coord.row).and_then(|row| row.get(coord.col))
    }
    fn set(&mut self, coord: &Coord, pipe: Pipe) {
        if let Some(existing) = self.0.get_mut(coord.row).and_then(|row| row.get_mut(coord.col)) {
            *existing = pipe;
        }
    }
    fn neighbors(&self, coord: &Coord) -> HashMap<Direction, Pipe> {
        let row = coord.row;
        let col = coord.col;
        let row_start = match row {
            0 => 0,
            x => x - 1,
        };
        let col_start = match col {
            0 => 0,
            x => x - 1,
        };
        (row_start..=cmp::min(self.rows(), row + 1))
            .filter(|neighbor_row| neighbor_row != &row)
            .map(|neighbor_row| {
                let coord = Coord { row: neighbor_row, col };
                if neighbor_row < row {
                    (Direction::North, self.get(&coord).unwrap().clone())
                } else {
                    (Direction::South, self.get(&coord).unwrap().clone())
                }
            })
            .chain((col_start..=cmp::min(self.cols(), col + 1))
                .filter(|neighbor_col| neighbor_col != &col)
                .map(|neighbor_col| {
                    let coord = Coord { row, col: neighbor_col };
                    if neighbor_col < col {
                        (Direction::West, self.get(&coord).unwrap().clone())
                    } else {
                        (Direction::East, self.get(&coord).unwrap().clone())
                    }
                })
            ).collect::<HashMap<_, _>>()
    }
    fn neighboring_connections(&self, coord: &Coord) -> HashMap<Direction, Pipe> {
        dbg!(self.neighbors(coord)).into_iter()
            .filter(|(direction, neighbor)| {
                let connections = match direction {
                    Direction::North => &NORTHWARD_CONECTIONS,
                    Direction::East => &EASTWARD_CONNECTIONS,
                    Direction::South => &SOUTHWARD_CONNECTIONS,
                    Direction::West => &WESTWARD_CONNECTIONS,
                };
                connections.contains(neighbor)
            })
            .collect::<HashMap<_, _>>()
    }
    fn start(&self) -> Coord {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let coord = Coord { row, col };
                if let Some(pipe) = self.get(&coord) {
                    if pipe == &Pipe::Start {
                        return coord;
                    }
                }
            }
        }
        panic!("failed to find start")
    }
    fn to_circuit(&self) -> Vec<Coord> {
        let start = self.start();
        dbg!(&start);
        let start_neighboring_connections = self.neighboring_connections(&start);
        let new_start_pipe = Pipe::from_neighboring_connections(&start_neighboring_connections);
        let mut pipe_matrix = self.clone();
        pipe_matrix.set(&start, *new_start_pipe);
        let mut iter = start_neighboring_connections.iter();
        // let mut previous_direction = iter.next().unwrap().0.opposite().clone();
        // let mut current_coord = start;
        let mut previous_direction = match new_start_pipe {
            Pipe::Vertical | Pipe::BendSouthWest | Pipe::BendSouthEast => Direction::North,
            Pipe::BendNorthWest | Pipe::BendNorthEast => Direction::South,
            Pipe::Horizontal => Direction::East,
            _ => panic!(),
        };
        let mut current_coord = match previous_direction.opposite() {
            Direction::North => Coord { row: start.row - 1, col: start.col },
            Direction::East => Coord { row: start.row, col: start.col + 1 },
            Direction::South => Coord { row: start.row + 1, col: start.col },
            Direction::West => Coord { row: start.row, col: start.col - 1 },
        };
        let mut circuit = vec![start];
        loop {
            // dbg!(&current_coord);
            if current_coord == start {
                break;
            }
            circuit.push(current_coord);
            let current_pipe = self.get(&current_coord).unwrap();
            let next_direction = current_pipe.get_other_direction(&previous_direction);
            // // TODO shouldn't be using the neighboring_connections function for anything but figuring out the start
            // let neighboring_connections = pipe_matrix.neighboring_connections(&current_coord);
            // dbg!(&neighboring_connections, &previous_direction);
            // let (next_direction, next_pipe) = neighboring_connections.into_iter()
            //     .find(|(direction, _)| *direction != previous_direction)
            //     .unwrap();
            // dbg!(&next_direction, &next_pipe);
            previous_direction = next_direction.opposite().clone();
            current_coord = match next_direction {
                Direction::North => Coord { row: current_coord.row - 1, col: current_coord.col },
                Direction::East => Coord { row: current_coord.row, col: current_coord.col + 1 },
                Direction::South => Coord { row: current_coord.row + 1, col: current_coord.col },
                Direction::West => Coord { row: current_coord.row, col: current_coord.col - 1 },
            };
        }
        circuit
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Ground,
    Start,
}
impl Pipe {
    fn from_neighboring_connections(neighboring_connections: &HashMap<Direction, Pipe>) -> &Self {
        let connected_to = |direction: &Direction| neighboring_connections.contains_key(direction);
        let panic = || panic!("failed to derive pipe from neighboring_connections: {neighboring_connections:?}");
        // TODO maybe should just do a bunch of .contains() calls since we don't know the order of the directions
        if connected_to(&Direction::North) {
            if connected_to(&Direction::West) {
                &Pipe::BendNorthWest
            } else if connected_to(&Direction::East) {
                &Pipe::BendNorthEast
            } else if connected_to(&Direction::South) {
                &Pipe::Vertical
            } else {
                panic()
            }
        } else if connected_to(&Direction::East) {
            if connected_to(&Direction::West) {
                &Pipe::Horizontal
            } else if connected_to(&Direction::South) {
                &Pipe::BendSouthEast
            } else {
                panic()
            }
        } else if connected_to(&Direction:: South) {
            if connected_to(&Direction::West) {
                &Pipe::BendSouthWest
            } else {
                panic()
            }
        } else {
            panic()
        }
    }
    fn get_other_direction(&self, direction: &Direction) -> &Direction {
        match self {
            Self::Vertical => match direction {
                Direction::North => &Direction::South,
                Direction::South => &Direction::North,
                _ => panic!(),
            },
            Self::Horizontal => match direction {
                Direction::West => &Direction::East,
                Direction::East => &Direction::West,
                _ => panic!(),
            },
            Self::BendNorthEast => match direction {
                Direction::North => &Direction::East,
                Direction::East => &Direction::North,
                _ => panic!(),
            },
            Self::BendNorthWest => match direction {
                Direction::North => &Direction::West,
                Direction::West => &Direction::North,
                _ => panic!(),
            },
            Self::BendSouthEast => match direction {
                Direction::South => &Direction::East,
                Direction::East => &Direction::South,
                _ => panic!(),
            },
            Self::BendSouthWest => match direction {
                Direction::South => &Direction::West,
                Direction::West => &Direction::South,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

fn parse(input: &str) -> ParseResult {
    let pipe_matrix = PipeMatrix(input.lines()
        .map(|line| {
            line.chars().map(|c| match c {
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::BendNorthEast,
                'J' => Pipe::BendNorthWest,
                '7' => Pipe::BendSouthWest,
                'F' => Pipe::BendSouthEast,
                '.' => Pipe::Ground,
                'S' => Pipe::Start,
                _ => todo!(),
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>());
    ParseResult { pipe_matrix }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { pipe_matrix } = parse(&input);
    let circuit = pipe_matrix.to_circuit();
    (circuit.len() / 2).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 4.to_string());
    }
}
