use std::error::Error;
use std::iter;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}
#[derive(Debug)]
struct Forest {
    height_map: Vec<Vec<u8>>,
}
#[derive(Debug)]
struct Sightlines {
    up: Vec<u8>,
    right: Vec<u8>,
    down: Vec<u8>,
    left: Vec<u8>,
}
impl Sightlines {
    fn new() -> Self {
        Self {
            up: vec![],
            right: vec![],
            down: vec![],
            left: vec![],
        }
    }
    fn iter(&self) -> Box<dyn Iterator<Item = &Vec<u8>> + '_> {
        Box::new(iter::once(&self.up)
            .chain(iter::once(&self.right))
            .chain(iter::once(&self.down))
            .chain(iter::once(&self.left)))
    }
}
impl Forest {
    fn from(s: String) -> Self {
        let height_map = s.lines()
            .map(|line| line.chars())
            .map(|chars| {
                chars.map(|c| c.to_digit(10).expect("couldn't parse digit") as u8)  // TODO fix this cast
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Forest { height_map }
    }
    fn get(&self, coord: &Coord) -> u8 {
        self.height_map
            .get(coord.y).unwrap()
            .get(coord.x).unwrap()
            .clone()
    }
    fn dimensions(&self) -> (usize, usize) {
        let x = self.height_map.len();
        let y = self.height_map.get(0).expect("out of bounds").len();
        (x, y)
    }
    fn sightlines(&self, target: &Coord) -> Sightlines {
        let dimensions = self.dimensions();
        let mut sightlines = Sightlines::new();
        for x in 0..target.x {
            sightlines.left.push(self.get(&Coord {x, y: target.y}))
        }
        sightlines.left.reverse();
        for x in (target.x + 1)..dimensions.0 {
            sightlines.right.push(self.get(&Coord {x, y: target.y}))
        }
        for y in 0..target.y {
            sightlines.up.push(self.get(&Coord {x: target.x, y}))
        }
        sightlines.up.reverse();
        for y in (target.y + 1)..dimensions.1 {
            sightlines.down.push(self.get(&Coord {x: target.x, y}))
        }
        sightlines
    }
    fn scenic_score(&self, target: &Coord) -> usize {
        // product of the number of trees that can be seen from target
        let target_height = self.get(target);
        let mut multipliers = vec![];
        for sightline in self.sightlines(&target).iter() {
            let mut visible_trees = 0;
            for tree in sightline.iter() {
                match tree {
                    tree if tree >= &target_height => {
                        visible_trees += 1;
                        break;
                    }
                    _ => visible_trees += 1,
                }
            }
            multipliers.push(visible_trees);
        }
        multipliers.iter().product()
    }
}

pub fn solve(input_path: &str) -> Result<String, Box<dyn Error>> {
    let input: String = std::fs::read_to_string(input_path)?;

    let forest = Forest::from(input);
    let dimensions = forest.dimensions();
    let scenic_scores: Vec<usize> = (0..dimensions.0).flat_map(|x| {
        (0..dimensions.1).map(|y| {
            let target = Coord {x, y};
            forest.scenic_score(&target)
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let max_scenic_score = scenic_scores.iter().max().unwrap();
    Ok(max_scenic_score.to_string())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let result = solve("data/test_input.txt").expect("bad result");
        let solution = 8.to_string();
        assert_eq!(solution, result)
    }
}
