#[derive(Debug)]
struct HotTubSituation {
    conditions: Vec<Condition>,
    damaged_group_sizes: Vec<u8>,
}
impl HotTubSituation {
    fn possible_arrangements_int(conditions: Vec<Condition>, damaged_group_sizes: Vec<u8>) -> u32 {
        if damaged_group_sizes.len() == 0 {
            // No remaining damaged groups, rest must be all Operational or Unknown
            if conditions.iter().any(|condition| condition == &Condition::Damaged) {
                return 0;
            }
            return 1;
        }
        if conditions.len() == 0 {
            // No remaining conditions, but still have damaged groups
            return 0;
        }
        let next_condition = conditions[0];
        match next_condition {
            Condition::Operational => HotTubSituation::possible_arrangements_int(
                conditions.into_iter()
                    .skip(1)
                    .collect::<Vec<_>>(),
                    damaged_group_sizes,
            ),
            Condition::Damaged => {
                let next_damaged_group_size = damaged_group_sizes[0];
                let damaged_group = conditions.iter().take(next_damaged_group_size as usize)
                    .filter(|condition| condition != &&Condition::Operational)
                    .collect::<Vec<_>>();
                if damaged_group.len() < (next_damaged_group_size as usize) {
                    return 0;
                }
                let mut conditions_to_consume = damaged_group.len();
                if let Some(next_condition) = conditions.iter().nth(damaged_group.len()) {
                    if next_condition == &Condition::Damaged {
                        // After a damaged group, must have a non-damaged condition
                        return 0;
                    }
                    conditions_to_consume += 1;
                }
                HotTubSituation::possible_arrangements_int(
                    conditions.into_iter().skip(conditions_to_consume).collect::<Vec<_>>(),
                    damaged_group_sizes.into_iter().skip(1).collect::<Vec<_>>(),
                )
            },
            Condition::Unknown => {
                // Sum of either case
                let operational_conditions = std::iter::once(Condition::Operational).chain(conditions.clone().into_iter().skip(1)).collect::<Vec<_>>();
                let damanged_conditions = std::iter::once(Condition::Damaged).chain(conditions.into_iter().skip(1)).collect::<Vec<_>>();
                HotTubSituation::possible_arrangements_int(operational_conditions, damaged_group_sizes.clone(),) +
                    HotTubSituation::possible_arrangements_int(damanged_conditions, damaged_group_sizes)
            },
        }
    }
    fn possible_arrangements(&self) -> u32 {
        HotTubSituation::possible_arrangements_int(self.conditions.clone(), self.damaged_group_sizes.clone())
    }
}

#[derive(Debug)]
struct ParseResult {
    hot_tub_situations: Vec<HotTubSituation>,
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

fn parse(input: &str) -> ParseResult {
    let hot_tub_situations =  input.lines()
        .map(|line| {
            let mut condition_and_damaged_strs = line.split_whitespace();
            let condition_str = condition_and_damaged_strs.next().unwrap();
            let damaged_str = condition_and_damaged_strs.next().unwrap();
            let conditions = condition_str.chars()
                .map(|c| match c {
                    '.' => Condition::Operational,
                    '#' => Condition::Damaged,
                    '?' => Condition::Unknown,
                    _ => panic!("unexpected condition char: '{c}'"),
                })
                .collect::<Vec<_>>();
            let damaged_group_sizes = damaged_str.split(',')
                .map(|damaged_num_str| {
                    damaged_num_str.parse::<u8>().expect(&format!("failed to parse into number: {damaged_num_str}"))
                })
                .collect::<Vec<_>>();
            HotTubSituation { conditions, damaged_group_sizes }
        }).collect::<Vec<_>>();
    ParseResult { hot_tub_situations }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { hot_tub_situations } = parse(&input);
    hot_tub_situations.iter()
        .map(|hot_tub_situation| hot_tub_situation.possible_arrangements())
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 21.to_string());
    }
}
