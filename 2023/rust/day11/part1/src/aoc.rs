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
    fn manhattan_distance(&self, to: &Coord) -> usize {
        self.row.abs_diff(to.row) + self.col.abs_diff(to.col)
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
        let new_col_count = self.col_count() + empty_cols.iter().filter(|empty| **empty).collect::<Vec<_>>().len();
        let mut new_rows = vec![];
        for row in 0..self.row_count() {
            if self.0[row].iter().all(|col| col == &Pixel::Empty) {
                new_rows.push([Pixel::Empty].repeat(new_col_count));
                new_rows.push([Pixel::Empty].repeat(new_col_count));
            } else {
                let mut new_row = vec![];
                for col in 0..self.col_count() {
                    if empty_cols[col] {
                        new_row.push(Pixel::Empty);
                        new_row.push(Pixel::Empty);
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
    Empty,
    Galaxy,
}

fn parse(input: &str) -> ParseResult {
    let image = Image(input.lines()
        .map(|line| {
            line.chars().map(|c| match c {
                '.' => Pixel::Empty,
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
    let mut distance_sum = 0usize;
    for galaxy_coord in galaxy_coords.iter() {
        for other_galaxy_coord in galaxy_coords.iter() {
            if galaxy_coord == other_galaxy_coord || completed.contains(other_galaxy_coord) {
                continue;
            }
            distance_sum += galaxy_coord.manhattan_distance(other_galaxy_coord);
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
        assert_eq!(result, 374.to_string());
    }
}
