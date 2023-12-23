static DIGIT_CHARS: &str = "1234567890";
fn is_digit_char(c: &char) -> bool {
    DIGIT_CHARS.contains(*c)
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    input.lines()
        .map(|line| line.chars())
        .map(|chars| {
            let first = chars.clone().find(is_digit_char).unwrap();
            let last = chars.clone().rfind(is_digit_char).unwrap();
            first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 142.to_string());
    }
}
