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
fn one_difference_location(str_a: &str, str_b: &str) -> Option<usize> {
    assert!(str_a.len() == str_b.len());
    let bitmask_a = str_to_bitmask(str_a);
    let bitmask_b = str_to_bitmask(str_b);
    let mut location: Option<usize> = None;
    for bitshift in 0..u32::BITS {
        if bitmask_a >> bitshift & 0b1 != bitmask_b >> bitshift & 0b1 {
            if location.is_some() {
                return None;
            }
            location = Some(str_a.len() - 1 - (bitshift as usize));
        }
    }
    location
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
    fn rotate_and_reflect(&mut self) {
        self.0 = self.rotated_and_reflected().0;
    }
    fn is_row_reflection(&self, row_index_a: &usize, row_index_b: &usize) -> bool {
        let iter_a = self.0[..=*row_index_a].iter().rev();
        let iter_b = self.0[*row_index_b..].iter();
        let mut zip = iter_a.zip(iter_b);
        zip.all(|(a, b)| a == b)
    }
    fn fix_if_smudged_row_reflection(&mut self, row_index_a: &usize, row_index_b: &usize) -> bool {
        // Return whether smudge was found && fixed
        let iter_a = (0..=*row_index_a).rev().map(|index_a| (index_a, &self.0[index_a]));
        let iter_b = (0..=*row_index_b).rev().map(|index_b| (index_b, &self.0[index_b]));
        let mut zip = iter_a.zip(iter_b);
        let mut smudge_at: Option<(usize, usize)> = None;
        let all_eq = zip.all(|((_row_index_a, a), (row_index_b, b))| {
            dbg!(&a, &b);
            if smudge_at.is_none() {
                dbg!(one_difference_location(&a, &b));
                if let Some(col_index_b) = one_difference_location(&a, &b) {
                    smudge_at = Some((row_index_b, col_index_b));
                    dbg!(&smudge_at);
                    return true;
                }
                return false;
            }
            a == b
        });
        if all_eq {
            if let Some((row_index_b, col_index_b)) = smudge_at {
                self.0[row_index_b].replace_range(col_index_b..col_index_b, "#");  // Doesn't matter which we replace with
                return true;
            }
            return false;
        }
        false
    }
    fn row_reflection_indices(&self) -> Vec<(usize, usize)> {
        let row_indices = (0..self.rows()).collect::<Vec<_>>();
        let row_index_windows = row_indices[..].windows(2);
        let mut result = vec![];
        for row_index_window in row_index_windows {
            if let [row_index_a, row_index_b] = row_index_window {
                if self.is_row_reflection(row_index_a, row_index_b) {
                    result.push((*row_index_a, *row_index_b));
                }
            }
        }
        result
    }
    fn fix_smudge(&mut self) {
        let row_indices = (0..self.rows()).collect::<Vec<_>>();
        let row_index_windows = row_indices[..].windows(2);
        for row_index_window in row_index_windows {
            if let [row_index_a, row_index_b] = row_index_window {
                if self.fix_if_smudged_row_reflection(row_index_a, row_index_b) {
                    return;
                }
            }
        }

        self.rotate_and_reflect();
        let row_indices = (0..self.rows()).collect::<Vec<_>>();
        let row_index_windows = row_indices[..].windows(2);
        for row_index_window in row_index_windows {
            if let [row_index_a, row_index_b] = row_index_window {
                if self.fix_if_smudged_row_reflection(row_index_a, row_index_b) {
                    self.rotate_and_reflect();
                    return;
                }
            }
        }

        panic!("did not find smudge to fix");
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
    let ParseResult { mut patterns } = parse(&input);
    for mut pattern in patterns.iter_mut() {
        pattern.fix_smudge();
    }
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
        assert_eq!(result, 400.to_string());
    }

    #[test]
    fn str_to_bitmask_test() {
        let result = str_to_bitmask("#.#.#");
        assert_eq!(result, 21);
    }

    #[test]
    fn one_difference_location_test() {
        let result = one_difference_location("#.#.#", "#.#..");
        assert_eq!(result, Some(4));
        let result = one_difference_location("#...#", "#.#.#");
        assert_eq!(result, Some(2));
        let result = one_difference_location("#.##..##.", "..##..##.");
        assert_eq!(result, Some(0));
        let result = one_difference_location("#.#.#", "#....");
        assert_eq!(result, None);
        let result = one_difference_location("#.#.#", "#.#.#");
        assert_eq!(result, None);
    }
}
