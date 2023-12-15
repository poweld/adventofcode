#[derive(Debug)]
struct Instruction(String);
impl Instruction {
    fn hashed(&self) -> u8 {
        let result = self.0.chars()
            .map(|c| c as u64)
            .fold(0u64, |acc, ascii_val| {
                ((acc + ascii_val) * 17) % 256
            });
        result as u8
    }
}

#[derive(Debug)]
struct InitSequence(Vec<Instruction>);

#[derive(Debug)]
struct ParseResult {
    init_sequence: InitSequence,
}

fn parse(input: &str) -> ParseResult {
    let instructions = input.lines()
        .flat_map(|line| line.split(","))
        .map(String::from)
        .map(|s| Instruction(s)).collect::<Vec<_>>();
    ParseResult { init_sequence: InitSequence(instructions) }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { init_sequence } = parse(&input);
    init_sequence.0.iter()
        .map(|instruction| instruction.hashed())
        .map(|n| n as u64)
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
        assert_eq!(result, 1320.to_string());
    }
}
