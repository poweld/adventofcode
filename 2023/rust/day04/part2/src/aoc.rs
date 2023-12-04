use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
#[allow(dead_code)]
struct Card {
    id: u32,
    numbers: Vec<u32>,
    winning_numbers: HashSet<u32>,
    count: u32,
}
impl Card {
    fn win_count(&self) -> u32 {
        self.numbers.iter()
            .filter(|num| self.winning_numbers.contains(num))
            .fold(0u32, |acc, _| acc + 1)
    }
}

fn parse_line(line: &str) -> Card {
    let mut header_and_rest = line.split(": ");
    let id = header_and_rest.next().unwrap()
        .split_whitespace()
        .last().unwrap()
        .parse::<u32>().unwrap();
    let mut winning_and_given_nums = header_and_rest.next().unwrap()
        .split(" | ")
        .map(|nums| {
            nums.split_whitespace()
                .map(|num| num.parse::<u32>().expect("failed to parse num: {num}"))
        });
    let winning_numbers: HashSet<u32> = winning_and_given_nums.next().unwrap().collect();
    let numbers: Vec<u32> = winning_and_given_nums.next().unwrap().collect();
    Card { id, numbers, winning_numbers, count: 1 }
}

fn parse(input: &str) -> Vec<Card> {
    input.lines()
        .map(parse_line)
        .collect::<Vec<_>>()
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut cards: VecDeque<Card> = VecDeque::from(parse(&input));
    let mut total_count = 0;
    while !cards.is_empty() {
        let current_card = cards.pop_front().unwrap();
        cards.iter_mut()
            .take(current_card.win_count() as usize)
            .for_each(|card| card.count += current_card.count);
        total_count += current_card.count;
    }
    total_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 30.to_string());
    }
}
