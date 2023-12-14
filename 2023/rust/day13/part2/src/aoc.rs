fn str_to_bitmask(s: &str) -> u32 {
    let mut result = 0;
    for c in s.chars() {
        result <<= 1;
        if c == '#' {
            result += 1
        }
    }
    result
}
fn exactly_one_difference(str_a: &str, str_b: &str) -> bool {
    let bitmask_a = str_to_bitmask(str_a);
    let bitmask_b = str_to_bitmask(str_b);
    let mut found_one = false;
    for bitshift in 0..u32::BITS {
        if bitmask_a >> bitshift & 0b1 != bitmask_b >> bitshift & 0b1 {
            if found_one {
                return false;
            }
            found_one = true;
        }
    }
    return found_one;
}

#[derive(Debug)]
struct Pattern(Vec<String>);
impl Pattern {
    fn rows(&self) -> usize {
        self.0.len()
    }
    fn cols(&self) -> usize {
        self.0[0].len()
    }
    fn rotated_and_reflected(&self) -> Pattern {
        // First column becomes first row. For easier column comparisons
        let mut col_strs = vec![];
        for _ in 0..self.cols() {
            col_strs.push(String::new());
        }
        for row in self.0.iter() {
            for (col_index, col) in row.chars().enumerate() {
                col_strs[col_index].push(col);
            }
        }
        Pattern(col_strs)
    }
    fn is_row_reflection(&self, row_index_a: &usize, row_index_b: &usize, with_smudge: bool) -> bool {
        let mut row_index_a = row_index_a.clone();
        let mut row_index_b = row_index_b.clone();
        let rows = self.rows();
        while row_index_a > 0 && row_index_b < rows - 1 {
            if self.0[row_index_a] != self.0[row_index_b] {
                return false;
            }
            row_index_a -= 1;
            row_index_b += 1;
        }
        self.0[row_index_a] == self.0[row_index_b]
    }
    fn row_reflection_indices(&self) -> Vec<(usize, usize)> {
        let row_indices = (0..self.rows()).collect::<Vec<_>>();
        let row_index_windows = row_indices[..].windows(2);
        let mut result = vec![];
        for row_index_window in row_index_windows {
            if let [row_index_a, row_index_b] = row_index_window {
                if self.is_row_reflection(row_index_a, row_index_b, false) {
                    result.push((*row_index_a, *row_index_b));
                }
            }
        }
        result
    }
    fn fix_smudge(&mut self) {
    }
}

#[derive(Debug)]
struct ParseResult {
    patterns: Vec<Pattern>,
}

fn parse(input: &str) -> ParseResult {
    let mut patterns = vec![];
    let mut pattern_data = vec![];
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(Pattern(pattern_data));
            pattern_data = vec![];
        } else {
            pattern_data.push(String::from(line));
        }
    }
    patterns.push(Pattern(pattern_data));
    ParseResult { patterns }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { patterns } = parse(&input);
    patterns.iter()
        .flat_map(|pattern| {
            pattern.row_reflection_indices().iter()
                .map(|(_, index_b)| index_b * 100)
                .chain(pattern.rotated_and_reflected().row_reflection_indices().iter()
                    .map(|(_, index_b)| *index_b)
                )
                .collect::<Vec<_>>()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 405.to_string());
    }

    #[test]
    fn str_to_bitmask_test() {
        let result = str_to_bitmask("#.#.#");
        assert_eq!(result, 21);
    }

    #[test]
    fn exactly_one_difference_test() {
        let result = exactly_one_difference("#.#.#", "#.#..");
        assert_eq!(result, true);
        let result = exactly_one_difference("#.#.#", "#....");
        assert_eq!(result, false);
        let result = exactly_one_difference("#.#.#", "#.#.#");
        assert_eq!(result, false);
    }
}
