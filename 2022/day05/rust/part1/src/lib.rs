mod my {
    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Stack {
        items: Vec<char>,
    }
    impl Stack {
        pub fn push(&mut self, x: char) {
            self.items.push(x)
        }
        pub fn pop(&mut self) -> Option<char> {
            self.items.pop()
        }
        pub fn len(&self) -> usize {
            self.items.len()
        }
        pub fn new() -> Self {
            Stack { items: Vec::new() }
        }
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct Stacks {
        items: Vec<Stack>,
    }
    impl Stacks {
        pub fn new() -> Self {
            Stacks { items: Vec::new() }
        }
        pub fn len(&self) -> usize {
            self.items.len()
        }
        pub fn from(lines: &Vec<&str>) -> Self {
            let mut this = Self::new();
            let stack_count = (lines.first().unwrap().len() + 1) / 4;
            dbg!(&stack_count);
            for _ in 0..stack_count {
                this.items.push(Stack::new());
            }

            let mut iter = lines.iter().rev();
            // consume first line with stack numbers
            iter.next();
            for line in iter {
                for stacks_index in 0..stack_count {
                    let line_index = 1 + (stacks_index * 4);
                    dbg!(&line_index);
                    let c = line.chars().nth(line_index).unwrap();
                    // dbg!(&c);
                    if c != ' ' {
                        this.items[stacks_index].push(c);
                    }
                }
            }

            this
        }
        fn format_crate(c: char) -> String {
            if c >= '0' && c <= '9' {
                format!(" {c} ")
            } else if c == ' ' {
                "   ".to_string()
            } else {
                format!("[{c}]")
            }
        }
        pub fn to_string(&self) -> String {
            // build in reverse order
            let mut s = String::new();
            let stack_count = self.items.len();
            let max_stack_len = self.items.iter().map(|x| x.len()).max().unwrap();

            // create the initial line with stack numbers
            for stacks_index in 0..stack_count {
                let n = stacks_index + 1;
                let c = char::from_digit(n as u32, 10).unwrap();
                let crate_str = Self::format_crate(c);
                s += &crate_str;
            }
            s += "\n";

            let mut stacks: Vec<_> = self.items.iter()
                .map(|stack| stack.items.iter())
                .collect();
            for _ in 0..max_stack_len {
                for stacks_index in 0..stack_count {
                    match stacks[stacks_index].next() {
                        Some(c) => s += &Self::format_crate(*c),
                        None => s += &Self::format_crate(' '),
                    }
                }
                s += "\n";
            }
            s += "\n";

            //s.lines().rev().collect()
            s.lines().rev().fold(String::new(), |s, line| {
                s + line + "\n"
            })
        }
        pub fn push(&mut self, stack_index: u8, c: char) {
            self.items[stack_index as usize].push(c)
        }
        pub fn pop(&mut self, stack_index: u8) -> Option<char> {
            self.items[stack_index as usize].pop()
        }
    }
}

use my::Stacks;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let lines = input.lines();
    let mut crate_input: Vec<&str> = Vec::new();
    let mut instruction_input: Vec<&str> = Vec::new();

    let mut saw_blank = false;
    for line in lines {
        if line.len() == 0 {
            saw_blank = true;
            continue;
        }
        if saw_blank {
            instruction_input.push(line);
        } else {
            crate_input.push(line);
        }
    }

    (crate_input, instruction_input)
}

#[derive(Debug)]
struct Instruction {
    count: u8,
    from: u8,
    to: u8,
}
impl Instruction {
    pub fn to_string(&self) -> String {
        format!("move {} from {} to {}", self.count, self.from, self.to)
    }
}

fn parse_instruction_input(lines: &Vec<&str>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in lines {
        let mut nums = line.split_whitespace()
                           .map(|s| s.parse())
                           .filter(|n| n.is_ok())
                           .map(|n| n.unwrap());

        let instruction = Instruction {
            count: nums.next().unwrap(),
            from: nums.next().unwrap(),
            to: nums.next().unwrap(),
        };

        instructions.push(instruction);
    }

    instructions
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path)
        .expect("failed to read input");

    let (crate_input, instruction_input) = parse_input(&input);
    // dbg!(&crate_input);
    // dbg!(&instruction_input);
    let mut stacks = Stacks::from(&crate_input);
    // dbg!(&stacks);
    // let stacks_str = &stacks.to_string();
    // dbg!(&stacks_str);
    let instructions = parse_instruction_input(&instruction_input);
    // dbg!(&instructions);

    for instruction in instructions {
        println!("{}", &stacks.to_string());
        println!("{}", &instruction.to_string());
        for _ in 0..instruction.count {
            let x = stacks.pop(instruction.from - 1).unwrap();
            stacks.push(instruction.to - 1, x);
        }
    }
    println!("{}", &stacks.to_string());

    dbg!(stacks.len());
    let mut result = String::new();
    for stacks_index in 0..stacks.len() {
        let c = stacks.pop(stacks_index as u8);
        match c {
            Some(c) => result.push(c),
            None => (),
        }
    }

    result
}
