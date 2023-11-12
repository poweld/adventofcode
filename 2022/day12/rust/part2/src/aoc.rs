use std::error::Error;

#[derive(Copy, Clone, Debug)]
struct Register(i64);
impl Register {
    fn new() -> Self {
        Self(1)
    }
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Addx(x) => self.0 += x,
            Instruction::Noop => (),
        };
    }
    fn get(&self) -> i64 {
        self.0
    }
}

#[derive(Debug)]
enum CpuState {
    Ready,
    Busy(Instruction, u8),
}

#[derive(Debug)]
struct Cpu {
    register: Register,
    state: CpuState,
}
impl Cpu {
    fn new() -> Self {
        Self {
            register: Register::new(),
            state: CpuState::Ready,
        }
    }
    fn submit(&mut self, instruction: Instruction) {
        self.state = CpuState::Busy(instruction, instruction.cycles())
    }
    fn execute_cycle(&mut self) -> i64 {
        let value_during_cycle = self.register.get();
        match self.state {
            CpuState::Ready => panic!("executing cycle when cpu is ready"),
            CpuState::Busy(instruction, cycles_remaining) => {
                match cycles_remaining {
                    1 => {
                        self.state = CpuState::Ready;
                        self.register.execute(instruction);
                    },
                    _ =>  {
                        self.state = CpuState::Busy(instruction, cycles_remaining - 1);
                    }
                }
            },
        }
        value_during_cycle
    }
    fn ready(&self) -> bool {
        match self.state {
            CpuState::Ready => true,
            _ => false,
        }
    }
    fn busy(&self) -> bool {
        !self.ready()
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Addx(i64),
    Noop,
}
impl Instruction {
    fn cycles(&self) -> u8 {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1,
        }
    }
}

fn parse(input: String) -> Vec<Instruction> {
    input.lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|split| match split[..] {
            ["addx", n] => Instruction::Addx(i64::from_str_radix(n, 10).unwrap()),
            ["noop"] => Instruction::Noop,
            _ => panic!(),
        })
        .collect::<Vec<_>>()
}

pub fn solve(input_path: &str) -> Result<String, Box<dyn Error>> {
    let input: String = std::fs::read_to_string(input_path)?;

    let instructions = parse(input);
    let mut cpu = Cpu::new();
    let crt_width = 40;
    let chars = instructions.iter()
        .flat_map(|instruction| {
            let mut results = vec![];
            cpu.submit(*instruction);
            while cpu.busy() {
                let result = cpu.execute_cycle();
                results.push(result);
            }
            results.into_iter()
        })
        .enumerate()
        .map(|(index, value)| {
            let cycle = index + 1;
            let sprite_positions = (value - 1)..=(value + 1);
            let crt_position = (index % crt_width) as i64;
            if sprite_positions.contains(&crt_position) {
                '#'
            } else {
                '.'
            }
        })
        .collect::<Vec<_>>();

    let result: String = chars.chunks(crt_width)
        .map(|char_chunk| String::from_iter(char_chunk.iter()))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(result)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solution() {
        let result = solve("data/test_input.txt").expect("bad result");
        let solution = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        let solution = solution.to_string();
        assert_eq!(result, solution)
    }
}
