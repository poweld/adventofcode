use std::collections::HashMap;

#[derive(Debug)]
struct Network(HashMap<String, (String, String)>);
impl Network {
    fn get_left(&self, location: &String) -> &String {
        &self.0
            .get(location).expect(&format!("failed to get left at location {location}"))
            .0
    }
    fn get_right(&self, location: &String) -> &String {
        &self.0
            .get(location).expect(&format!("failed to get right at location {location}"))
            .1
    }
}

#[derive(Debug)]
struct ParseResult {
    instructions: String,
    network: Network,
}

fn parse(input: &str) -> ParseResult {
    let mut lines = input.lines();
    let instructions = String::from(lines.next().unwrap());
    lines.next();
    let network = lines.map(|line| {
        let mut location_and_destinations = line.split(" = ");
        let location = location_and_destinations.next().unwrap().to_string();
        let mut destinations = location_and_destinations.next().unwrap().to_string();
        destinations.remove(0);
        destinations.pop();
        let mut destinations_split = destinations.split(", ");
        let destinations = (destinations_split.next().unwrap().to_string(), destinations_split.next().unwrap().to_string());
        (location, destinations)
    }).collect();
    ParseResult { instructions, network: Network(network) }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { instructions, network } = parse(&input);
    let mut count = 0u64;
    let mut location = &String::from("AAA");
    let destination = &String::from("ZZZ");
    for instruction in instructions.chars().cycle() {
        if location == destination {
            break;
        }
        match instruction {
            'L' => location = network.get_left(&location),
            'R' => location = network.get_right(&location),
            _ => panic!("unexpected instruction: '{instruction}'"),
        }
        count += 1;
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 6.to_string());
    }
}
