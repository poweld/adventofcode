mod grid;
use crate::grid::*;

use std::collections::HashSet;

pub fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

fn parse(input: &str) -> Grid {
  let data = input.lines()
    .map(|line| {
      line.chars().map(|c| {
        (c as u8) - 48
      })
      .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();
  Grid { data }
}

fn trailheads(grid: &Grid) -> Vec<Position> {
  grid.positions().into_iter()
    .filter(|position| grid.get(&position) == Some(0))
    .collect::<Vec<_>>()
}

fn paths_to_top(position: &Position, grid: &Grid) -> Vec<Vec<Position>> {
  paths_to_top_rec(&position, &grid, &vec![position.clone()])
}

fn paths_to_top_rec(position: &Position, grid: &Grid, acc: &Vec<Position>) -> Vec<Vec<Position>> {
  if acc.len() == 10 {
    vec![acc.clone()]
  } else {
    position.cardinal_neighbors().into_iter()
      .filter(|position| grid.get(&position) == Some(acc.len() as u8))
      .flat_map(|to_visit| {
        let mut acc = acc.clone();
        acc.push(to_visit.clone());
        paths_to_top_rec(&to_visit, &grid, &acc)
      })
      .collect::<Vec<_>>()
  }
}

pub fn part1(input: &str) -> usize {
  let grid = parse(&input);
  let trailheads = trailheads(&grid);
  trailheads.into_iter()
    .map(|trailhead| paths_to_top(&trailhead, &grid))
    .map(|paths| {
      paths.into_iter()
        .map(|path| path[path.len() - 1].clone())
        .collect::<HashSet<_>>()
        .len()
    })
    .sum()
}

pub fn part2(input: &str) -> usize {
  let grid = parse(&input);
  let trailheads = trailheads(&grid);
  trailheads.into_iter()
    .flat_map(|trailhead| paths_to_top(&trailhead, &grid))
    .collect::<Vec<_>>()
    .len()
}
