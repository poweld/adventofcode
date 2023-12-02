use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
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
    input.lines()
        .map(parse_line)
        .map(|game| {
            let mut max_values: HashMap<String, u32> = HashMap::new();
            for round_count in game.round_counts.iter() {
                for (color, count) in round_count.iter() {
                    max_values.entry(color.to_string())
                        .and_modify(|max_count| *max_count = std::cmp::max(*count, *max_count))
                        .or_insert(*count);
                }
            }
            max_values.values().fold(1, |acc, value| acc * value)
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
        assert_eq!(result, 2286.to_string());
    }
}
