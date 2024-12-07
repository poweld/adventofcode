use std::collections::HashSet;

pub fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

type GridElement = char;
type Grid = Vec<Vec<GridElement>>;
type Position = (usize, usize);
type Direction = (isize, isize);

const UP: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (0, 1);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
fn rotate_90(direction: Direction) -> Direction {
  match direction {
    UP => RIGHT,
    RIGHT => DOWN,
    DOWN => LEFT,
    LEFT => UP,
    _ => panic!(),
  }
}

enum StepResult {
  Success(Position),
  Collision(Position),
  FallOff,
}

fn step(position: Position, direction: Direction, grid: &Grid) -> StepResult {
  position.0.checked_add_signed(direction.0)
    .filter(|row| row < &grid.len())
    .and_then(|row| {
      position.1.checked_add_signed(direction.1)
      .filter(|col| col < &grid[0].len())
      .map(|col| {
        (row, col)
      })
    })
    // Check for collision
    .map(|new_position| {
      match grid[new_position.0][new_position.1] {
        '#' => StepResult::Collision(new_position),
        _ => StepResult::Success(new_position),
      }
    })
    .unwrap_or(StepResult::FallOff)
}

#[derive(Debug)]
struct Game {
  pub ray: Ray,
  pub offshoots: Vec<(Ray, Position)>,
  pub grid: Grid,
  pub visited: HashSet<Ray>,
}

impl Game {
  fn tick(&mut self) -> bool {
    match step(self.ray.position, self.ray.direction, &self.grid) {
      StepResult::Success(position) => {
        self.ray.position = position;
        self.visited.insert(self.ray.clone());
        true
      },
      StepResult::Collision(_) => {
        self.ray.direction = rotate_90(self.ray.direction);
        true
      },
      StepResult::FallOff => false,
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Ray {
  pub position: Position,
  pub direction: Direction,
}

fn parse(input: &str) -> Game {
  let grid = input.lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Grid>();
  let mut position_direction = None;
  for row in 0..grid.len() {
    if position_direction.is_some() { break; }
    for col in 0..grid[0].len() {
      match grid[row][col] {
        '.' | '#' => (),
        '^' => { position_direction = Some(((row, col), UP)); break },
        '>' => { position_direction = Some(((row, col), RIGHT)); break },
        'v' => { position_direction = Some(((row, col), DOWN)); break },
        '<' => { position_direction = Some(((row, col), LEFT)); break },
        _ => panic!(),
      }
    }
  }
  let (position, direction) = position_direction.unwrap();
  let ray = Ray { position, direction };
  Game { ray: ray.clone(), offshoots: Vec::new(), grid: grid, visited: HashSet::from([ray]) }
}

pub fn part1(input: &str) -> u32 {
  let mut game = parse(&input);
  let mut visited = HashSet::from([game.ray.position]);
  while game.tick() {
    visited.insert(game.ray.position);
  }
  visited.len().try_into().unwrap()
}

pub fn part2(input: &str) -> u32 {
  // let mut game = parse(&input);
  // let mut visited = HashSet::from([game.ray.clone()]);
  // let mut loop_obstructions = HashSet::new();
  // while game.tick() {
  //   visited.insert(game.ray.clone());
  //   let rotated_ray = Ray { direction: rotate_90(game.ray.direction), ..game.ray };
  //   if let Some((position, _)) = step(rotated_ray.position, rotated_ray.direction, &game.grid) {
  //     let rotated_ray = Ray { position: position, ..rotated_ray };
  //     if visited.contains(&rotated_ray) {
  //       if let Some((position, _)) = step(game.ray.position, game.ray.direction, &game.grid) {
  //         loop_obstructions.insert(position);
  //       }
  //     }
  //   }
  // }
  // dbg!(&visited, &loop_obstructions);
  // loop_obstructions.len().try_into().unwrap()
  1
}
