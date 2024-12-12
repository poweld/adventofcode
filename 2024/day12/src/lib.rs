mod grid;
use crate::grid::*;

use std::collections::HashSet;

pub fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

fn parse(input: &str) -> Grid<char> {
  let data = input.lines().map(|line| {
    line.chars().collect::<Vec<_>>()
  }).collect::<Vec<_>>();
  Grid { data }
}

#[derive(Debug)]
struct Region {
  label: char,
  positions: HashSet<Position>,
  perimeter: Vec<Position>,
}

fn to_regions(grid: &Grid<char>) -> Vec<Region> {
  let mut regions: Vec<Region> = Vec::new();
  for position in grid.positions() {
    if regions.iter().any(|region| region.positions.contains(&position)) {
      // Already part of a region
      continue;
    }
    let new_region_label = grid.get(&position).unwrap();
    let mut new_region_positions = HashSet::new();
    let mut new_region_perimeter = Vec::new();
    let mut frontier = vec![position.clone()];
    // Process the frontier building up the region
    while let Some(new_region_position) = frontier.pop() {
      if new_region_positions.contains(&new_region_position) { continue; }
      new_region_positions.insert(new_region_position.clone());
      for neighbor_position in new_region_position.cardinal_neighbors().into_iter() {
        if new_region_positions.contains(&neighbor_position) { continue; }
        let neighbor_label = grid.get(&neighbor_position);
        match neighbor_label {
          Some(neighbor_label) if Some(neighbor_label) == Some(new_region_label) => frontier.push(neighbor_position),
          _ => { new_region_perimeter.push(neighbor_position.clone()); },
        }
      }
    }
    let new_region = Region { label: new_region_label, positions: new_region_positions, perimeter: new_region_perimeter };
    regions.push(new_region);
  }
  regions
}

pub fn part1(input: &str) -> usize {
  let grid = parse(&input);
  let regions = to_regions(&grid);
  regions.into_iter()
    .map(|region| region.positions.len() * region.perimeter.len())
    .sum()
}

pub fn part2(input: &str) -> usize {
  // TODO blehh tried several things and nothing panned out
  // Need to find the # of walls on the perimeter of each region
  // Ideally follow the perimeter and every turn increases the wall count,
  // but the way the perimeter is currently represented doesn't really gel with that
  1
}
