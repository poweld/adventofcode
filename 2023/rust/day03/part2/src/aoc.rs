use std::collections::{HashSet, HashMap};
use std::hash::Hash;

#[derive(Debug)]
struct Board {
    rows: usize,
    cols: usize,
    chars: Vec<Vec<char>>,
}
impl Board {
    fn new(rows: usize, cols: usize, data: &str) -> Self {
        let chars: Vec<Vec<char>> = data.lines()
            .map(|line| line.chars().collect())
            .collect();
        Self { rows, cols, chars }
    }
    fn get(&self, row: usize, col: usize) -> Option<char> {
        self.chars.get(row)?.get(col).copied()
    }
    // This could be better
    fn neighbors(&self, row: usize, col: usize) -> Vec<(char, Coord)> {
        let start_row = match row {
            0 => 0,
            row => row - 1,
        };
        let end_row = std::cmp::min(row + 1, self.rows - 1);
        let start_col = match col {
            0 => 0,
            col => col - 1,
        };
        let end_col = std::cmp::min(col + 1, self.cols - 1);
        (start_row..=end_row)
            .flat_map(|current_row| {
                (start_col..=end_col)
                    .filter(move |current_col| current_row != row || current_col != &col)
                    .map(move |current_col| {
                        let coord = Coord { row: current_row, col: current_col };
                        (self.get(current_row, current_col).unwrap(), coord)
                    })
            })
            .collect()
    }
}

fn dimensions(input: &str) -> (usize, usize) {
    let lines = input.lines();
    let cols = lines.clone().next().unwrap().len();
    let rows = lines.map(|_| 1).sum::<usize>();
    (rows, cols)
}

fn parse(input: &str) -> Board {
    let (rows, cols) = dimensions(input);
    Board::new(rows, cols, input)
}

#[derive(Debug,Hash,PartialEq,Eq,Clone,Copy)]
struct Coord {
    row: usize,
    col: usize,
}

static DIGIT_CHARS: &str = "0123456789";
pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let digit_chars_set: HashSet<char> = HashSet::from_iter(DIGIT_CHARS.chars());
    let is_digit = |c: &char| digit_chars_set.contains(&c);
    let is_star = |c: &char| c == &'*';
    let board = parse(&input);
    let mut star_location_to_nums: HashMap<Coord, Vec<u64>> = HashMap::new();
    for row in 0..board.rows {
        let mut current_num_str = String::new();
        let mut star_coords: HashSet<Coord> = HashSet::new();
        for col in 0..board.cols {
            let c = board.get(row, col).unwrap();
            if is_digit(&c) {
                current_num_str.push(c);
                let star_neighbors = board.neighbors(row, col);
                let star_neighbors = star_neighbors.iter()
                    .filter(|(neighbor, _)| is_star(neighbor))
                    .collect::<Vec<_>>();
                for (_, coord) in star_neighbors.iter() {
                    star_coords.insert(*coord);
                }
                if star_coords.len() > 0 && col == (board.cols - 1) {
                    // Make sure to capture if last col
                    let num = current_num_str.parse::<u64>().unwrap();
                    for coord in star_coords.iter() {
                        star_location_to_nums.entry(*coord)
                            .and_modify(|nums| nums.push(num))
                            .or_insert(vec![num]);
                    }
                }
            } else {
                if current_num_str.len() > 0 {
                    if star_coords.len() > 0 {
                        let num = current_num_str.parse::<u64>().unwrap();
                        for coord in star_coords.iter() {
                            star_location_to_nums.entry(*coord)
                                .and_modify(|nums| nums.push(num))
                                .or_insert(vec![num]);
                        }
                    }
                    current_num_str = String::new();
                }
                star_coords = HashSet::new();
            }
        }
    }
    star_location_to_nums.values()
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 467835.to_string());
    }
}
