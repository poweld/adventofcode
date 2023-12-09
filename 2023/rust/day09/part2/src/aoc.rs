#[derive(Debug)]
struct ParseResult {
    nums_vecs: Vec<Vec<i64>>,
}

fn parse(input: &str) -> ParseResult {
    let lines = input.lines();
    let parse_to_i64 = |num_str| i64::from_str_radix(num_str, 10)
        .expect(&format!("failed to parse i64 from str: {num_str}"));
    let nums_vecs = lines.map(|line| line.split_whitespace())
        .map(|num_strs| num_strs.map(parse_to_i64).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    ParseResult { nums_vecs }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { nums_vecs } = parse(&input);
    let mut predictions: Vec<i64> = vec![];
    for nums in nums_vecs {
        let mut number_triangle = vec![nums.clone()];
        let mut current_nums = nums;
        loop {
            let deltas = current_nums[..].windows(2)
                .map(|arr| arr[1] - arr[0])
                .collect::<Vec<_>>();
            number_triangle.push(deltas.clone());
            if deltas.iter().all(|delta| delta == &0) {
                break;
            }
            current_nums = deltas;
        }
        number_triangle.reverse();
        let prediction = number_triangle.iter()
            .fold(0, |acc, x| x.first().unwrap() - acc);
        predictions.push(prediction);
    }
    predictions.iter()
        .sum::<i64>()
        .to_string()
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
