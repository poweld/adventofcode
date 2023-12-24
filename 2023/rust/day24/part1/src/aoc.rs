mod my {
    #[derive(Debug)]
    pub struct Coord {
        x: isize,
        y: isize,
    }
    impl Coord {
        fn from(s: &str) -> Coord {
            let mut iter = s.split(", ")
                .map(|mut num| num.trim())
                .map(|num| dbg!(num).parse::<isize>().unwrap());
            Self {
                x: iter.next().unwrap(),
                y: iter.next().unwrap(),
            }
        }
    }
    type Velocity = Coord;

    #[derive(Debug)]
    pub struct Hailstone {
        position: Coord,
        velocity: Velocity,
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
    dbg!(&hailstones);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        todo!();
        assert_eq!(result, 0.to_string());
    }
}
