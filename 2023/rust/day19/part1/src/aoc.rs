use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    attribute: char,
    operator: char,
    success_rule_name: String,
};

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
};

#[derive(Debug)]
struct ParseResult {
    workflows: Vec<Workflow>,
}

fn parse(input: &str) -> ParseResult {
    let workflows: Vec<Vec<char>> = input.lines()
        .map(|line| {
            let mut rules = line.split(',');
        })
        .collect();
    ParseResult { workflows }
}

#[derive(Debug, Clone)]
struct Runner {
    coord: Coord,
    direction: Direction,
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { plane } = parse(&input);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 19114.to_string());
    }
}
