mod my {
    #[derive(Debug)]
    pub struct ParseResult {
    }

    pub fn parse(input: &str) -> ParseResult {
        todo!()
    }
}

use my::*;

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { } = parse(&input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        todo!();
        assert_eq!(result, 0.to_string());
    }
}
