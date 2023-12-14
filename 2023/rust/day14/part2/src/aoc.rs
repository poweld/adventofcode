use std::collections::HashMap;

#[derive(Debug)]
struct Dish(Vec<String>);
impl Dish {
    // fn rows(&self) -> usize {
    //     self.0.len()
    // }
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
    fn tilt(&mut self, direction: &char) {
        match direction {
            &'N' | &'S' => self.rotate_and_reflect(),
            _ => (),
        };
        self.0 = self.0.iter()
            .map(|row_str| {
                row_str.split("#")
                    .map(|s| {
                        let mut chars: Vec<char> = s.chars().collect();
                        chars.sort();
                        let iter = chars.into_iter();
                        match direction {
                            &'N' | &'W' => iter.rev().collect::<String>(),
                            &'S' | &'E' => iter.collect::<String>(),
                            _ => panic!("unexpected direction char"),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("#")
            })
            .collect::<Vec<_>>();
        match direction {
            &'N' | &'S' => self.rotate_and_reflect(),
            _ => (),
        };
    }
    fn serialized(&self) -> String {
        self.0.join("")
    }
    fn wash_cycle(&mut self, count: u64) {
        let mut seen: HashMap<String, u64> = HashMap::new();
        let mut loop_range: Option<std::ops::RangeInclusive<u64>> = None;
        let tilt_order = [&'N', &'W', &'S', &'E'];
        for n in 0..count {
            let key = self.serialized();
            if let Some(first_n) = seen.get(&key) {
                loop_range = Some(*first_n..=n);
                break;
            }
            seen.insert(key, n);
            for direction in tilt_order {
                self.tilt(direction);
            }
        }
        if let Some(loop_range) = loop_range {
            let remaining = (count - loop_range.start()) % (loop_range.end() - loop_range.start());
            for _ in 0..remaining {
                for direction in tilt_order {
                    self.tilt(direction);
                }
            }
        }
    }
    fn weight_on_north_beams(&self) -> u64 {
        let dish = self.rotated_and_reflected();
        dish.0.iter()
            .map(|s| {
                s.chars()
                    .zip((1..=dish.cols()).rev())
                    .filter(|(c, _)| c == &'O')
                    .map(|(_, value)| value as u64)
                    .sum::<u64>()
             })
            .sum::<u64>()
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
    let cycles = 1_000_000_000;
    dish.wash_cycle(cycles);
    dish.weight_on_north_beams().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 64.to_string());
    }
}
