use std::collections::HashSet;

#[derive(Debug,Clone)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct NumCoord {
    num: u32,
    start: Coord,
    end: Coord,
}

#[derive(Debug)]
struct SymbolCoord {
    symbol: char,
    coord: Coord,
}

static digit_chars: &str = "0123456789";
fn parse(input: &str) -> Vec<u32> {
    let digit_chars_set: HashSet<char> = HashSet::from_iter(digit_chars.chars());
    let rows_enumerated = input.lines()
        .map(|line| line.chars().enumerate())
        .enumerate();
    let mut previous_nums: Vec<NumCoord> = vec![];
    let mut previous_symbols: Vec<SymbolCoord> = vec![];
    let mut current_nums: Vec<NumCoord> = vec![];
    let mut current_symbols: Vec<SymbolCoord> = vec![];
    let mut result: Vec<u32> = vec![];
    for (row, cols_enumerated) in rows_enumerated {
        let mut current_num_str: Option<String> = None;
        let mut start = Coord { col: 0, row: 0 };
        for (col, c) in cols_enumerated {
            // dbg!((&row, &col), &c);
            let coord = Coord { row, col };
            match c {
                digit if digit_chars_set.contains(&c) => {
                    if current_num_str.is_some() {
                        let mut s = current_num_str.unwrap();
                        s.push(c);
                        current_num_str = Some(s);
                    } else {
                        current_num_str = Some(c.to_string());
                        start = coord;
                    }
                    // dbg!(&current_num_str);
                },
                symbol => {
                    if let Some(current_num_str) = current_num_str.take() {
                        let end = coord.clone();
                        let num = current_num_str.parse::<u32>().unwrap();
                        let num_coord = NumCoord { num, start: start.clone(), end };
                        current_nums.push(num_coord);
                    }
                    if symbol == '.' {
                        continue;
                    }
                    let symbol_coord = SymbolCoord { symbol, coord };
                    current_symbols.push(symbol_coord);
                },
            }
        }
        dbg!(&current_nums, &current_symbols);
        // for current_num in current_nums.iter() {
        //     if current_symbols.iter().any(|symbol| {
        //         (current_num.start.col > 0 && (current_num.start.col - 1) == symbol.coord.col) ||
        //         current_num.end.col == symbol.coord.col
        //     }) {
        //         result.push(current_num.num);
        //     }
        // }
        // for current_symbol in current_symbols.iter() {
        //     for previous_num in previous_nums.iter() {
        //         if (previous_num.start.col > 0 && (previous_num.start.col - 1) == current_symbol.coord.col) ||
        //             previous_num.end.col == current_symbol.coord.col {
        //             result.push(previous_nums
        //         }
        //     }
        //     if previous_nums.iter().any(|num| {
        //         (num.start.col > 0 && (num.start.col - 1) == current_symbol.coord.col) ||
        //         num.end.col == current_symbol.coord.col
        //     }) {
        //         result.push(num.num);
        //     }
        // }
        previous_nums = current_nums;
        previous_symbols = current_symbols;
        current_nums = vec![];
        current_symbols = vec![];
    }
    result
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    parse(&input).iter()
        .sum::<u32>()
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
