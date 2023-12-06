#[derive(Debug)]
#[allow(dead_code)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse(input: &str) -> Vec<Race> {
    let mut times_and_distances = input.lines()
        .map(|line| line.split(":"))
        .map(|split| split.last().expect("failed to get last value from split"))
        .map(|nums_str| nums_str.split_whitespace())
        .map(|num_strs| {
            num_strs.map(|num_str| num_str.parse::<u64>().expect("failed to parse num"))
        });
    let time_iter = times_and_distances.next().unwrap();
    let distance_iter = times_and_distances.next().unwrap();
    time_iter.zip(distance_iter)
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<_>>()
}

fn distance(charge_time: u64, time: u64) -> u64 {
    let velocity = charge_time;
    let remaining_time = time - charge_time;
    remaining_time * velocity
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let races = parse(&input);
    races.iter().map(|race| {
        (0..=race.time)
            .map(|charge_time| distance(charge_time, race.time) > race.distance)
            .filter(|win| *win)
            .count()
    })
    .reduce(|acc, win_count| acc * win_count)
    .unwrap()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 288.to_string());
    }
}
