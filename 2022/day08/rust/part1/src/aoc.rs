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
            .get(coord.x).unwrap()
            .get(coord.y).unwrap()
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
        for x in (target.x + 1)..dimensions.0 {
            sightlines.right.push(self.get(&Coord {x, y: target.y}))
        }
        for y in 0..target.y {
            sightlines.up.push(self.get(&Coord {x: target.x, y}))
        }
        for y in (target.y + 1)..dimensions.1 {
            sightlines.down.push(self.get(&Coord {x: target.x, y}))
        }
        sightlines
    }
    fn visible(&self, target: &Coord) -> bool {
        let target_height = self.get(target);
        self.sightlines(target)
            .iter()
            .any(|sightline| {
                sightline.len() == 0 ||
                sightline.iter().all(|tree| tree < &target_height)
            })
    }
}

pub fn solve(input_path: &str) -> Result<String, Box<dyn Error>> {
    let input: String = std::fs::read_to_string(input_path)?;

    let forest = Forest::from(input);
    let dimensions = forest.dimensions();
    let mut visible_count = 0;
    for x in 0..dimensions.0 {
        for y in 0..dimensions.1 {
            let target = Coord {x, y};
            match forest.visible(&target) {
                true => visible_count += 1,
                false => (),
            }
        }
    }

    Ok(visible_count.to_string())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let result = solve("data/test_input.txt").expect("bad result");
        let solution = 21.to_string();
        assert_eq!(solution, result)
    }
}
