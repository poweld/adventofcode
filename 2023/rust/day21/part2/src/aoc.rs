use std::collections::HashSet;
use std::hash::Hash;

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
    fn normalize(&self, coord: &Coord) -> Coord {
        let row = match coord.row % (self.rows() as i64) {
            row if row < 0 => row + (self.rows() as i64),
            row => row % self.rows() as i64,
        };
        let col = match coord.col % (self.cols() as i64) {
            col if col < 0 => col + (self.cols() as i64),
            col => col % self.cols() as i64,
        };
        Coord { row, col }
    }
    fn get(&self, coord: &Coord) -> Option<&char> {
        let coord = self.normalize(coord);
        let row = coord.row as usize;
        let col = coord.col as usize;
        self.0.get(row).and_then(|cols| cols.get(col))
    }
    fn coords(&self) -> Vec<Coord> {
        (0..self.rows()).flat_map(|row| {
            (0..self.cols()).map(move |col| {
                Coord { row: row as i64, col: col as i64 }
            })
        }).collect::<Vec<_>>()
    }
    fn open_coords(&self) -> Vec<Coord> {
        self.coords().into_iter()
            .filter(|coord| self.get(&coord) != Some(&'#'))
            .collect()
    }
    fn neighbors(&self, coord: &Coord) -> Vec<Coord> {
        let neighbors = [
            Coord { row: coord.row - 1, col: coord.col },
            Coord { row: coord.row + 1, col: coord.col },
            Coord { row: coord.row, col: coord.col - 1 },
            Coord { row: coord.row, col: coord.col + 1 },
        ];
        neighbors.into_iter()
            .filter(|coord| self.get(&coord).unwrap() != &'#')
            .collect()
    }
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

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { plane } = parse(&input);
    let start = plane.get_start();
    let mut evens: u64 = 0;
    let mut odds: u64 = 0;
    let mut seen: HashSet<Coord> = HashSet::new();
    let mut frontier: Vec<Coord> = plane.neighbors(&start);
    let mut steps = 0;
    // let steps_desired = 26501365;
    let steps_desired = 5000;
    // TODO should we be looking not at just the first plane, but the first 5 so that we get back to the original plane
    // after that?
    // TODO or maybe steps to get back to the start in the next plane?
    // TODO  OR OR
    // if number of steps is logarithmically related to garden count AND number of steps is a factor of desired steps...
    let open_coord_set: HashSet<Coord> = HashSet::from_iter(plane.open_coords().into_iter());
    while open_coord_set.difference(&seen).collect::<HashSet<_>>().len() != 0 {
        steps += 1;

        let mut new_frontier = vec![];
        for coord in &frontier {
            if (0..plane.rows()).contains(&(coord.row as usize)) && (0..plane.cols()).contains(&(coord.col as usize)) {
                if steps % 2 == 0 {
                    evens += 1;
                } else {
                    odds += 1;
                }
            }
            for neighbor in plane.neighbors(&coord) {
                if !seen.contains(&neighbor) {
                    new_frontier.push(neighbor);
                    seen.insert(neighbor);
                }
            }
        }
        frontier = new_frontier;
    }
    println!("original plane counts: odds: {odds}, evens: {evens}");

    match steps_desired {
        even if even % 2 == 0 => evens.to_string(),
        _ => odds.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 42.to_string());
    }
}
