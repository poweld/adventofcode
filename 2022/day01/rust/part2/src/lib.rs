pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path)
        .expect("failed to read input");
    let mut sums = input.lines()
         .fold(vec![0], |mut acc: Vec<u32>, x: &str| match x {
             "" => {
                 acc.push(0);
                 acc
             },
             line => {
                 *acc.last_mut().unwrap() += line.parse::<u32>().unwrap();
                 acc
             },
         });
    sums.sort_by(|a, b| b.cmp(a));  // sort modifies inline and has no return
    sums[..3]
        .iter()
        .sum::<u32>()
        .to_string()
}
