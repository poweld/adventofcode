pub fn single_difference_at(a: &str, b: &str) -> Option<usize> {
    let mut a_chars = a.chars();
    let mut b_chars = b.chars();
    let mut difference_at: Option<usize> = None;
    for index in 0..a.len() {
        let a_char = a_chars.next();
        let b_char = b_chars.next();
        if a_char != b_char {
            if difference_at.is_some() {
                return None;
            }
            difference_at = Some(index);
        }
    }
    difference_at
}

pub fn parse(input: &String) -> Vec<String> {
    input.lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).expect("failed to read input path: {input_path}");
    let lines = parse(&input);
    let mut iter_a = lines.iter();
    while let Some(a) = iter_a.next() {
        for b in iter_a.clone() {
            if let Some(diff_at) = single_difference_at(&a, &b) {
                dbg!(&a, &b, &diff_at);
                let mut chars: Vec<char> = a.chars().collect();
                chars.remove(diff_at);
                return chars.into_iter().collect();
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, "fgij".to_string());
    }
}
