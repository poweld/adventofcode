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
    fn is_row_reflection(&self, row_index_a: &usize, row_index_b: &usize) -> bool {
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
    fn row_reflection_indices(&self) -> Option<(usize, usize)> {
        let row_indices = (0..self.rows()).collect::<Vec<_>>();
        let row_index_windows = row_indices[..].windows(2);
        for row_index_window in row_index_windows {
            if let [row_index_a, row_index_b] = row_index_window {
                if self.is_row_reflection(row_index_a, row_index_b) {
                    return Some((*row_index_a, *row_index_b));
                }
            }
        }
        return None
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
        .map(|pattern| {
            if let Some((index_a, index_b)) = pattern.row_reflection_indices() {
                index_b
            } else if let Some((index_a, index_b)) = pattern.rotated_and_reflected().row_reflection_indices() {
            } else {
                panic!("could not find reflection")
            }
        })
    dbg!(&patterns);
    dbg!(patterns[0].rotated_and_reflected());
    dbg!(patterns[0].row_reflection_indices());
    dbg!(patterns[0].rotated_and_reflected().row_reflection_indices());
    todo!()
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
}
