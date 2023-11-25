use std::error::Error;
use std::collections::HashSet;

struct Monkey {
    items: Vec<u128>,
    operation: Box<dyn Fn(u128) -> u128>,
    test: Box<dyn Fn(u128) -> (u128, usize)>,
}
impl Monkey {
    fn to_string(&self, monkey_number: usize) -> String {
        format!("Monkey {monkey_number}: {0:?}", self.items)
    }
}

struct MonkeyBuilder {
    items: Option<Vec<u128>>,
    operation: Option<Box<dyn Fn(u128) -> u128>>,
    test: Option<Box<dyn Fn(u128) -> (u128, usize)>>,
}
impl MonkeyBuilder {
    fn new() -> Self {
        Self { items: None, operation: None, test: None }
    }
    fn with_items(self, items: Vec<u128>) -> Self {
        Self { items: Some(items), ..self }
    }
    fn with_operation(self, operation: Box<dyn Fn(u128) -> u128>) -> Self {
        Self { operation: Some(operation), ..self }
    }
    fn with_test(self, test: Box<dyn Fn(u128) -> (u128, usize)>) -> Self {
        Self { test: Some(test), ..self }
    }
    fn build(self) -> Monkey {
        Monkey {
            items: self.items.unwrap(),
            operation: self.operation.unwrap(),
            test: self.test.unwrap(),
        }
    }
}

struct TestBuilder {
    divisor: Option<u128>,
    true_monkey: Option<usize>,
    false_monkey: Option<usize>,
}
impl TestBuilder {
    fn new() -> Self {
        Self { divisor: None, true_monkey: None, false_monkey: None }
    }
    fn with_divisor(self, divisor: u128) -> Self {
        Self { divisor: Some(divisor), ..self }
    }
    fn with_true_monkey(self, true_monkey: usize) -> Self {
        Self { true_monkey: Some(true_monkey), ..self }
    }
    fn with_false_monkey(self, false_monkey: usize) -> Self {
        Self { false_monkey: Some(false_monkey), ..self }
    }
    fn build(self) -> Box<dyn Fn(u128) -> (u128, usize)> {
        generate_test(self.divisor.unwrap(), self.true_monkey.unwrap(), self.false_monkey.unwrap())
    }
}

enum ParserState {
    MonkeyIndex,
    StartingItems,
    Operation,
    Test,
    TestTrue,
    TestFalse,
    Blank,
}
impl ParserState {
    fn next(&self) -> ParserState {
        match self {
            Self::MonkeyIndex => Self::StartingItems,
            Self::StartingItems => Self::Operation,
            Self::Operation => Self::Test,
            Self::Test => Self::TestTrue,
            Self::TestTrue => Self::TestFalse,
            Self::TestFalse => Self::Blank,
            Self::Blank => Self::MonkeyIndex,
        }
    }
}

fn generate_operation(left: Option<u128>, op: &str, right: Option<u128>) -> Box<dyn Fn(u128) -> u128> {
    match op {
        "+" => match left {
            None => match right {
                None => Box::new(move |x| x + x),
                Some(right) => Box::new(move |x| x + right),
            },
            Some(left) => match right {
                None => Box::new(move |x| left + x),
                Some(right) => Box::new(move |_| left + right),
            },
        },
        "*" => match left {
            None => match right {
                None => Box::new(move |x| x * x),
                Some(right) => Box::new(move |x| x * right),
            },
            Some(left) => match right {
                None => Box::new(move |x| left * x),
                Some(right) => Box::new(move |_| left * right),
            },
        },
        op => panic!("invalid operator: {op}"),
    }
}

fn generate_test(divisor: u128, true_monkey: usize, false_monkey: usize) -> Box<dyn Fn(u128) -> (u128, usize)> {
    Box::new(move |x| {
        match x % divisor {
            0 => (0, true_monkey),
            x => (x, false_monkey),
        }
    })
}


/*
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
*/
fn parse(input: String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut parser_state = ParserState::MonkeyIndex;
    // let mut starting_items: Vec<u128> = vec![];
    // let mut operation: Box<dyn Fn(u128) -> u128> = Box::new(|x| x);
    // let mut divisor: &str = "";
    // let mut true_monkey: &str = "";
    // let mut false_monkey: &str = "";
    // let mut test: Box<dyn Fn(u128) -> (u128, usize)>;
    let mut monkey_builder = MonkeyBuilder::new();
    let mut test_builder = TestBuilder::new();
    for line in input.lines() {
        match parser_state {
            ParserState::MonkeyIndex => (),
            ParserState::StartingItems => {
                let slice = &line[18..];
                let items = slice.split(", ")
                    .map(|item| item.parse::<u128>().unwrap())
                    .collect::<Vec<_>>();
                monkey_builder = monkey_builder.with_items(items);
                //dbg!(&items);
            },
            ParserState::Operation => {
                let slice = &line[19..];
                let operation = match slice.split(" ").collect::<Vec<_>>()[..] {
                    [left, op, right] => {
                        let left = left.parse::<u128>().ok();
                        let right = right.parse::<u128>().ok();
                        generate_operation(left, op, right)
                    },
                    _ => panic!("unable to parse operation: {slice}"),
                };
                monkey_builder = monkey_builder.with_operation(operation);
                //dbg!((*operation)(10));
            },
            ParserState::Test => {
                let divisor = &line[21..].parse::<u128>().unwrap();
                test_builder = test_builder.with_divisor(*divisor);
                //dbg!(&divisor);
            },
            ParserState::TestTrue => {
                let true_monkey = &line[29..].parse::<usize>().unwrap();
                test_builder = test_builder.with_true_monkey(*true_monkey);
                //dbg!(&true_monkey);
            },
            ParserState::TestFalse => {
                let false_monkey = &line[30..].parse::<usize>().unwrap();
                test_builder = test_builder.with_false_monkey(*false_monkey);
                //dbg!(&false_monkey);

                let test = test_builder.build();

                monkey_builder = monkey_builder.with_test(test);
                let monkey = monkey_builder.build();
                monkeys.push(monkey);

                test_builder = TestBuilder::new();
                monkey_builder = MonkeyBuilder::new();
            },
            ParserState::Blank => (),
        }
        parser_state = parser_state.next();
    }
    monkeys
}

#[derive(Debug)]
struct Movement {
    from_monkey_index: usize,
    to_monkey_index: usize,
    from_item_index: usize,
    new_value: u128,
}

pub fn solve(input_path: &str) -> Result<String, Box<dyn Error>> {
    let input: String = std::fs::read_to_string(input_path)?;

    let mut monkeys = parse(input);
    let mut inspection_count: Vec<u128> = [0].repeat(monkeys.len());
    let num_rounds = 10_000;

    for round in 1..=num_rounds {
        // println!("======= ROUND {round} =======");
        for monkey_number in 0..monkeys.len() {
            let monkey = &monkeys[monkey_number];
            let mut movements: Vec<Movement> = vec![];
            // println!("Monkey {monkey_number}:");
            for item_index in 0..monkey.items.len() {
                inspection_count[monkey_number] += 1;
                let item = monkey.items[item_index];
                // println!("  Monkey inspects an item with a worry level of {item}.");
                let new_value = (monkey.operation)(item.clone());
                // println!("    Worry level is increased to {new_value}");
                // let new_value = new_value / 3;
                // println!("    Monkey gets bored with the item. Worry level is divided by 3 to {new_value}");
                let (_, throw_to) = (monkey.test)(new_value.clone());
                // if true {
                //     println!("value was: {item}, new_value: {new_value}");
                // }
                // println!("    Current worry level tested");
                // println!("    Item with worry level {new_value} is thrown to monkey {throw_to}");
                movements.push(Movement {
                    from_monkey_index: monkey_number,
                    to_monkey_index: throw_to,
                    from_item_index: item_index,
                    new_value: new_value,
                })
            }
            let remove_indices = movements.iter()
                .map(|movement| movement.from_item_index)
                .collect::<HashSet<_>>();
            monkeys[monkey_number].items = monkeys[monkey_number].items.iter()
                .enumerate()
                .filter(|(item_index, item)| !remove_indices.contains(item_index))
                .map(|(_, item)| item.clone())
                .collect::<Vec<_>>();
            for movement in movements {
                monkeys[movement.to_monkey_index].items.push(movement.new_value);
            }
        }
        match round {
            _ if (round % 1000 == 0) || ([1, 20].contains(&(round as usize))) => {
                println!("======= AFTER ROUND {round} =======");
                for (index, count) in inspection_count.iter().enumerate() {
                    println!("Monkey {0} inspected items {1} times.", index, count);
                }
            },
            _ => (),
        }
    }

    inspection_count.sort();
    inspection_count.reverse();
    dbg!(&inspection_count);
    let result = inspection_count[0..=1].iter().product::<u128>().to_string();

    Ok(result)
}

// dap -> okay, maybe the way to go is loop detection. if we end up at the same monkey with the same worry level...
// the number of inspections
// ...
// if all the inspection counts are the same muliple of a previous set of counts, can set everything back to 0
// in fact, we should be able to just figure out the final score using the period and multiplier without looping more
// ...
// does each item have its own loop period?

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let result = solve("data/test_input.txt").expect("bad result");
        let solution = 2713310158u128.to_string();
        assert_eq!(result, solution)
    }
}
