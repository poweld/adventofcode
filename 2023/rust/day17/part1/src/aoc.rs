use std::collections::{HashSet, HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    row: i64,
    col: i64,
}
impl Coord {
    fn manhattan_distance(&self, to: &Coord) -> u64 {
        self.row.abs_diff(to.row) + self.col.abs_diff(to.col)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct CoordDir(Coord, Direction);

#[derive(Debug, Clone)]
struct Plane(Vec<Vec<u64>>);
impl std::fmt::Display for Plane {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = self.0.iter().map(|digits| digits.iter()
            .map(|digit| char::from_digit(*digit as u32, 10).expect("failed to parse digit: {digit}"))
            .collect::<String>()).collect::<Vec<_>>().join("\n");
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
    fn neighbors(&self, coord: &Coord) -> Vec<CoordDir> {
        let neighbors = vec![
            CoordDir(Coord { row: coord.row - 1, col: coord.col }, Direction::North),
            CoordDir(Coord { row: coord.row + 1, col: coord.col }, Direction::South),
            CoordDir(Coord { row: coord.row, col: coord.col - 1 }, Direction::West),
            CoordDir(Coord { row: coord.row, col: coord.col + 1 }, Direction::East),
        ];
        neighbors.into_iter().filter(|cd| self.is_in_bounds(&cd.0)).collect()
    }
    fn get(&self, coord: &Coord) -> Option<&u64> {
        let row = coord.row as usize;
        let col = coord.col as usize;
        if self.is_in_bounds(coord) {
            self.0.get(row).and_then(|cols| cols.get(col))
        } else {
            None
        }
    }
    fn set(&mut self, coord: &Coord, val: u64) {
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
}

#[derive(Debug)]
struct ParseResult {
    plane: Plane,
}

fn parse(input: &str) -> ParseResult {
    let plane_data: Vec<Vec<u64>> = input.lines()
        .map(|line| line.chars().map(|c| (c.to_digit(10).unwrap()) as u64).collect::<Vec<_>>())
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

#[derive(Debug, Clone)]
struct Runner {
    coord: Coord,
    direction: Direction,
}

fn reconstruct_path(came_from: &HashMap<Coord, Coord>, current: &Coord) -> Vec<Coord> {
    let mut total_path = VecDeque::from([*current]);
    let mut current = current;
    while came_from.contains_key(&current) {
        current = came_from.get(&current).unwrap();
        total_path.push_front(*current);
    }
    Vec::from(total_path)
}
fn astar(start: &Coord, goal: &Coord, plane: &Plane) -> Vec<Coord> {
    let h = |from: &Coord| from.manhattan_distance(goal);
    let mut open_set: HashSet<Coord> = HashSet::from([*start]);
    let mut came_from: HashMap<Coord, Coord> = HashMap::new();
    let mut gscore: HashMap<Coord, u64> = HashMap::from([(*start, 0u64)]);
    let mut fscore: HashMap<Coord, u64> = HashMap::from([(*start, h(start))]);
    let mut count = 0;
    while !open_set.is_empty() {
        let current = {
            let get_fscore = |coord: &Coord| fscore.get(coord).unwrap_or(&u64::MAX).clone();
            let mut coords: Vec<&Coord> = open_set.iter().collect();
            coords.sort_by(|a, b| get_fscore(a).cmp(&get_fscore(b)));
            coords[0].clone()
        };
        if current == *goal {
            return reconstruct_path(&came_from, &current);
        }
        open_set.remove(&current);
        for CoordDir(neighbor, direction) in plane.neighbors(&current) {
            let get_gscore = |coord: &Coord| gscore.get(coord).unwrap_or(&u64::MAX).clone();
            let d = |current, neighbor| plane.get(&neighbor).unwrap();
            let tentative_gscore = get_gscore(&current) + d(current, neighbor);
            if tentative_gscore < get_gscore(&neighbor) {
                came_from.insert(neighbor, current);
                gscore.insert(neighbor, tentative_gscore);
                fscore.insert(neighbor, tentative_gscore + h(&neighbor));
                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                }
            }
        }
    }
    panic!("open open set, but goal was never reached");
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { plane } = parse(&input);
    println!("{plane}\n");
    let start = Coord { row: 0, col: 0 };
    let goal = Coord { row: (plane.rows() as i64) - 1, col: (plane.cols() as i64) - 1 };
    let result = dbg!(astar(&start, &goal, &plane));
    result.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 102.to_string());
    }
}
