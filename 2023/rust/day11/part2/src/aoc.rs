use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
struct ParseResult {
    image: Image,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}
impl Coord {
    fn manhattan_distance(&self, to: &Coord, image: &Image) -> u64 {
        let row_step = match self.row < to.row {
            true => 1,
            false => -1,
        };
        let col_step = match self.col < to.col {
            true => 1,
            false => -1,
        };
        let mut current_row = self.row as i64;
        let mut current_col = self.col as i64;
        let mut distance = 0u64;

        while current_row != (to.row as i64) {
            current_row += row_step;
            let coord = Coord { row: (current_row as usize), col: self.col };
            distance += match image.get(&coord).unwrap() {
                Pixel::Empty(size) => *size as u64,
                Pixel::Galaxy => 1,
            };
        }
        while current_col != (to.col as i64) {
            current_col += col_step;
            let coord = Coord { row: self.row, col: (current_col as usize) };
            distance += match image.get(&coord).unwrap() {
                Pixel::Empty(size) => *size as u64,
                Pixel::Galaxy => 1,
            };
        }

        distance
    }
}

#[derive(Debug)]
struct Image(Vec<Vec<Pixel>>);
impl Image {
    fn row_count(&self) -> usize {
        self.0.len()
    }
    fn col_count(&self) -> usize {
        self.0[0].len()
    }
    fn coords(&self) -> Vec<Coord> {
        (0..self.row_count()).flat_map(|row| {
            (0..self.col_count()).map(move |col| {
                Coord { row, col }
            })
        }).collect::<Vec<_>>()
    }
    fn galaxy_coords(&self) -> Vec<Coord> {
        self.coords().iter()
            .map(|coord| (coord, self.get(coord).unwrap()))
            .filter(|(_, pixel)| *pixel == &Pixel::Galaxy)
            .map(|(coord, _)| *coord)
            .collect::<Vec<_>>()
    }
    fn get(&self, coord: &Coord) -> Option<&Pixel> {
        self.0.get(coord.row).and_then(|row| row.get(coord.col))
    }
    fn expand(&self) -> Self {
        let mut empty_cols = [true].repeat(self.col_count());
        let coords = self.coords();
        for coord in coords.iter() {
            let maybe_pixel = self.get(&coord);
            if let Some(Pixel::Galaxy) = maybe_pixel {
                empty_cols[coord.col] = false;
            }
        }
        let mut new_rows = vec![];
        let big_empty_pixel = Pixel::Empty(1_000_000);
        for row in 0..self.row_count() {
            if self.0[row].iter().all(|col| match col {
                Pixel::Empty(_) => true,
                _ => false,
            }) {
                new_rows.push([big_empty_pixel].repeat(self.col_count()));
            } else {
                let mut new_row = vec![];
                for col in 0..self.col_count() {
                    if empty_cols[col] {
                        new_row.push(big_empty_pixel);
                    } else {
                        let coord = Coord { row, col };
                        new_row.push(*self.get(&coord).unwrap());
                    }
                }
                new_rows.push(new_row);
            }
        }
        Self(new_rows)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
enum Pixel {
    Empty(u32),
    Galaxy,
}

fn parse(input: &str) -> ParseResult {
    let image = Image(input.lines()
        .map(|line| {
            line.chars().map(|c| match c {
                '.' => Pixel::Empty(1),
                '#' => Pixel::Galaxy,
                _ => panic!("unexpected pixel char: {c}"),
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>());
    ParseResult { image }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { image } = parse(&input);
    let image = image.expand();
    let galaxy_coords = image.galaxy_coords();
    let mut completed: HashSet<&Coord> = HashSet::new();
    let mut distance_sum = 0;
    for galaxy_coord in galaxy_coords.iter() {
        for other_galaxy_coord in galaxy_coords.iter() {
            if galaxy_coord == other_galaxy_coord || completed.contains(other_galaxy_coord) {
                continue;
            }
            distance_sum += galaxy_coord.manhattan_distance(other_galaxy_coord, &image);
        }
        completed.insert(galaxy_coord);
    }
    distance_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 82000210.to_string());
    }
}
