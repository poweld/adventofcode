#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let solution = "MCD".to_string();
        assert_eq!(solution, part2::solve("data/test_input.txt"))
    }
}
