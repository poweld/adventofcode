use std::error::Error;

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
    let result = nums.iter()
        .sum::<i32>()
        .to_string();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt").expect("bad result");
        assert_eq!(result, 0.to_string());
    }
}
