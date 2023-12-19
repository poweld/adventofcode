use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::{cmp, iter};

fn split_range_lt(range: &RangeInclusive<u64>, lt: &u64) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
    let lt_range = range.start().clone()..=(cmp::min(range.end().clone(), lt - 1));
    let gt_range = (cmp::max(range.start().clone(), *lt))..=range.end().clone();
    (lt_range, gt_range)
}
fn split_range_gt(range: &RangeInclusive<u64>, gt: &u64) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
    let lt_range = range.start().clone()..=(cmp::min(range.end().clone(), *gt));
    let gt_range = (cmp::max(range.start().clone(), gt + 1))..=range.end().clone();
    (lt_range, gt_range)
}

#[derive(Debug)]
enum Rule {
    Filter {
        part: char,
        operator: char,
        rhs: u64,
        jump_match: String,
    },
    JumpDefault(String),
}
impl Rule {
    fn check(&self, part_range: &PartRange) -> (PartRange, PartRange, String) {
        match self {
            Self::Filter { part, operator, rhs, jump_match } => {
                let lhs = part_range.0.get(&part).unwrap();
                return match operator {
                    '<' => {
                        let (lt_range, gt_range) = split_range_lt(&lhs, &rhs);
                        let mut lt_partrange = part_range.clone();
                        lt_partrange.0.insert(*part, lt_range);
                        let mut gt_partrange = part_range.clone();
                        gt_partrange.0.insert(*part, gt_range);
                        (lt_partrange, gt_partrange, jump_match.clone())
                    },
                    '>' => {
                        let (lt_range, gt_range) = split_range_gt(&lhs, &rhs);
                        let mut lt_partrange = part_range.clone();
                        lt_partrange.0.insert(*part, lt_range);
                        let mut gt_partrange = part_range.clone();
                        gt_partrange.0.insert(*part, gt_range);
                        (gt_partrange, lt_partrange, jump_match.clone())
                    },
                    _ => panic!("unexpected operator: {operator}"),
                };
            },
            Self::JumpDefault(jump_match) => (part_range.clone(), part_range.clone(), jump_match.clone()),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct ParseResult {
    workflows: Vec<Workflow>
}

#[derive(Debug, Clone)]
struct PartRange(HashMap<char, RangeInclusive<u64>>);

fn parse(input: &str) -> ParseResult {
    let mut lines = input.lines();
    let workflows: Vec<Workflow> = lines.by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut chars = line.chars();
            let name: String = chars.by_ref()
                .take_while(|c| c != &'{')
                .collect();
            let rules_str: String = chars
                .take_while(|c| c != &'}')
                .collect();
            let rule_strs: Vec<&str> = rules_str.split(',').collect();
            let rules: Vec<Rule> = rule_strs.iter()
                .take(rule_strs.len() - 1)
                .map(|filter_str| {
                    let (remaining, jump_match) = filter_str.split_once(':').unwrap();
                    let jump_match = jump_match.to_string();
                    let mut remaining_chars = remaining.chars();
                    let part: char = remaining_chars.next().unwrap();
                    let operator: char = remaining_chars.next().unwrap();
                    let rhs = remaining_chars.collect::<String>().parse::<u64>().unwrap();
                    Rule::Filter { part, operator, rhs, jump_match }
                })
                .chain(iter::once(Rule::JumpDefault(rule_strs.last().unwrap().to_string())))
                .collect();
            Workflow { name, rules }
        })
        .collect();

    ParseResult { workflows }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { workflows } = parse(&input);
    let workflow_rules: HashMap<String, Workflow> = workflows.into_iter()
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();
    let init_workflow = String::from("in");
    let init_part_range = PartRange(
        "xmas".chars()
            .map(|part| (part, 1..=4000))
            .collect()
    );
    let mut part_range_workflows: Vec<(PartRange, String)> = vec![(init_part_range, init_workflow)];
    let mut completed_part_ranges: Vec<PartRange> = vec![];
    while !part_range_workflows.is_empty() {
        let (mut part_range, workflow) = part_range_workflows.pop().unwrap();
        for rule in workflow_rules.get(&workflow).unwrap().rules.iter() {
            let (new_partrange, remaining_partrange, next_workflow) = rule.check(&part_range);
            match next_workflow.as_str() {
                "A" => completed_part_ranges.push(new_partrange),
                "R" => (),
                _ => part_range_workflows.push((new_partrange, next_workflow)),
            };
            part_range = remaining_partrange;
        }
    }
    completed_part_ranges.into_iter()
        .map(|part_range| part_range.0.values().fold(1u64, |acc, range| {
            acc * cmp::max(1u64, (range.end() - range.start() + 1) as u64)
        }))
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        let result = solve("data/test_input.txt");
        println!("result: {result}");
        assert_eq!(result, 167409079868000u64.to_string());
    }
}
