use std::collections::{HashSet, HashMap, VecDeque, BinaryHeap};
use std::hash::Hash;
use std::cmp::Ordering;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CoordDir2(Coord, Option<Direction>, Option<Direction>);
    // dir_minus_one: Option<Direction>,
    // dir_minus_two: Option<Direction>,
impl CoordDir2 {
    fn new(coord: &Coord) -> Self {
        Self(coord.clone(), None, None)
    }
    fn push_dir(&mut self, dir: Direction) {
        self.1 = self.2;
        self.2 = Some(dir);
    }
    fn all_dirs_equal(&self, dir: &Direction) -> bool {
        self.1.is_some() && self.2.is_some() && self.1.unwrap() == *dir && self.2.unwrap() == *dir
    }
}

struct Edge {
    node: usize,
    cost: usize,
    direction: Direction,
}

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
    // fn to_edges(&self) -> Vec<Vec<Edge>> {
    //     self.coords().into_iter()
    //         .map(|coord| {
    //             self.neighbors(&coord).into_iter()
    //                 .map(|neighbor_coorddir| {
    //                     Edge {
    //                         node: (coord.row * self.cols() + coord.col),
    //                         cost: self.get(&neighbor_coorddir.0).unwrap(),
    //                         direction: 
    //                     }
    //                 })
    //                 .collect::<Vec<_>>()
    //         })
    //         .collect::<Vec<_>>()
    // }
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
impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Clone)]
struct Runner {
    coord: Coord,
    direction: Direction,
}

fn reconstruct_path(came_from: &HashMap<CoordDir2, CoordDir2>, current: &CoordDir2) -> Vec<Coord> {
    let mut total_path = VecDeque::from([current]);
    let mut current = current;
    while came_from.contains_key(&current) {
        current = came_from.get(&current).unwrap();
        total_path.push_front(current);
    }
    total_path.into_iter()
        .map(|coorddir2| coorddir2.0)
        .collect::<Vec<_>>()
}

fn astar(start: &Coord, goal: &Coord, plane: &Plane) -> Vec<Coord> {
    let heuristic = |from: &Coord| from.manhattan_distance(goal);
    let mut open_set: HashSet<CoordDir2> = HashSet::from([CoordDir2::new(start)]);
    let mut came_from: HashMap<CoordDir2, CoordDir2> = HashMap::new();
    let mut gscore: HashMap<CoordDir2, u64> = HashMap::from([(CoordDir2::new(start), 0u64)]);
    let mut fscore: HashMap<CoordDir2, u64> = HashMap::from([(CoordDir2::new(start), heuristic(start))]);
    let mut count = 0;
    while !open_set.is_empty() {
        dbg!(&came_from);
        // dbg!(&gscore, &fscore);
        let current = {
            let get_fscore = |coord: &CoordDir2| fscore.get(coord).unwrap_or(&u64::MAX).clone();
            let mut coords: Vec<&CoordDir2> = open_set.iter().collect();
            coords.sort_by(|a, b| get_fscore(a).cmp(&get_fscore(b)));
            coords[0].clone()
        };
        if current.0 == *goal {
            return reconstruct_path(&came_from, &current);
        }
        open_set.remove(&current);
        let current_previous_dir = current.2;
        for CoordDir(neighbor, dir) in plane.neighbors(&current.0) {
            let get_gscore = |coord: &CoordDir2| gscore.get(coord).unwrap_or(&u64::MAX).clone();
            let distance = |current: CoordDir2, neighbor| {
                //dbg!(&current);
                if current.all_dirs_equal(&dir) {
                    // Attempting three moves in a row in the same direction
                    return 9900009999999u64;  // Doesn't seem like this works properly :(
                }
                return *plane.get(&neighbor).unwrap()
            };
            let tentative_gscore = get_gscore(&current) + distance(current.clone(), neighbor);
            let mut neighbor_coorddir2 = CoordDir2::new(&neighbor);
            if let Some(current_previous_dir) = current_previous_dir {
                neighbor_coorddir2.push_dir(current_previous_dir);
            }
            neighbor_coorddir2.push_dir(dir);
            if tentative_gscore < get_gscore(&neighbor_coorddir2) {
                came_from.insert(neighbor_coorddir2.clone(), current.clone());
                gscore.insert(neighbor_coorddir2.clone(), tentative_gscore);
                fscore.insert(neighbor_coorddir2.clone(), tentative_gscore + heuristic(&neighbor));
                if !open_set.contains(&neighbor_coorddir2) {
                    open_set.insert(neighbor_coorddir2);
                }
            }
        }
    }
    panic!("open open set, but goal was never reached");
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct State {
    cost: u64,
    position: Coord,
    direction_count: Option<(Direction, u8)>,
}
// impl Ord for State {
//     fn cmp(&self, other: &Self) -> Ordering {
//         other.cost.cmp(&self.cost)
//             .then_with(|| self.position.cmp(&other.position))
//     }
// }
// impl PartialOrd for State {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// fn dijkstra(start: &Coord, goal: &Coord, plane: &Plane) -> Vec<Coord> {
//     let mut dist: Vec<_> = (0..plane.rows()).map(|_| u64::MAX).collect();
//     let mut heap = BinaryHeap::new();
//     dist[start] = 0;
//     heap.push(State { cost: 0, position: *start, direction_count: None });
//     todo!()
// }

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { plane } = parse(&input);
    // plane.set(&Coord { row: 0, col: 0 }, 0);
    println!("{plane}\n");
    let start = Coord { row: 0, col: 0 };
    // let goal = Coord { row: 0, col: 3 };
    let goal = Coord { row: (plane.rows() as i64) - 1, col: (plane.cols() as i64) - 1 };
    let result = dbg!(astar(&start, &goal, &plane));
    // let result = dbg!(dijkstra(&start, &goal, &plane));
    result.iter()
        .skip(1)
        .map(|coord| plane.get(&coord).unwrap())
        .sum::<u64>()
        .to_string()
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
