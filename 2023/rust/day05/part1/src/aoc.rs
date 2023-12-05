use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
#[allow(dead_code)]
struct Map {
    source_category: String,
    dest_category: String,
    source_range: Range<u64>,
    dest_range: Range<u64>,
}

#[derive(Debug)]
struct Mapper {
    category_maps: HashMap<String, Vec<Map>>,
}
impl Mapper {
    fn new() -> Self {
        let category_maps = HashMap::new();
        Self { category_maps }
    }
    fn map_to_category(&self, source_category: &str, source_id: u64, dest_category: &str) -> u64 {
        if source_category == dest_category {
            source_id
        } else {
            let category_maps = self.category_maps.get(source_category)
                .expect("failed to find source category");
            let category_map = category_maps.iter()
                .find(|m| m.source_range.contains(&source_id));
            match category_map {
                Some(category_map) => {
                    let range_offset = source_id - category_map.source_range.start;
                    let new_source_id = category_map.dest_range.start + range_offset;
                    self.map_to_category(&category_map.dest_category, new_source_id, dest_category)
                },
                None => {
                    // TODO uh oh, don't have a dest category to pull from when no mapping was added
                    // does this need to be statically added?
                    // HACK IT FOR NOW
                    let category_map = category_maps.iter().next().expect("couldn't find a single category map");
                    self.map_to_category(&category_map.dest_category, source_id, dest_category)
                }
            }
        }
    }
}

fn parse(input: &str) -> (Vec<u64>, Mapper) {
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
    lines.next();  // Consume empty line

    let mut mapper = Mapper::new();
    let mut source_category = String::new();
    let mut dest_category = String::new();
    let mut definition_line = true;
    let mut maps: Vec<Map> = vec![];
    for line in lines {
        if line.is_empty() {
            mapper.category_maps.insert(source_category.clone(), maps);
            maps = vec![];
            definition_line = true;
        } else if definition_line {
            let mut definition_parts = line.split_whitespace()
                .next().expect("failed to get first part of definition line")
                .split("-");
            source_category = definition_parts.next().expect("failed to get source category").to_string();
            definition_parts.next();
            dest_category = definition_parts.next().expect("failed to get dest category").to_string();
            definition_line = false;
        } else {
            let mut nums = line.split_whitespace()
                .map(|num_str| num_str.parse::<u64>().expect("failed to parse num"));
            let dest_range_start = nums.next().expect("failed to get dest_range_start str");
            let source_range_start = nums.next().expect("failed to get source_range_start str");
            let range_len = nums.next().expect("failed to get range_len str");
            let source_range = source_range_start..(source_range_start + range_len);
            let dest_range = dest_range_start..(dest_range_start + range_len);
            let map = Map {
                source_category: source_category.clone(),
                dest_category: dest_category.clone(),
                source_range,
                dest_range
                };
            maps.push(map);
        }
    }
    mapper.category_maps.insert(source_category.clone(), maps);

    (seeds, mapper)
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let (seeds, mapper) = parse(&input);
    seeds.iter()
        .map(|seed| mapper.map_to_category(&"seed", *seed, &"location"))
        .min().expect("could not get min")
        .to_string()
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
