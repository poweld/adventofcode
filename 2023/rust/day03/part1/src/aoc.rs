use std::collections::HashSet;

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
    fn neighbors(&self, row: usize, col: usize) -> Vec<char> {
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
                    .map(move |current_col| self.get(current_row, current_col).unwrap())
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

static DIGIT_CHARS: &str = "0123456789";
pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let digit_chars_set: HashSet<char> = HashSet::from_iter(DIGIT_CHARS.chars());
    let is_digit = |c: &char| digit_chars_set.contains(&c);
    let is_symbol = |c: &char| !is_digit(c) && c != &'.';
    let board = parse(&input);
    let mut parts: Vec<u64> = vec![];
    for row in 0..board.rows {
        let mut current_num_str = String::new();
        let mut is_part_number = false;
        for col in 0..board.cols {
            let c = board.get(row, col).unwrap();
            if is_digit(&c) {
                current_num_str.push(c);
                if !is_part_number {
                    is_part_number = board.neighbors(row, col).iter()
                        .any(is_symbol)
                } else if col == (board.cols - 1) {
                    // Make sure to capture if last col
                    parts.push(current_num_str.parse::<u64>().unwrap());
                }
            } else {
                if current_num_str.len() > 0 {
                    if is_part_number {
                        parts.push(current_num_str.parse::<u64>().unwrap());
                    }
                    current_num_str = String::new();
                }
                is_part_number = false;
            }
        }
    }
    parts.iter()
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 4361.to_string());
    }
}
