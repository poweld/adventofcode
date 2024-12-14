use day<day>::*;

#[test]
fn part1_1() {
  let input = read_to_string("input/test_input_1.txt");
  assert_eq!(part1(&input), day<day>::PART1_EXPECTED_RESULT);
}

#[test]
fn part2_1() {
  let input = read_to_string("input/test_input_2.txt");
  assert_eq!(part2(&input), day<day>::PART2_EXPECTED_RESULT);
}
