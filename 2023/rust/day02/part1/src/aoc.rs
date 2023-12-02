use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub round_counts: Vec<HashMap<String, u32>>,
}

fn parse_line(line: &str) -> Game {
    let game_and_rounds: Vec<&str> = line.split(": ").collect();
    let id = game_and_rounds[0]
        .split_whitespace()
        .last()
        .map(|s| s.parse::<u32>().ok())
        .flatten()
        .unwrap();
    let rounds = game_and_rounds[1].split("; ");
    let round_counts = rounds.map(|round| {
        let entries = round.split(", ");
        entries
            .map(|entry| entry.split_whitespace().collect::<Vec<_>>())
            .map(|count_and_color| {
                let count = count_and_color[0].parse::<u32>().unwrap();
                let color = count_and_color[1].to_string();
                (color, count)
            })
            .collect()
    }).collect();
    Game { id, round_counts }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let limits = [(12, "red"), (13, "green"), (14, "blue")];
    input.lines()
        .map(parse_line)
        .filter(|game| {
            game.round_counts.iter().all(|round_count| {
                limits.iter().all(|(limit, color)| {
                    round_count.get(&color[..]).unwrap_or_else(|| &0) <= limit
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
