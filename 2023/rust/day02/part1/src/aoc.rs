use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub round_counts: Vec<HashMap<String, u32>>,
}

fn parse_line(line: &str) -> Game {
    let (game, rounds) = line.split_once(": ").unwrap();
    let id = game.split_whitespace()
        .last()
        .map(|s| s.parse::<u32>().ok())
        .flatten()
        .unwrap();
    let round_counts = rounds.split("; ")
        .map(|round| {
            round.split(", ")
                .map(|entry| entry.split_once(' ').unwrap())
                .map(|(count, color)| {
                    let count = count.parse::<u32>().unwrap();
                    let color = color.to_string();
                    (color, count)
                })
                .collect()
        }).collect();
    Game { id, round_counts }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let limits = [(12, String::from("red")), (13, String::from("green")), (14, String::from("blue"))];
    input.lines()
        .map(parse_line)
        .filter(|game| {
            game.round_counts.iter().all(|round_count| {
                limits.iter().all(|(limit, color)| {
                    round_count.get(color).unwrap_or(&0) <= limit
                })
            })
        })
        .map(|game| game.id)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 8.to_string());
    }
}
