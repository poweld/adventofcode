#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let solution = "CMZ".to_string();
        assert_eq!(solution, part1::solve("data/test_input.txt"))
    }
}
