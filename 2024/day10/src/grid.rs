/*
Grid:
       C  C  C
       o  o  o
       l  l  l
       0  1  2

       v  v  v

Row0 > 0  1  2 ...

Row1 > 3  4  5 ...

Row2 > 6  7  8 ...
      
       .  .  .
       .  .  .
       .  .  .
*/

// Note that even though grid positions can only ever be 
// unsigned, it's more convenient to use signed values so that
// arithmetic on them is simpler
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Position {
  pub row: isize,
  pub col: isize,
}

pub enum Direction {
  North, Northeast, East, Southeast, South, Southwest, West, Northwest
}

impl Direction {
  #[allow(dead_code)]
  fn value(&self) -> Position {
    match self {
      Self::North => Position { row: -1, col: 0 },
      Self::Northeast => Position { row: -1, col: 1 },
      Self::East => Position { row: 0, col: 1 },
      Self::Southeast => Position { row: 1, col: 1 },
      Self::South => Position { row: 1, col: 0 },
      Self::Southwest => Position { row: 1, col: -1 },
      Self::West => Position { row: 0, col: -1 },
      Self::Northwest => Position { row: -1, col: -1 },
    }
  }
  #[allow(dead_code)]
  pub fn cardinal_directions() -> Vec<Self> {
    vec![Self::North, Self::East, Self::South, Self::West]
  }
  #[allow(dead_code)]
  pub fn intercardinal_directions() -> Vec<Self> {
    vec![Self::Northeast, Self::Southeast, Self::Southwest, Self::Northwest]
  }
  #[allow(dead_code)]
  pub fn all_directions() -> Vec<Self> {
    let mut d = Direction::cardinal_directions();
    d.extend(Direction::intercardinal_directions());
    d
  }
}

impl Position {
  #[allow(dead_code)]
  pub fn cardinal_neighbors(&self) -> Vec<Position> {
    Direction::cardinal_directions().into_iter().map(|direction| {
      self.add(&direction.value())
    }).collect::<Vec<_>>()
  }
  #[allow(dead_code)]
  pub fn intercardinal_neighbors(&self) -> Vec<Position> {
    Direction::intercardinal_directions().into_iter().map(|direction| {
      self.add(&direction.value())
    }).collect::<Vec<_>>()
  }
  #[allow(dead_code)]
  pub fn neighbors(&self) -> Vec<Position> {
    let mut n = self.cardinal_neighbors();
    n.extend(self.intercardinal_neighbors());
    n
  }
  #[allow(dead_code)]
  pub fn add(&self, other: &Position) -> Position {
    Position { row: self.row + other.row, col: self.col + other.col }
  }
}

type GridElement = u8;
#[derive(Debug)]
pub struct Grid {
  pub data: Vec<Vec<GridElement>>,
}

impl Grid {
  #[allow(dead_code)]
  pub fn rows(&self) -> usize {
    self.data.len()
  }
  #[allow(dead_code)]
  pub fn cols(&self) -> usize {
    self.data[0].len()
  }
  #[allow(dead_code)]
  pub fn get(&self, position: &Position) -> Option<GridElement> {
    let Position { row, col } = position;
    if *row >= 0 && *col >= 0 {
      let (row, col): (usize, usize) = (row.unsigned_abs(), col.unsigned_abs());
      self.data.get(row)
        .and_then(|cols| cols.get(col))
        .copied()
    } else {
      None
    }
  }
  #[allow(dead_code)]
  pub fn is_valid_position(&self, position: &Position) -> bool {
    let Position { row, col } = position;
    *row >= 0 && *row < self.rows().try_into().unwrap() && *col >= 0 && *col < self.cols().try_into().unwrap()
  }
  #[allow(dead_code)]
  pub fn positions(&self) -> Vec<Position> {
    (0..self.rows()).flat_map(|row| {
      (0..self.cols()).map(move |col| {
        Position { row: row.try_into().unwrap(), col: col.try_into().unwrap() }
      })
    })
    .collect::<Vec<_>>()
  }
}
