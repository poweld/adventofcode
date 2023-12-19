use std::collections::HashMap;

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
    fn matches(&self, partmap: &PartMap) -> bool {
        match self {
            Self::Filter { part, operator, rhs, .. } => {
                if let Some(lhs) = partmap.0.get(&part) {
                    return match operator {
                        '<' => lhs < rhs,
                        '>' => lhs > rhs,
                        _ => panic!("unexpected operator: {operator}"),
                    }
                }
                return false;
            },
            Self::JumpDefault(_) => true,
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}
impl Workflow {
    fn to_next_jump(&self, partmap: &PartMap) -> &String {
        let rule = self.rules.iter().find(|rule| rule.matches(&partmap))
            .expect("could not find matching rule");
        match rule {
            Rule::Filter { jump_match, .. } => jump_match,
            Rule::JumpDefault(jump) => jump,
        }
    }
}

#[derive(Debug)]
struct ParseResult {
    workflows: Vec<Workflow>,
    partmaps: Vec<PartMap>,
}

#[derive(Debug)]
struct PartMap(HashMap<char, u64>);

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
                    let Some((remaining, jump_match)) = filter_str.split_once(':')
                        else { panic!("could not split filter string: {filter_str}") };
                    let jump_match = jump_match.to_string();
                    let mut remaining_chars = remaining.chars();
                    let part: char = remaining_chars.next().unwrap();
                    let operator: char = remaining_chars.next().unwrap();
                    let rhs = remaining_chars.collect::<String>().parse::<u64>().unwrap();
                    Rule::Filter { part, operator, rhs, jump_match }
                })
                .chain(std::iter::once(Rule::JumpDefault(rule_strs.last().unwrap().to_string())))
                .collect();
            Workflow { name, rules }
        })
        .collect();

    let partmaps: Vec<PartMap> = lines
        .map(|line| {
            let parts_str: String = line.chars()
                .skip(1)
                .take_while(|c| c != &'}')
                .collect();
            parts_str.split(',')
                .map(|part_str| {
                    let Some((part, value)) = part_str.split_once('=')
                        else { panic!("could not split part string: {part_str}") };
                    let part = part.chars().next().unwrap();
                    let value = value.parse::<u64>().unwrap();
                    (part, value)
                })
                .collect::<HashMap<char, u64>>()
        })
        .map(|hashmap| PartMap(hashmap))
        .collect();

    ParseResult { workflows, partmaps }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let ParseResult { workflows, partmaps } = parse(&input);
    let workflow_rules: HashMap<String, Workflow> = workflows.into_iter()
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();
    partmaps.into_iter()
        .map(|partmap| {
            let mut next_workflow = &String::from("in");
            while workflow_rules.contains_key(next_workflow) {
                let workflow = workflow_rules.get(next_workflow).unwrap();
                next_workflow = workflow.to_next_jump(&partmap);
            }
            match next_workflow.as_str() {
                "A" => partmap.0.values().sum(),
                "R" => 0,
                _ => panic!("unexpected terminal workflow: {next_workflow}"),
            }
        })
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
        assert_eq!(result, 19114.to_string());
    }
}
