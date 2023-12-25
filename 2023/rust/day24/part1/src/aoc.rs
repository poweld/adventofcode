mod my {
    #[derive(Debug)]
    pub struct Coord {
        x: i64,
        y: i64,
    }
    impl Coord {
        fn from(s: &str) -> Coord {
            let mut iter = s.split(", ")
                .map(|num| num.trim())
                .map(|num| num.parse::<i64>().unwrap());
            Self {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Velocity {
        x: i64,
        y: i64,
    }
    impl Velocity {
        fn from(s: &str) -> Velocity {
            let mut iter = s.split(", ")
                .map(|num| num.trim())
                .map(|num| num.parse::<i64>().unwrap());
            Self {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
            }
        }
    }

    #[derive(Debug)]
    pub struct Hailstone {
        position: Coord,
        velocity: Velocity,
    }
    impl Hailstone {
        /*
        char get_line_intersection(float p0_x, float p0_y, float p1_x, float p1_y,
            float p2_x, float p2_y, float p3_x, float p3_y, float *i_x, float *i_y)
        {
            float s1_x, s1_y, s2_x, s2_y;
            s1_x = p1_x - p0_x;     s1_y = p1_y - p0_y;
            s2_x = p3_x - p2_x;     s2_y = p3_y - p2_y;

            float s, t;
            s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / (-s2_x * s1_y + s1_x * s2_y);
            t = ( s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / (-s2_x * s1_y + s1_x * s2_y);

            if (s >= 0 && s <= 1 && t >= 0 && t <= 1)
            {
                // Collision detected
                if (i_x != NULL)
                    *i_x = p0_x + (t * s1_x);
                if (i_y != NULL)
                    *i_y = p0_y + (t * s1_y);
                return 1;
            }

            return 0; // No collision
        }
        */
        pub fn intersection(&self, other: &Self, xy_min: u64, xy_max: u64) -> Option<(f64, f64)> {
            let a_initial_coord = Coord {
                x: self.position.x + (self.velocity.x * xy_min as i64),
                y: self.position.y + (self.velocity.y * xy_min as i64),
            };
            let b_initial_coord = Coord {
                x: other.position.x + (other.velocity.x * xy_min as i64),
                y: other.position.y + (other.velocity.y * xy_min as i64),
            };
            let a_final_coord = Coord {
                x: self.position.x + (self.velocity.x * xy_max as i64),
                y: self.position.y + (self.velocity.y * xy_max as i64),
            };
            let b_final_coord = Coord {
                x: other.position.x + (other.velocity.x * xy_max as i64),
                y: other.position.y + (other.velocity.y * xy_max as i64),
            };
            let p0_x = a_initial_coord.x as f64;
            let p0_y = a_initial_coord.y as f64;
            let p1_x = a_final_coord.x as f64;
            let p1_y = a_final_coord.y as f64;
            let p2_x = b_initial_coord.x as f64;
            let p2_y = b_initial_coord.y as f64;
            let p3_x = b_final_coord.x as f64;
            let p3_y = b_final_coord.y as f64;
            let s1_x = p1_x - p0_x;
            let s1_y = p1_y - p0_y;
            let s2_x = p3_x - p2_x;
            let s2_y = p3_y - p2_y;
            let s = (-s1_y * (p0_x - p2_x) + s1_x * (p0_y - p2_y)) / (-s2_x * s1_y + s1_x * s2_y);
            let t = ( s2_x * (p0_y - p2_y) - s2_y * (p0_x - p2_x)) / (-s2_x * s1_y + s1_x * s2_y);
            dbg!(&s, &t);
            if s >= 0.0 && s <= 1.0 && t >= 0.0 && t <= 1.0 {
                return Some((p0_x + (t * s1_x), p0_y + (t * s1_y)))
            }
            None
        }
    }

    #[derive(Debug)]
    pub struct ParseResult {
        pub hailstones: Vec<Hailstone>,
    }

    pub fn parse(input: &str) -> ParseResult {
        let hailstones: Vec<Hailstone> = input.lines()
            .map(|line| {
                let (position, velocity) = line.split_once(" @ ").unwrap();
                let position = Coord::from(position);
                let velocity = Velocity::from(velocity);
                Hailstone { position, velocity }
            })
            .collect();
        ParseResult { hailstones }
    }
}

use my::*;

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { hailstones } = parse(&input);
    let is_test = input_path.contains("test");
    let xy_min: u64 = match is_test {
        true => 7,
        false => 200_000_000_000_000,
    };
    let xy_max: u64 = match is_test {
        true => 27,
        false => 400_000_000_000_000,
    };
    let mut collisions = 0;
    for stone_index in 0..hailstones.len() {
        for other_stone_index in (stone_index + 1)..hailstones.len() {
            let intersection = hailstones[stone_index].intersection(&hailstones[other_stone_index], xy_min, xy_max);
            if intersection.is_some() {
                collisions += 1;
            }
        }
    }
    collisions.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 2.to_string());
    }
}
