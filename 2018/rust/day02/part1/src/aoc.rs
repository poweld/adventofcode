use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Counter<T: Hash> {
    pub count_map: HashMap<T, i64>
}
impl<T: Hash + Eq> Counter<T> {
    pub fn new() -> Self {
        let count_map = HashMap::new();
        Self { count_map }
    }
    pub fn incr(&mut self, x: T) {
        self.count_map.entry(x)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
}

pub fn invert_hashmap<K: Hash + Clone, V: Hash + Eq + Clone>(hashmap: &HashMap<K, V>) -> HashMap<V, K> {
    let mut inverted: HashMap<V, K> = HashMap::new();
    for (k, v) in hashmap.iter() {
        inverted.insert(v.clone(), k.clone());
    }
    inverted
    // hashmap.iter()
    //     .map(|(k, v)| (v.clone(), k.clone()))
    //     .collect()
}

pub fn parse(input: &String) -> Vec<String> {
    input.lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).expect("failed to read input path: {input_path}");
    let lines = parse(&input);
    let counters: Vec<_> = lines.iter().map(|line| {
        let mut counter = Counter::new();
        for c in line.chars() {
            counter.incr(c);
        }
        counter
    }).collect();
    dbg!(&counters);
    let inverted_count_maps: Vec<HashMap<i64, char>> = counters.into_iter()
        .map(|counter| invert_hashmap(&counter.count_map)).collect();
    dbg!(&inverted_count_maps);
    let doubles = inverted_count_maps.iter()
        .filter(|m| m.contains_key(&2))
        .count();
    dbg!(doubles);
    let triples = inverted_count_maps.iter()
        .filter(|m| m.contains_key(&3))
        .count();
    dbg!(triples);
    // let triples = inverted_count_maps.get(3).expect("couldn't find triples in inverted hash map");
    (doubles * triples).to_string()
    // todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 12.to_string());
    }

    #[test]
    fn invert_test() {
        let hashmap: HashMap<char, i64> = HashMap::from([('a', 123), ('b', 456)]);
        let result: HashMap<i64, char> = invert_hashmap(&hashmap);
        let expected: HashMap<i64, char> = HashMap::from([(123, 'a'), (456, 'b')]);
        assert_eq!(result, expected)
    }
}
