use std::ops::Range;

#[derive(Debug)]
struct Elf {
    assignments: Range<u32>,
}

impl Elf {
    pub fn from_str(s: &str) -> Self {
        let assignments = to_range_inclusive(s);
        Elf { assignments }
    }
    pub fn assignments_overlap(&self, other: &Elf) -> bool {
        (self.assignments.contains(&other.assignments.start)
            || self.assignments.contains(&(other.assignments.end - 1)))
        ||
        (other.assignments.contains(&self.assignments.start)
            || other.assignments.contains(&(self.assignments.end - 1)))
    }
}

fn to_range_inclusive(s: &str) -> Range<u32> {
    let mut parts = s.split('-')
        .map(|s| s.parse().unwrap());
    Range::<u32> {
        start: parts.next().unwrap(),
        end: parts.next().unwrap() + 1,
    }
}

fn to_elves(line: &str) -> (Elf, Elf) {
    let mut iter = line.split(',')
        .map(|s| Elf::from_str(s));
    (iter.next().unwrap(), iter.next().unwrap())
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path)
        .expect("failed to read input");
    let lines = input.lines();

    let elf_pairs = lines.map(|line| to_elves(line));

    elf_pairs.map(|pair| {
        let overlaps = pair.0.assignments_overlap(&pair.1);
        match overlaps {
            true => 1,
            false => 0,
        }
    })
    .sum::<u32>()
    .to_string()
}
