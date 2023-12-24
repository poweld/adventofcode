use std::collections::HashSet;

#[derive(Debug)]
#[allow(dead_code)]
struct Card {
    id: u32,
    numbers: Vec<u32>,
    winning_numbers: HashSet<u32>,
}
impl Card {
    fn score(&self) -> u32 {
        self.numbers.iter()
            .filter(|num| self.winning_numbers.contains(num))
            .fold(0u32, |acc, _| {
                match acc {
                    0 => 1,
                    x => x * 2,
                }
            })
    }
}

fn parse_line(line: &str) -> Card {
    let (header, rest) = line.split_once(": ").unwrap();
    let id = header.split_whitespace()
        .last().unwrap()
        .parse::<u32>().unwrap();
    let mut winning_and_given_nums = rest.split(" | ")
        .map(|nums| {
            nums.split_whitespace()
                .map(|num| num.parse::<u32>().expect("failed to parse num: {num}"))
        });
    let winning_numbers: HashSet<u32> = winning_and_given_nums.next().unwrap().collect();
    let numbers: Vec<u32> = winning_and_given_nums.next().unwrap().collect();
    Card { id, numbers, winning_numbers }
}

fn parse(input: &str) -> Vec<Card> {
    input.lines()
        .map(parse_line)
        .collect::<Vec<_>>()
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    parse(&input).iter()
        .map(|card| card.score())
        .sum::<u32>()
        .to_string()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 13.to_string());
    }
}
