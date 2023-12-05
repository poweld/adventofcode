use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
#[allow(dead_code)]
struct Map {
    source_range: Range<u64>,
    dest_range: Range<u64>,
}
impl Map {
    fn from_str(map_str: &str) -> Self {
        let mut nums = map_str.split_whitespace()
            .map(|num_str| num_str.parse::<u64>().expect("failed to parse num"));
        let dest_range_start = nums.next().expect("failed to get dest_range_start str");
        let source_range_start = nums.next().expect("failed to get source_range_start str");
        let range_len = nums.next().expect("failed to get range_len str");
        let source_range = source_range_start..(source_range_start + range_len);
        let dest_range = dest_range_start..(dest_range_start + range_len);
        Self { source_range, dest_range }
    }
}

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
struct CacheKey(String, u64, String);

#[derive(Debug)]
struct Mapper {
    maps: Vec<Map>,
}
impl Mapper {
    fn new() -> Self {
        let maps = vec![];
        Self { maps }
    }
    // fn map_to_category(&self, source_category: &str, source_id: u64, dest_category: &str, cache: &mut HashMap<CacheKey, u64>) -> u64 {
    // }
}

fn parse_seeds(seeds_line: &str) -> Vec<Range<u64>> {
    let seed_num_strs = seeds_line
        .split(": ")
        .last()
        .expect("failed to extract seed num strings");
    let seeds_num_split: Vec<&str> = seed_num_strs.split_whitespace().collect();
    seeds_num_split[..].chunks(2)
        .map(|chunk| {
            dbg!(&chunk);
            let seed_range_start = chunk[0].parse::<u64>().expect("could not parse seed range start");
            let seed_range_len = chunk[1].parse::<u64>().expect("could not parse seed range len");
            seed_range_start..(seed_range_start + seed_range_len)
        })
        .collect::<Vec<_>>()
}

fn parse(input: &str) -> (Vec<Range<u64>>, Mapper) {
    // TODO there's a LOT of seeds, may need to approach this differently
    // Get all the ranges that can _lower_ the resulting value, along with _how much_ they lower the resulting value
    // TODO or... take the initial set of map ranges and modify/add to them at each stage, so you end up with a single mapping
    // ^^^^^^^^^^^^^^^^^^  though still not sure how to handle the fact that there's a ton of seeds to check...
    let mut lines = input.lines();
    let seeds_line = lines.next();
    let seed_ranges = parse_seeds(lines.next().unwrap());

    let mut mapper = Mapper::new();
    let mut maps: Vec<Map> = vec![];
    let is_map_line = |line: &&str| !(line.is_empty() || line.ends_with(&":"));
    for line in lines.filter(is_map_line) {
        let map = Map::from_str(line);
        let source_range = map.source_range;
        for maps_index in 0..maps.len() {
            let existing_source_range = maps[maps_index].source_range;
            if (
                existing_source_range.start == source_range.start &&
                existing_source_range.end == source_range.end
            ) {
                maps[maps_index] = map;
            } else if (
                existing_source_range.contains(source_range.end - 1)
                // TODO charlie is staring at me. need to update the existing range and possibly add new ones.
            )
        }
    }
    let mapper = Mapper { maps };
    (seed_ranges, mapper)
}

// TODO is this needed?
fn range_overlap(range1: &Range<u64>, range2: &Range<u64>) -> Option<Range<u64>> {
    if range2.start >= range1.start {
        Some(Range { start: range2.start, end: range1.end })
    } else if range1.start > range2.start {
        Some(Range { start: range1.start, end: range2.end })
    } else {
        None
    }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let (seed_ranges, mapper) = parse(&input);
    dbg!(&seed_ranges, &mapper);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 46.to_string());
    }
}
