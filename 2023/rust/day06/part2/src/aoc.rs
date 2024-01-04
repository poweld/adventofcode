#[derive(Debug)]
#[allow(dead_code)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse(input: &str) -> Race {
    let mut time_and_distance = input.lines()
        .map(|line| line.split(":"))
        .map(|split| split.last().expect("failed to get last value from split"))
        .map(|nums_str| nums_str.split_whitespace())
        .map(|num_strs| num_strs.fold(String::new(), |acc, s| acc + s))
        .map(|num_str| num_str.parse::<u64>().expect("failed to parse num"));
    let time = time_and_distance.next().unwrap();
    let distance = time_and_distance.next().unwrap();
    Race { time, distance }
}

fn distance(charge_time: u64, time: u64) -> u64 {
    let velocity = charge_time;
    let remaining_time = time - charge_time;
    remaining_time * velocity
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let race = parse(&input);
    (0..=race.time)
        .map(|charge_time| distance(charge_time, race.time))
        .filter(|distance| distance > &race.distance)
        .count()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 71503.to_string());
    }
}
