mod my {
    use std::collections::{HashSet, VecDeque};
    use std::hash::Hash;

    type CoordElement = usize;
    #[derive(Debug, PartialEq, Eq, Clone, Hash)]
    pub struct Coord {
        pub row: CoordElement,
        pub col: CoordElement,
    }

    type PlaneChars = Vec<Vec<char>>;
    #[derive(Debug)]
    pub struct Plane {
        chars: PlaneChars,
    }
    impl Plane {
        pub fn start(&self) -> Coord {
            Coord { row: 0, col: 1 }
        }
        pub fn goal(&self) -> Coord {
            let row = self.chars.len() - 1;
            let col = self.chars[0].len() - 2;
            Coord { row, col }
        }
        pub fn get(&self, coord: &Coord) -> char {
            self.chars[coord.row][coord.col]
        }
        pub fn walkable_neighbors(&self, coord: &Coord) -> Vec<Coord> {
            let mut neighbors = vec![
                Coord { row: coord.row + 1, col: coord.col },
                Coord { row: coord.row, col: coord.col + 1 },
            ];
            if let Some(row) = coord.row.checked_sub(1) {
                neighbors.push(Coord { row, col: coord.col } );
            }
            if let Some(col) = coord.col.checked_sub(1) {
                neighbors.push(Coord { row: coord.row, col } );
            }
            neighbors.into_iter()
                .filter(|neighbor| match self.get(&neighbor) {
                    '#' => false,
                    _ => true,
                })
                .collect::<Vec<_>>()
        }
    }

    #[derive(Debug)]
    pub struct ParseResult {
        pub plane: Plane,
    }

    pub fn parse(input: &str) -> ParseResult {
        let plane_chars: PlaneChars = input.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let plane = Plane { chars: plane_chars };
        ParseResult { plane }
    }

    #[derive(Debug)]
    struct LongestPathContext<'longest> {
        next: Coord,
        goal: &'longest Coord,
        seen: &'longest mut HashSet<Coord>,
        longest: &'longest mut usize,
    }

    fn longest_path_rec(context: &mut LongestPathContext, plane: &Plane) {
        if context.next == *context.goal {
            let len = context.seen.len() + 1;
            if len > *context.longest {
                *context.longest = dbg!(len);
            }
            return;
        }

        context.seen.insert(context.next.clone());

        let neighbors = plane.walkable_neighbors(&context.next).into_iter()
            .filter(|neighbor| !context.seen.contains(&neighbor))
            .collect::<Vec<_>>();
        let mut longest = 0;
        for neighbor in neighbors {
            let mut new_context = LongestPathContext {
                next: neighbor.clone(),
                goal: context.goal,
                seen: context.seen,
                longest: context.longest,
            };
            longest_path_rec(&mut new_context, &plane);
        }

        context.seen.remove(&context.next);
    }

    pub fn longest_path(start: Coord, goal: Coord, plane: &Plane) -> usize {
        let mut context = LongestPathContext {
            next: Coord { row: start.row + 1, col: start.col },
            goal: &goal,
            seen: &mut HashSet::new(),
            longest: &mut 0,
        };
        longest_path_rec(&mut context, &plane);
        *context.longest
    }
}

use my::*;

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { plane } = parse(&input);
    let start = plane.start();
    let goal = plane.goal();
    longest_path(start, goal, &plane).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 154.to_string());
    }
}
