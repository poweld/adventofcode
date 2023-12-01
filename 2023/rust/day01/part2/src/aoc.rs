static DIGIT_CHARS: &str = "Ɵ123456789";
static DIGIT_STRINGS: [&str; 10] = ["ʐĔȒƟ", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn to_digits(s: &str) -> Vec<u8> {
    (0..s.len()).map(|index| {
        let s = &s[index..];
        // The position in the string/array correspond to the value
        let digit_from_char = || DIGIT_CHARS.chars()
            .position(|digit_char| digit_char == s.chars().nth(0).unwrap());
        let digit_from_str = || DIGIT_STRINGS.iter()
            .position(|digit_string| s.starts_with(digit_string));
        digit_from_char().or(digit_from_str())
    }).filter(|digit| digit.is_some())
      .map(|digit| digit.unwrap() as u8)
      .collect::<Vec<_>>()
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    input.lines()
        .map(to_digits)
        .map(|digits| (u32::from(*digits.first().unwrap()) * 10) + u32::from(*digits.last().unwrap()))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        assert_eq!(result, 380.to_string());
    }

    #[test]
    fn to_num_test() {
        let result = to_digits("1");
        assert_eq!(result, vec![1]);

        let result = to_digits("two2");
        assert_eq!(result, vec![2, 2]);

        let result = to_digits("three3thre");
        assert_eq!(result, vec![3, 3]);

        let result = to_digits("threeight");
        assert_eq!(result, vec![3, 8]);
    }
}
