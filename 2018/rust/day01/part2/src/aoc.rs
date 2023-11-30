use std::error::Error;
use std::collections::HashSet;

fn parse(input: &String) -> Vec<i32> {
    input.lines()
        .map(|line| {
            let mut chars = line.chars();
            let sign = chars.nth(0).expect("failed to parse sign");
            let num_str = &line[1..];
            let num = i32::from_str_radix(&num_str, 10).expect("failed to parse num");
            match sign {
                '+' => num,
                '-' => -1 * num,
                _ => panic!("unexpected sign: {}", sign),
            }
        })
        .collect()
}

pub fn solve(input_path: &str) -> Result<String, Box<dyn Error>> {
    let input = std::fs::read_to_string(input_path);
    let nums = parse(&input?);
    let mut seen: HashSet<i32> = HashSet::new();
    let mut sum: i32 = 0;
    seen.insert(sum);

    let result;
    'outer: loop {
        for num in &nums {
            sum += num;
            if seen.contains(&sum) {
                result = sum.to_string();
                break 'outer;
            } else {
                seen.insert(sum);
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt").expect("bad result");
        assert_eq!(result, 14.to_string());
    }
}
