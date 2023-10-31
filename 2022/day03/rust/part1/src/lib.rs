use std::collections::HashSet;

#[derive(Debug)]
struct Compartment { items: HashSet<char> }

fn to_compartment(s: &str) -> Compartment {
    let items =  HashSet::from_iter(s.chars());
    Compartment { items }
}

#[derive(Debug)]
struct Rucksack {
    compartment1: Compartment,
    compartment2: Compartment,
}

fn to_rucksack(line: &str) -> Rucksack {
    let mid = line.len() / 2;
    let compartment1 = to_compartment(&line[..mid]);
    let compartment2 = to_compartment(&line[mid..]);
    Rucksack { compartment1, compartment2 }
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

    let sacks = input.lines()
                     .map(|line| to_rucksack(line));
    sacks.map(|sack| {
        let intersect = &sack.compartment1.items & &sack.compartment2.items;
        intersect.iter().map(|c| to_priority(c))
                        .sum::<u32>()
    })
    .sum::<u32>()
    .to_string()
}
