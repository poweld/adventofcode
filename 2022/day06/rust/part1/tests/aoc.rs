#[cfg(test)]
mod tests {
    #[test]
    fn solve() {
        let solution = 7.to_string();
        assert_eq!(solution, part1::solve("data/test_input.txt"))
    }
}
