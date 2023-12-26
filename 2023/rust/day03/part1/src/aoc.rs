use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Board {
    rows: usize,
    cols: usize,
}
impl Board {
    fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
    fn row_range_neighbors(&self, coord_a: &Coord, coord_b: &Coord) -> Vec<Coord> {
        let start_row = match coord_a.row {
            0 => 0,
            row => row - 1,
        };
        let end_row = std::cmp::min(coord_b.row + 1, self.rows - 1);
        let start_col = match coord_a.col {
            0 => 0,
            col => col - 1,
        };
        let end_col = std::cmp::min(coord_b.col + 1, self.cols - 1);
        let in_num_cols = coord_a.col..=coord_b.col;
        (start_row..=end_row)
            .flat_map(|current_row| {
                let cols = in_num_cols.clone();
                (start_col..=end_col)
                    .filter(move |current_col| current_row != coord_a.row || !cols.contains(&current_col))
                    .map(move |current_col| Coord { row: current_row, col: current_col })
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

struct ParseOutput {
    symbols: HashMap<char, Vec<Coord>>,
    nums: Vec<(usize, (Coord, Coord))>,
    board: Board,
}

fn parse(input: &str) -> ParseOutput {
    let (rows, cols) = dimensions(input);
    let mut symbols: HashMap<char, Vec<Coord>> = HashMap::new();
    let mut nums: Vec<(usize, (Coord, Coord))> = Vec::new();
    for (row, line) in input.lines().enumerate() {
        let mut num_chars: Vec<(char, Coord)> = Vec::new();
        for (col, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let coord = Coord { row, col };
                num_chars.push((c, coord));
            } else {
                if c != '.' {
                    let coord = Coord { row, col };
                    symbols.entry(c)
                        .and_modify(|coords| (*coords).push(coord.clone()))
                        .or_insert(Vec::from([coord]));
                }
                if num_chars.len() > 0 {
                    let coord_a = &num_chars.first().unwrap().1;
                    let coord_b = &num_chars.last().unwrap().1;
                    let num: usize = num_chars.iter().map(|nc| nc.0).collect::<String>().parse().unwrap();
                    nums.push((num, (coord_a.clone(), coord_b.clone())));
                    num_chars = Vec::new();
                }
            }
        }
        if num_chars.len() > 0 {
            let coord_a = &num_chars.first().unwrap().1;
            let coord_b = &num_chars.last().unwrap().1;
            let num: usize = num_chars.iter().map(|nc| nc.0).collect::<String>().parse().unwrap();
            nums.push((num, (coord_a.clone(), coord_b.clone())));
        }
    }
    let board = Board::new(rows, cols);
    ParseOutput { symbols, nums, board }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseOutput { symbols, nums, board } = parse(&input);
    let nums_touching_symbols: Vec<usize> = nums.iter()
        .filter(|(_, (start, end))| {
            board.row_range_neighbors(&start, &end).iter()
                .any(|neighbor| symbols.values().any(|coords| coords.contains(neighbor)))
        })
        .map(|nc| nc.0)
        .collect();
    nums_touching_symbols.iter().sum::<usize>().to_string()
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
