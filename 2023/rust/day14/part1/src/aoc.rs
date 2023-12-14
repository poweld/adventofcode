#[derive(Debug)]
struct Dish(Vec<String>);
impl Dish {
    fn rows(&self) -> usize {
        self.0.len()
    }
    fn cols(&self) -> usize {
        self.0[0].len()
    }
    fn rotated_and_reflected(&self) -> Dish {
        // First column becomes first row, and so on
        let mut col_strs = vec![];
        for _ in 0..self.cols() {
            col_strs.push(String::new());
        }
        for row in self.0.iter() {
            for (col_index, col) in row.chars().enumerate() {
                col_strs[col_index].push(col);
            }
        }
        Dish(col_strs)
    }
    fn rotate_and_reflect(&mut self) {
        self.0 = self.rotated_and_reflected().0;
    }
    fn tilt_left(&mut self) {
        self.0 = self.0.iter()
            .map(|row_str| {
                row_str.split("#")
                    .map(|s| {
                        let mut chars: Vec<char> = s.chars().collect();
                        chars.sort_by(|a, b| {
                            if a == b {
                                std::cmp::Ordering::Equal
                            } else if a == &'O' {
                                std::cmp::Ordering::Less
                            } else {
                                std::cmp::Ordering::Greater
                            }
                        });
                        chars.into_iter().collect::<String>()
                    })
                    .collect::<Vec<_>>()
                    .join("#")
            })
            .collect::<Vec<_>>();
    }
}

#[derive(Debug)]
struct ParseResult {
    dish: Dish,
}

fn parse(input: &str) -> ParseResult {
    let dish = Dish(input.lines().map(String::from).collect::<Vec<_>>());
    ParseResult { dish }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { mut dish } = parse(&input);
    dbg!(&dish);
    dish.rotate_and_reflect();
    dish.tilt_left();
    dbg!(&dish);
    dish.0.iter()
        .map(|s| {
            s.chars()
                .zip((1..=dish.cols()).rev())
                .filter(|(c, _)| c == &'O')
                .map(|(_, value)| value as u64)
                .sum::<u64>()
         })
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
        assert_eq!(result, 136.to_string());
    }
}
