pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path)
        .expect("failed to read input");
    input.lines()
         .fold(vec![0], |mut acc: Vec<u32>, x: &str| match x {
             "" => {
                 acc.push(0);
                 acc
             },
             line => {
                 *acc.last_mut().unwrap() += line.parse::<u32>().unwrap();
                 acc
             },
         })
         .iter()
         .max().expect("couldn't get max")
         .to_string()
}
