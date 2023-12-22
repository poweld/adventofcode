mod my {
    use std::cmp::Ordering;
    use std::collections::HashSet;

    type CoordElement = usize;
    #[derive(Debug, PartialEq, Eq)]
    struct Coord3d {
        x: CoordElement,
        y: CoordElement,
        z: CoordElement,
    }
    impl Coord3d {
        fn from(s: &str) -> Self {
            let coord_elements: Vec<CoordElement> = s.split(',')
                .map(|coord_elem| coord_elem.parse::<CoordElement>().unwrap())
                .collect();
            let [x, y, z] = coord_elements[0..3] else { panic!("failed while extracting Coord3d elements") };
            Coord3d { x, y, z }
        }
    }

    #[derive(Debug, Eq)]
    struct BrickPosition {
        start: Coord3d,
        end: Coord3d,
    }
    impl Ord for BrickPosition {
        fn cmp(&self, other: &Self) -> Ordering {
            self.start.z.cmp(&other.start.z)
        }
    }
    impl PartialOrd for BrickPosition {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(&other))
        }
    }
    impl PartialEq for BrickPosition {
        fn eq(&self, other: &Self) -> bool {
            self.start.z == other.start.z
        }
    }

    type Brick = usize;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct BrickData {
        position: BrickPosition,
    }

    static TOWER_XY_DIMENSION: usize = 10;
    static TOWER_Z_DIMENSION: usize = 300;
    #[derive(Debug)]
    pub struct Tower {
        bricks_data: Vec<BrickData>,
        bricks: Vec<Vec<Vec<Option<Brick>>>>,
    }
    impl Tower {
        pub fn new() -> Self {
            let bricks_data = vec![];
            let default_brick: Option<Brick> = None;
            let default_z_column: Vec<Option<Brick>> = (0..TOWER_Z_DIMENSION).map(|_z| default_brick).collect();
            let bricks = (0..TOWER_XY_DIMENSION).map(|_x| {
                (0..TOWER_XY_DIMENSION).map(|_y| {
                    default_z_column.clone()
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
            Self { bricks_data, bricks }
        }
        fn get_brick(&self, coord: &Coord3d) -> Option<Brick> {
            self.bricks[coord.x][coord.y][coord.z]
        }
        fn set_brick(&mut self, coord: &Coord3d, brick: Brick) {
            self.bricks[coord.x][coord.y][coord.z] = Some(brick);
        }
        fn find_z_for(&self, brick_data: &BrickData) -> CoordElement {
            let x_range = brick_data.position.start.x..=brick_data.position.end.x;
            let y_range = brick_data.position.start.y..=brick_data.position.end.y;
            let mut z = brick_data.position.start.z;
            let rays: Vec<Coord3d> = x_range
                .flat_map(|x| y_range.clone()
                    .map(move |y| Coord3d { x, y, z })
                ).collect();
            while z > 0 {
                for ray in &rays {
                    if self.get_brick(&Coord3d { z: z - 1, ..*ray }).is_some() {
                        return z;
                    }
                }
                z -= 1;
            }
            z
        }
        fn get_supported_by(&self, brick: &Brick) -> HashSet<Brick> {
            let brick_data = &self.bricks_data[*brick];
            let mut supported_bricks: HashSet<Brick> = HashSet::new();
            let x_range = brick_data.position.start.x..=brick_data.position.end.x;
            let y_range = brick_data.position.start.y..=brick_data.position.end.y;
            let z = brick_data.position.end.z + 1;
            for x in x_range {
                for y in y_range.clone() {
                    let supported_brick = self.get_brick(&Coord3d { x, y, z });
                    if supported_brick.is_some() {
                        supported_bricks.insert(supported_brick.unwrap());
                    }
                }
            }
            supported_bricks
        }
        fn get_supporting(&self, brick: &Brick) -> HashSet<Brick> {
            let brick_data = &self.bricks_data[*brick];
            let mut supporting_bricks: HashSet<Brick> = HashSet::new();
            let x_range = brick_data.position.start.x..=brick_data.position.end.x;
            let y_range = brick_data.position.start.y..=brick_data.position.end.y;
            let z = brick_data.position.start.z - 1;
            for x in x_range {
                for y in y_range.clone() {
                    let supporting_brick = self.get_brick(&Coord3d { x, y, z });
                    if supporting_brick.is_some() {
                        supporting_bricks.insert(supporting_brick.unwrap());
                    }
                }
            }
            supporting_bricks
        }
        pub fn can_be_disintegrated(&self) -> HashSet<Brick> {
            let mut can_be_disintegrated: HashSet<Brick> = HashSet::new();
            for brick in 0..self.bricks_data.len() {
                let supported_bricks = self.get_supported_by(&brick);
                if supported_bricks.iter().all(|supported_brick| self.get_supporting(supported_brick).len() > 1) {
                    can_be_disintegrated.insert(brick);
                }
            }
            can_be_disintegrated
        }
        pub fn add_brick(&mut self, brick_data: BrickData) {
            let new_z_start = self.find_z_for(&brick_data);
            let new_z_end = new_z_start + brick_data.position.end.z - brick_data.position.start.z;
            let brick = self.bricks_data.len();
            let x_range = brick_data.position.start.x..=brick_data.position.end.x;
            let y_range = brick_data.position.start.y..=brick_data.position.end.y;
            let z_range = new_z_start..=new_z_end;
            for x in x_range {
                for y in y_range.clone() {
                    for z in z_range.clone() {
                        self.set_brick(&Coord3d { x, y, z }, brick);
                    }
                }
            }
            let new_brick_position = BrickPosition {
                start: Coord3d {
                    z: new_z_start,
                    ..brick_data.position.start
                },
                end: Coord3d {
                    z: new_z_end,
                    ..brick_data.position.end
                },
            };
            self.bricks_data.push(BrickData { position: new_brick_position });
        }
    }

    #[derive(Debug)]
    pub struct ParseResult {
        pub bricks_data: Vec<BrickData>,
    }

    pub fn parse(input: &str) -> ParseResult {
        let bricks_data: Vec<BrickData> = input.lines()
            .map(|line| line.split_once('~').unwrap())
            .map(|(start_str, end_str)| (Coord3d::from(start_str), Coord3d::from(end_str)))
            .map(|(start, end)| BrickPosition { start, end })
            .map(|position| BrickData { position })
            .collect();
        ParseResult { bricks_data }
    }
}

use my::*;

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { mut bricks_data } = parse(&input);
    bricks_data.sort();
    let mut tower = Tower::new();
    for brick_data in bricks_data {
        tower.add_brick(brick_data);
    }
    tower.can_be_disintegrated().len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 5.to_string());
    }
}
