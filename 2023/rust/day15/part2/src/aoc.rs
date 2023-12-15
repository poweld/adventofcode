fn hash(key: &String) -> usize {
    let result = key.chars()
        .map(|c| c as usize)
        .fold(0, |acc, ascii_val| {
            ((acc + ascii_val) * 17) % 256
        });
    result
}

#[derive(Debug, Clone)]
struct KeyVal {
    key: String,
    val: u8,
}

#[derive(Debug)]
struct HashMap(Vec<Vec<KeyVal>>);
impl HashMap {
    fn new() -> Self {
        let buckets = vec![vec![]; 256];
        Self(buckets)
    }
    fn insert(&mut self, key: String, val: u8) {
        let bucket = &mut self.0[hash(&key)];
        let position = bucket.iter().position(|keyval| keyval.key == key);
        if let Some(position) = position {
            bucket[position] = KeyVal { key, val };
        } else {
            bucket.push(KeyVal { key, val });
        }
    }
    fn remove(&mut self, key: &String) {
        let bucket = &mut self.0[hash(&key)];
        let position = bucket.iter().position(|keyval| keyval.key == *key);
        if let Some(position) = position {
            bucket.remove(position);
        }
    }
    fn score(&self) -> u64 {
        self.0.iter()
            .enumerate()
            .flat_map(|(bucket_index, bucket)| {
                bucket.iter()
                    .enumerate()
                    .map(move |(keyval_index, keyval)| {
                        ((bucket_index as u64) + 1) * ((keyval_index as u64) + 1) * (keyval.val as u64)
                    })
            })
            .sum::<u64>()
    }
}

#[derive(Debug)]
struct Instruction {
    key: String,
    val: Option<u8>,
    operator: char,
}

#[derive(Debug)]
struct InitSequence(Vec<Instruction>);

#[derive(Debug)]
struct ParseResult {
    init_sequence: InitSequence,
}

fn parse(input: &str) -> ParseResult {
    let instructions = input.lines()
        .flat_map(|line| line.split(","))
        .map(|instruction_str| {
            let chars = instruction_str.chars().collect::<Vec<_>>();
            let operator_index = chars.iter().position(|c| c == &'=').or(chars.iter().position(|c| c == &'-')).expect("could not find operator");
            let operator: char = chars[operator_index];
            let key: String = chars[..operator_index].iter().collect();
            let val_str: String = chars[(operator_index + 1)..].iter().collect();
            let val: Option<u8> = val_str.parse().ok();
            Instruction { key, val, operator }
        })
        .collect::<Vec<_>>();
    ParseResult { init_sequence: InitSequence(instructions) }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { init_sequence } = parse(&input);
    let mut hashmap = HashMap::new();
    for instruction in init_sequence.0.iter() {
        match instruction.operator {
            '-' => hashmap.remove(&instruction.key),
            '=' => hashmap.insert(instruction.key.clone(), instruction.val.unwrap().clone()),
            _ => panic!("unknown operator: {}", instruction.operator),
        }
    }
    hashmap.score().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 145.to_string());
    }
}
