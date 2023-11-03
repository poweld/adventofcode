mod my {
    use std::collections::{HashMap, VecDeque};
    use std::hash::Hash;

    struct Counter<K:Eq+Hash> {
        map: HashMap<K, isize>,
    }
    impl<K:Eq+Hash> Counter<K> {
        pub fn new() -> Self {
            let map = HashMap::new();
            Self { map }
        }
        pub fn add(&mut self, k: K) {
            self.map.entry(k)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
        }
        pub fn sub(&mut self, k: K) {
            self.map.entry(k)
                    .and_modify(|count| *count -= 1)
                    .or_insert(-1);
        }
        pub fn get(&self, k: &K) -> isize {
            *self.map.get(k).unwrap_or(&0)
        }
    }

    pub struct DataReader {
        marker_len: usize,
        deque: VecDeque<char>,
        counter: Counter<char>,
    }
    impl DataReader {
        pub fn new(marker_len: usize) -> Self {
            assert!(marker_len > 0, "marker_len must be > 0");
            let deque = VecDeque::new();
            let counter = Counter::new();
            Self { marker_len, deque, counter }
        }
        pub fn push(&mut self, c: char) {
            self.deque.push_back(c);
            self.counter.add(c);

            if self.deque.len() > self.marker_len {
                let c = self.deque.pop_front().unwrap();
                self.counter.sub(c);
            }
        }
        pub fn all_entries_different(&self) -> bool {
            if self.deque.len() == self.marker_len {
                self.deque.iter()
                    .all(|c| self.counter.get(c) == 1)
            } else {
                false
            }
        }
    }
}

use my::DataReader;

pub fn solve(input_path: &str) -> String {
    let input: String = std::fs::read_to_string(input_path)
        .expect("failed to read input");

    // start of packet marker indicated by four different characters in a row
    let mut reader = DataReader::new(14);
    let mut chars_read = 0;
    'for_funsies: for c in input.chars() {
        chars_read += 1;
        reader.push(c);
        if reader.all_entries_different() { break 'for_funsies }
    }

    let result = chars_read.to_string();

    result
}
