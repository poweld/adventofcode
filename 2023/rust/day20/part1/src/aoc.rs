trait Module {
    fn handle_pulse(&self, pulse: Pulse);
    fn low_pulses_emitted(&self) -> u64;
    fn high_pulses_emitted(&self) -> u64;
}

#[derive(Debug)]
struct Broadcast;
impl Module for Broadcast {
}

#[derive(Debug)]
struct FlipFlip;
impl Module for FlipFlip {
}

#[derive(Debug)]
struct Conjunction {
    most_recent_pulse: Pulse,
}
impl Module for Conjunction {
}

enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct ParseResult {
}

fn parse(input: &str) -> ParseResult {
    let mut lines = input.lines();
    ParseResult { workflows, partmaps }
}

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
        assert_eq!(result, 19114.to_string());
    }
}
