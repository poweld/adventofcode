use std::collections::{HashMap, HashSet};

pub fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

type PageId = u32;

#[derive(Debug)]
struct ParseResults {
  pub pages: HashMap<PageId, Page>,
  pub updates: Vec<Vec<PageId>>,
}

impl ParseResults {
  fn new() -> ParseResults {
    ParseResults { pages: HashMap::new(), updates: Vec::new() }
  }
}

fn parse(input: &str) -> ParseResults {
  let mut results = ParseResults::new();
  let mut lines = input.lines();
  // Page rules
  while let Some(line) = lines.next() {
    if line.is_empty() {
      break;
    }
    let rule = line.split_once('|').unwrap();
    let (dependency_id, dependent_id) = (rule.0.parse().unwrap(), rule.1.parse().unwrap());
    results.pages.entry(dependent_id)
      .and_modify(|dependent| { dependent.dependencies.insert(dependency_id); })
      .or_insert({
        let mut dependent = Page::new();
        dependent.dependencies.insert(dependency_id);
        dependent
      });
  }
  // Updates
  while let Some(line) = lines.next() {
    let page_ids = line.split(',')
      .map(|s| s.parse().unwrap())
      .collect::<Vec<_>>();
    results.updates.push(page_ids);
  }
  results
}

#[derive(Debug)]
struct Page {
  pub dependencies: HashSet<PageId>,
}

impl Page {
  fn new() -> Page {
    Page { dependencies: HashSet::new() }
  }
}

// Paritition updates into (valid, invalid)
fn partition_updates(updates: Vec<Vec<PageId>>, pages: &HashMap<PageId, Page>) -> (Vec<Vec<PageId>>, Vec<Vec<PageId>>) {
  updates.into_iter()
    .fold((Vec::new(), Vec::new()), |(mut valid, mut invalid), update| {
      let mut remaining = update.iter().map(|x| x.to_owned()).collect::<HashSet<_>>();
      if update.iter().all(|page_id| {
        if !pages.contains_key(&page_id) || remaining.is_disjoint(&pages[page_id].dependencies) {
          remaining.remove(page_id);
          true
        } else {
          false
        }
      }) {
        valid.push(update);
        (valid, invalid)
      } else {
        invalid.push(update);
        (valid, invalid)
      }
    })
}

pub fn part1(input: &str) -> u32 {
  let ParseResults { pages, updates } = parse(&input);
  let (valid_updates, _) = partition_updates(updates, &pages);
  valid_updates.into_iter()
    .map(|update| update[update.len()/2])
    .sum()
}

fn repair(mut update: Vec<u32>, pages: &HashMap<PageId, Page>) -> Vec<u32> {
  let mut repaired_update = Vec::new();
  let mut remaining = update.iter().map(|x| x.to_owned()).collect::<HashSet<_>>();
  while update.len() > 0 {
    let inext = update.iter().position(|page_id| {
      !pages.contains_key(&page_id) || remaining.is_disjoint(&pages[page_id].dependencies)
    }).unwrap();
    let next = update[inext];
    repaired_update.push(next);
    remaining.remove(&next);
    update.remove(inext);
  }
  repaired_update
}

pub fn part2(input: &str) -> u32 {
  let ParseResults { pages, updates } = parse(&input);
  let (_, invalid_updates) = partition_updates(updates, &pages);
  invalid_updates.into_iter()
    .map(|update| repair(update, &pages))
    .map(|update| update[update.len()/2])
    .sum()
}
