use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    items: HashSet<char>,
}

fn to_rucksack(line: &str) -> Rucksack {
    Rucksack { items: HashSet::from_iter(line.chars()) }
}

const LOWER_A_ORD: u8 = 'a' as u8;
const LOWER_Z_ORD: u8 = 'z' as u8;
fn ascii_lowercase() -> HashSet<char> {
    let chars = (LOWER_A_ORD..=LOWER_Z_ORD).map(|ord| ord as char);
    HashSet::from_iter(chars)
}
const UPPER_A_ORD: u8 = 'A' as u8;
const UPPER_Z_ORD: u8 = 'Z' as u8;
fn ascii_uppercase() -> HashSet<char> {
    let chars = (UPPER_A_ORD..=UPPER_Z_ORD).map(|ord| ord as char);
    HashSet::from_iter(chars)
}

fn to_priority(c: &char) -> u32 {
    let c_ord = *c as u8;
    if ascii_lowercase().contains(c) {
        (c_ord - LOWER_A_ORD + 1) as u32
    } else if ascii_uppercase().contains(c) {
        (c_ord - UPPER_A_ORD + 27) as u32
    } else {
        panic!("invalid character found: {c}")
    }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path)
        .expect("failed to read input");

    let sets: Vec<_> = input.lines()
                     .map(|line| to_rucksack(line))
                     .map(|sack| sack.items)
                     .collect();
    let set_chunks = sets.chunks(3);
    set_chunks.map(|set_chunk| {
        let mut iter = set_chunk.iter();
        let maybe_chunk_priority = iter.next().map(|set| {
            let intersect_set = iter.fold(set.clone(), |set1, set2| {
                let intersect = set1.intersection(&set2).cloned();
                HashSet::from_iter(intersect)
            });
            intersect_set.iter()
                .map(|c| to_priority(c))
                .sum::<u32>()
        });
        maybe_chunk_priority.expect("did not get a priority sum from chunk")
    })
    .sum::<u32>()
    .to_string()
}
