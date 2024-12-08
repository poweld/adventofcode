use std::collections::{HashMap, HashSet};

pub fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

type Position = (isize, isize);
type Grid = Vec<Vec<char>>;

#[derive(Debug)]
struct ParseResults {
  grid: Grid,
}

fn parse(input: &str) -> ParseResults {
  let grid = input.lines().map(|line|
    line.chars().collect::<Vec<_>>()
  ).collect::<Vec<_>>();
  ParseResults { grid }
}

fn find_antennae(grid: &Grid) -> HashMap<char, Vec<Position>> {
  let mut antennae: HashMap<char, Vec<Position>> = HashMap::new();
  for (irow, row) in grid.iter().enumerate() {
    for (icol, c) in row.iter().enumerate() {
      let position = (irow.try_into().unwrap(), icol.try_into().unwrap());
      if c != &'.' {
        antennae.entry(*c)
          .and_modify(|positions| positions.push(position))
          .or_insert(vec![position]);
      }
    }
  }
  antennae
}

fn find_nodes(grid: &Grid, antennae: &HashMap<char, Vec<Position>>, repeating: bool) -> Vec<Position> {
  let (rows, cols) = (grid.len().try_into().unwrap(), grid[0].len().try_into().unwrap());
  let mut nodes = Vec::new();
  for positions in antennae.values() {
    for (iposition, position) in positions.iter().enumerate() {
      for other_position in positions.iter().take(iposition).chain(positions.iter().skip(iposition + 1)) {
        let delta = (other_position.0 - position.0, other_position.1 - position.1);
        let mut other_position = other_position.clone();
        if repeating {
          nodes.push(*position);
          loop {
            let node_position = (other_position.0 + delta.0, other_position.1 + delta.1);
            if node_position.0 >= 0 && node_position.0 < rows && node_position.1 >= 0 && node_position.1 < cols {
              nodes.push(node_position);
              other_position = node_position;
            } else {
              break;
            }
          }
        } else {
          let node_position = (other_position.0 + delta.0, other_position.1 + delta.1);
          if node_position.0 >= 0 && node_position.0 < rows && node_position.1 >= 0 && node_position.1 < cols {
            nodes.push(node_position);
          }
        }
      }
    }
  }
  nodes
}

pub fn part1(input: &str) -> usize {
  let ParseResults { grid } = parse(&input);
  let antennae = find_antennae(&grid);
  let nodes = find_nodes(&grid, &antennae, false);
  let nodes: HashSet<&Position> = HashSet::from_iter(&nodes);
  nodes.len()
}

pub fn part2(input: &str) -> usize {
  let ParseResults { grid } = parse(&input);
  let antennae = find_antennae(&grid);
  let nodes = find_nodes(&grid, &antennae, true);
  let nodes: HashSet<&Position> = HashSet::from_iter(&nodes);
  nodes.len()
}
