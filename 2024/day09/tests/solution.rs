use day09::*;

#[test]
fn part1_1() {
  let input = read_to_string("input/test_input_1.txt");
  assert_eq!(part1(&input), 1928);
}

#[test]
fn part2_1() {
  let input = read_to_string("input/test_input_2.txt");
  assert_eq!(part2(&input), 2858);
}
