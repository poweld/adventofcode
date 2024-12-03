use std::collections::HashMap;

pub fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

fn parse(input: &str) -> Vec<Vec<char>> {
  input.lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>()
}

const XMAS: &str = "XMAS";

fn check_str(
  grid: &Vec<Vec<char>>,
  position: &(usize, usize),
  direction: &(isize, isize),
  remaining: &str,
  mut acc: &mut Vec<(usize, usize)>
) -> Option<Vec<(usize, usize)>> {
  if grid[position.0][position.1] == remaining.chars().nth(0).unwrap() {
    match remaining.len() {
      x if x > 1 => {
        acc.push(*position);
        position.0.checked_add_signed(direction.0)
          .filter(|row| row < &grid.len())
          .and_then(|row| {
            position.1.checked_add_signed(direction.1)
              .filter(|col| col < &grid[0].len())
              .map(|col| (row, col))
          })
          .and_then(|new_position| check_str(&grid, &new_position, &direction, &remaining[1..], &mut acc))
        },
      1 => {
        acc.push(*position);
        Some(acc.to_vec())
      },
      _ => panic!(),
    }
  } else {
    None
  }
}

static ALL_DIRECTIONS: [(isize, isize); 8] = [(0, 1), (1, 0), (1, 1), (0, -1), (-1, 0), (-1, -1), (1, -1), (-1, 1)];
static DIAG_DIRECTIONS: [(isize, isize); 4] = [(1, 1), (-1, -1), (1, -1), (-1, 1)];

fn find_strs(grid: &Vec<Vec<char>>, s: &str, directions: &Vec<(isize, isize)>) -> Vec<Vec<(usize, usize)>> {
  let (rows, cols) = (grid.len(), grid[0].len());
  (0..rows).flat_map(|row| {
    (0..cols).flat_map(move |col| {
      directions.into_iter()
        .map(move |direction| check_str(&grid, &(row, col), &direction, &s, &mut Vec::new()))
    })
  })
  .filter(|result| result.is_some())
  .map(|result| result.unwrap())
  .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> u32 {
  let data = parse(&input);
  find_strs(&data, &XMAS, &ALL_DIRECTIONS.to_vec())
    .len().try_into().unwrap()
}

const MAS: &str = "MAS";

pub fn part2(input: &str) -> u32 {
  let data = parse(&input);
  let mases = find_strs(&data, &MAS, &DIAG_DIRECTIONS.to_vec());
  let mut counter: HashMap<(usize, usize), u32> = HashMap::new();
  for mas in mases {
    counter.entry(mas[1])
      .and_modify(|count| *count += 1)
      .or_insert(1);
  }
  counter.values()
    .filter(|count| *count > &1)
    .count().try_into().unwrap()
}
