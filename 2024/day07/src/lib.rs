pub fn read_to_string(path: &str) -> String {
  let input = std::fs::read_to_string(path).unwrap();
  let input = input.trim();
  String::from(input)
}

type Operand = u64;

#[derive(Debug)]
struct PartialEquation {
  pub result: Operand,
  pub operands: Vec<Operand>,
}

fn parse(input: &str) -> Vec<PartialEquation> {
  input.lines().map(|line| {
    let (result, operands) = line.split_once(": ").unwrap();
    let result = result.parse().unwrap();
    let operands = operands.split(" ")
      .map(|operand| operand.parse().unwrap())
      .collect::<Vec<_>>();
    PartialEquation { result, operands }
  })
  .collect::<Vec<_>>()
}

fn equation_results(operands: &Vec<Operand>, operators: Vec<&str>) -> Vec<Operand> {
  let mut operands = operands.iter();
  let mut results = vec![operands.next().unwrap().clone()];
  while let Some(right) = operands.next() {
    results = results.into_iter().flat_map(|left| {
      operators.iter().map(move |operator| {
        match *operator {
          "+" => left + right,
          "*" => left * right,
          "||" => format!("{}{}", &left, &right).parse().unwrap(),
          _ => panic!(),
        }
      })
    }).collect()
  }
  results
}

pub fn part1(input: &str) -> u64 {
  let partial_equations = parse(&input);
  partial_equations.into_iter().map(|partial_equation| {
    let results = equation_results(&partial_equation.operands, vec!["+", "*"]);
    if results.contains(&partial_equation.result) {
      Some(partial_equation.result)
    } else {
      None
    }
  })
  .map(|x| x.unwrap_or(0))
  .sum()
}

pub fn part2(input: &str) -> u64 {
  let partial_equations = parse(&input);
  partial_equations.into_iter().map(|partial_equation| {
    let results = equation_results(&partial_equation.operands, vec!["+", "*", "||"]);
    if results.contains(&partial_equation.result) {
      Some(partial_equation.result)
    } else {
      None
    }
  })
  .map(|x| x.unwrap_or(0))
  .sum()
}
