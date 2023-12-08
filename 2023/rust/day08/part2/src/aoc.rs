use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{SyncSender, Receiver};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

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
    fn locations(&self) -> HashSet<&String> {
        self.0.keys().collect()
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
    let start_suffix = &"A";
    let destination_suffix = &"Z";
    let mut start_locations: Vec<String> = vec![];
    for location in network.locations() {
        if location.ends_with(start_suffix) {
            start_locations.push(location.to_string());
        }
    }
    let network = Arc::new(network);

    let nthreads = start_locations.len();
    // let (tx, rx): (SyncSender<u64>, Receiver<u64>) = mpsc::sync_channel(1);
    let transceivers: Vec<(SyncSender<u64>, Receiver<u64>)> = (0..nthreads)
        .map(|_| mpsc::sync_channel(1)).collect();
    let mut children = vec![];
    for thread_id in 0..nthreads {
        let start_location = start_locations[thread_id].clone();
        let instructions = instructions.clone();
        let network = Arc::clone(&network);
        // let thread_tx = tx.clone();
        let thread_tx = transceivers[thread_id].0.clone();
        let child = thread::spawn(move || {
            let mut location: &String = &start_location;
            let mut count = 0u64;
            for instruction in instructions.chars().cycle() {
                if location.ends_with(destination_suffix) {
                    //println!("thread {thread_id} sending count {count}");
                    thread_tx.send(count).unwrap();
                    //println!("thread {thread_id} unblocked!");
                }
                match instruction {
                    'L' => location = network.get_left(&location),
                    'R' => location = network.get_right(&location),
                    _ => panic!("unexpected instruction: '{instruction}'"),
                }
                count += 1;
            }
        });
        children.push(child);
    }

    let mut maybe_max_count: Option<u64> = None;
    let mut thread_id_count: HashMap<usize, u64> = HashMap::new();
    loop {
        for (thread_id, (_, rx)) in transceivers.iter().enumerate() {
            if let Some(max_count) = maybe_max_count {
                if let Some(count) = thread_id_count.get(&thread_id) {
                    if count == &max_count {
                        continue;
                    }
                }
                let count = rx.recv().unwrap();
                thread_id_count.insert(thread_id, count);
                if count > max_count {
                    maybe_max_count = Some(count);
                }
            } else {
                let count = rx.recv().unwrap();
                maybe_max_count = Some(count);
                thread_id_count.insert(thread_id, count);
            }
        }
        if (
            thread_id_count.len() == nthreads &&
            thread_id_count.values().all(|count| count == &maybe_max_count.unwrap())
        ) {
            break;
        }
    }
    dbg!(maybe_max_count);
    dbg!(thread_id_count);

    todo!()
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
