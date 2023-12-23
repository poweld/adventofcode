mod my {
    use std::collections::{HashSet, VecDeque};
    use std::hash::Hash;

    type CoordElement = usize;
    #[derive(Debug, PartialEq, Eq, Clone, Hash)]
    pub struct Coord {
        pub row: CoordElement,
        pub col: CoordElement,
    }

    type PlaneChars = Vec<Vec<char>>;
    #[derive(Debug)]
    pub struct Plane {
        chars: PlaneChars,
    }
    impl Plane {
        pub fn start(&self) -> Coord {
            Coord { row: 0, col: 1 }
        }
        pub fn goal(&self) -> Coord {
            let row = self.chars.len() - 1;
            let col = self.chars[0].len() - 2;
            Coord { row, col }
        }
        pub fn get(&self, coord: &Coord) -> char {
            self.chars[coord.row][coord.col]
        }
        pub fn walkable_neighbors(&self, coord: &Coord) -> Vec<Coord> {
            let mut neighbors = vec![
                Coord { row: coord.row + 1, col: coord.col },
                Coord { row: coord.row, col: coord.col + 1 },
            ];
            if let Some(row) = coord.row.checked_sub(1) {
                neighbors.push(Coord { row, col: coord.col } );
            }
            if let Some(col) = coord.col.checked_sub(1) {
                neighbors.push(Coord { row: coord.row, col } );
            }
            neighbors.into_iter()
                .filter(|neighbor| match self.get(&neighbor) {
                    '#' => false,
                    '.' => true,
                    '>' => neighbor != &Coord { row: coord.row, col: coord.col - 1 },
                    'v' => neighbor != &Coord { row: coord.row - 1, col: coord.col },
                    '<' => neighbor != &Coord { row: coord.row, col: coord.col + 1 },
                    '^' => neighbor != &Coord { row: coord.row + 1, col: coord.col },
                    _ => panic!("unexpected map char at: {neighbor:?}"),
                })
                .collect::<Vec<_>>()
        }
    }

    #[derive(Debug)]
    pub struct ParseResult {
        pub plane: Plane,
    }

    pub fn parse(input: &str) -> ParseResult {
        let plane_chars: PlaneChars = input.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let plane = Plane { chars: plane_chars };
        ParseResult { plane }
    }

    #[derive(Debug)]
    struct PathingState {
        path: Vec<Coord>,
        seen: HashSet<Coord>,
        next: Coord,
    }

    pub fn longest_path(start: Coord, goal: Coord, plane: &Plane) -> Vec<Coord> {
        let initial_state = PathingState {
            path: vec![],
            seen: HashSet::from([start.clone()]),
            next: start.clone(),
        };
        let mut to_explore: VecDeque<PathingState> = VecDeque::from([initial_state]);
        let mut completed_paths: Vec<Vec<Coord>> = vec![];
        while to_explore.len() > 0 {
            let PathingState { mut path, mut seen, next: current } = to_explore.pop_front().unwrap();

            if current == goal {
                completed_paths.push(path);
                continue;
            }
            path.push(current.clone());
            seen.insert(current.clone());

            let neighbors = plane.walkable_neighbors(&current).into_iter()
                .filter(|neighbor| !seen.contains(&neighbor))
                .collect::<Vec<_>>();
            if neighbors.len() == 1 {
                // Less cloning around if only one option
                to_explore.push_back(PathingState {
                    path, seen, next: neighbors[0].clone()
                });
            } else {
                for neighbor in neighbors {
                    to_explore.push_back(PathingState {
                        path: path.clone(), seen: seen.clone(), next: neighbor.clone()
                    });
                }
            }
        }
        completed_paths.sort_by(|a, b| b.len().cmp(&a.len()));
        completed_paths[0].clone()
    }
}

use my::*;

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { plane } = parse(&input);
    let start = plane.start();
    let goal = plane.goal();
    longest_path(start, goal, &plane).len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 94.to_string());
    }
}
