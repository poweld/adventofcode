use std::error::Error;

struct Monkey {
    starting_items: Vec<u32>,
    operation: fn(u32) -> u32,
    test: fn(u32, &mut Monkey, &mut Monkey) -> bool,
}

// fn parse(input: String) -> Vec<Monkey> {
//     input.lines()
//         .map(|line| line.split_whitespace().collect::<Vec<_>>())
//         .map(|split| match split[..] {
//             ["addx", n] => Instruction::Addx(i64::from_str_radix(n, 10).unwrap()),
//             ["noop"] => Instruction::Noop,
//             _ => panic!(),
//         })
//         .collect::<Vec<_>>()
// }

fn monkey_business<'a>(x: u32, monkeyA: &mut Monkey, monkeyB: &mut Monkey) -> bool {
    true
}

pub fn solve(input_path: &str) -> Result<String, Box<dyn Error>> {
    let input: String = std::fs::read_to_string(input_path)?;

    let starting_items = vec![];
    let operation = |x| x * x;
    //let test = |x, monkeyA, monkeyB| true;
    let test = monkey_business;
    let mut monkeyA = Monkey { starting_items, operation, test };

    let starting_items = vec![];
    let operation = |x| x * x;
    let test: fn(u32, &mut Monkey, &mut Monkey) -> bool = |x, monkeyA, monkeyB| false;
    //let test = monkey_business;
    let mut monkeyB = Monkey { starting_items, operation, test };

    dbg!((monkeyA.operation)(2));
    dbg!((monkeyA.test)(10, &mut monkeyA, &mut monkeyB));
    dbg!((monkeyB.test)(10, &mut monkeyA, &mut monkeyB));

    todo!()
    // Ok(result)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let result = solve("data/test_input.txt").expect("bad result");
        let solution = 10605.to_string();
        assert_eq!(result, solution)
    }
}
