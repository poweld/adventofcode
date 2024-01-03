use std::ops::Range;

#[derive(Debug)]
struct Map {
    source_range: Range<u64>,
    dest_range: Range<u64>,
}
impl Map {
    fn process(&self, item: u64) -> Option<u64> {
        if self.source_range.contains(&item) {
            Some(self.dest_range.start + item - self.source_range.start)
        } else { None }
    }
}

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<Map>>) {
    let mut lines = input.lines();
    let seeds_line = lines.next();
    let seed_num_strs = seeds_line
        .map(|s| s.split(": "))
        .map(|split| split.last())
        .flatten()
        .expect("failed to extract seed num strings");
    let seeds: Vec<u64> = seed_num_strs.split_whitespace()
        .map(|num_str| num_str.parse::<u64>().expect("failed to parse num"))
        .collect();
    lines.next(); lines.next();

    let mut map_stages: Vec<Vec<Map>> = Vec::new();
    let mut current_map_stage: Vec<Map> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            map_stages.push(current_map_stage);
            current_map_stage = Vec::new();
            lines.next();
            continue;
        }
        let map_parts: Vec<u64> = line.split_whitespace()
            .map(|num_str| num_str.parse::<u64>().expect("failed to parse num"))
            .collect();
        let dest_range_start = map_parts[0];
        let source_range_start = map_parts[1];
        let range_len = map_parts[2];
        let map = Map {
            source_range: source_range_start..(source_range_start + range_len),
            dest_range: dest_range_start..(dest_range_start + range_len),
        };
        current_map_stage.push(map);

    }
    map_stages.push(current_map_stage);
    (seeds, map_stages)
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let (seeds, map_stages) = parse(&input);
    let results = seeds.into_iter().map(|seed| {
        map_stages.iter().fold(seed, |acc, map_stage| {
            map_stage.iter().map(|map| map.process(acc))
                .find(|result| result.is_some())
                .flatten()
                .unwrap_or(acc)
        })
    });
    results.min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 35.to_string());
    }
}
