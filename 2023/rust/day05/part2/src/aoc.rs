use std::ops::Range;

enum OverlapResult {
    OverlapLeft,
    OverlapFull,
    OverlapRight,
    NoOverlap,
}

fn overlap(range1: &Range<u64>, range2: &Range<u64>) -> OverlapResult {
        /*
        [ and ) may be in the same location!
        OverlapFull: [-----)
                       [-)

        OverlapLeft:      [-----)
                     [-----)

        OverlapRight: [-----)
                           [-----)
        NoOverlap: [-----)
                         [-----)
        */
        //dbg!("overlap(", &range1, &range2, ")");
        if range2.start == range2.end {
            if range1.contains(&range2.start) {
               OverlapResult::OverlapFull
            } else {
               OverlapResult::NoOverlap
            }
        } else {
            if range1.contains(&range2.start) &&
               range1.contains(&(range2.end - 1)) {
               OverlapResult::OverlapFull
            } else if range1.contains(&(range2.end - 1)) {
               OverlapResult::OverlapLeft
            } else if range1.contains(&range2.start) {
               OverlapResult::OverlapRight
            } else {
               OverlapResult::NoOverlap
            }
        }
}

fn overlay(seed_range: &Range<u64>, source_range: &Range<u64>, dest_range: &Range<u64>) -> Vec<Range<u64>> {
    match overlap(&seed_range, &source_range) {
        OverlapResult::OverlapFull => {
           vec![
               Range { start: seed_range.start, end: source_range.start },
               dest_range.clone(),
               Range { start: source_range.end, end: seed_range.end },
           ]
        },
        OverlapResult::OverlapLeft => {
            // TODO bleh getting tired of thinking about this, but I think we're on the right track
            let offset = seed_range.start - source_range.start;
            let mapped_len = source_range.end - seed_range.start;
            let dest_range_start = dest_range.start + offset;
            vec![
                Range { start: dest_range_start, end: dest_range_start + mapped_len },
                Range { start: seed_range.start + mapped_len, end: seed_range.end },
            ]
        },
        OverlapResult::OverlapRight => {
            let offset = source_range.start - seed_range.start;
            let mapped_len = seed_range.end - source_range.start;
            let dest_range_start = dest_range.start + offset;
            vec![
            ]
        },
        OverlapResult::NoOverlap => {
            todo!()
        },
    }
}

#[derive(Debug,Clone)]
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
    fn overlay(&self, other: &Self) -> Vec<Self> {
        match overlap(&self.source_range, &other.source_range) {
            OverlapResult::OverlapFull => {
               vec![
                   Map {
                       source_range: Range { start: self.source_range.start, end: other.source_range.start },
                       dest_range: self.dest_range.clone(),
                   },
                   other.clone(),
                   Map {
                       source_range: Range { start: other.source_range.end, end: self.source_range.end },
                       dest_range: self.dest_range.clone(),
                   },
               ]
            },
            OverlapResult::OverlapLeft => {
                vec![
                    other.clone(),
                    Map {
                        source_range: Range { start: other.source_range.end, end: self.source_range.end },
                        dest_range: self.dest_range.clone(),
                    },
                ]
            },
            OverlapResult::OverlapRight => {
                vec![
                    Map {
                        source_range: Range { start: self.source_range.start, end: other.source_range.start },
                        dest_range: self.dest_range.clone(),
                    },
                    other.clone(),
                ]
            },
            OverlapResult::NoOverlap => {
                vec![self.clone(), other.clone()]
            },
        }
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
    fn lowest_mapped(&self, seed_ranges: &Vec<Range<u64>>) -> u64 {
        seed_ranges.iter()
            .map(|seed_range| {
                self.maps.iter().map(|map| {
                    dbg!(&seed_range, &map);
                    dbg!(match overlap(&seed_range, &map.source_range) {
                        OverlapResult::OverlapFull => map.dest_range.start,
                        OverlapResult::OverlapLeft => map.dest_range.start + (seed_range.start - map.source_range.start),
                        OverlapResult::OverlapRight => map.dest_range.start,
        // TODO: almost there, but this isn't correct.
                        OverlapResult::NoOverlap => seed_range.start,
                    })
                })
                .min().unwrap()
            })
            .min().unwrap()
    }
}

fn parse_seeds(seeds_line: &str) -> Vec<Range<u64>> {
    let seed_num_strs = seeds_line
        .split(": ")
        .last()
        .expect("failed to extract seed num strings");
    let seeds_num_split: Vec<&str> = seed_num_strs.split_whitespace().collect();
    seeds_num_split[..].chunks(2)
        .map(|chunk| {
            let seed_range_start = chunk[0].parse::<u64>().expect("could not parse seed range start");
            let seed_range_len = chunk[1].parse::<u64>().expect("could not parse seed range len");
            seed_range_start..(seed_range_start + seed_range_len)
        })
        .collect::<Vec<_>>()
}

fn parse(input: &str) -> (Vec<Range<u64>>, Mapper) {
    let mut lines = input.lines();
    let seed_ranges = parse_seeds(lines.next().unwrap());
    dbg!(&seed_ranges);

    let mut maps: Vec<Map> = vec![];
    let is_map_line = |line: &&str| !(line.is_empty() || line.ends_with(&":"));
    for line in lines.filter(is_map_line) {
        let new_map = Map::from_str(line);
        // dbg!(&new_map);
        if maps.is_empty() {
            maps.push(new_map);
        } else {
            maps = maps.iter()
                .flat_map(|map| map.overlay(&new_map))
                //.flat_map(|map| dbg!(map.overlay(&new_map)))
                .collect();
        }
        // dbg!(&maps);
    }
    let mapper = Mapper { maps };
    (seed_ranges, mapper)
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let (seed_ranges, mapper) = parse(&input);
    dbg!("here!");
    // dbg!(&seed_ranges, &mapper);
    // dbg!(&seed_ranges);
    mapper.lowest_mapped(&seed_ranges)
        .to_string()
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
