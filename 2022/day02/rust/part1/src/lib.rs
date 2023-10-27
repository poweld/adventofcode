#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Strategy {
    fn score(&self) -> u32 {
        match self.opponent_move {
            Move::Rock => match self.player_move {
                Move::Rock => 3,
                Move::Paper => 6,
                Move::Scissors => 0,
            },
            Move::Paper => match self.player_move {
                Move::Rock => 0,
                Move::Paper => 3,
                Move::Scissors => 6,
            },
            Move::Scissors => match self.player_move {
                Move::Rock => 6,
                Move::Paper => 0,
                Move::Scissors => 3,
            },
        }
    }
}

#[derive(Debug)]
struct Strategy {
    opponent_move: Move,
    player_move: Move,
}

fn char_to_move(c: &char) -> Move {
    match c {
        'A' | 'X' => Move::Rock,
        'B' | 'Y' => Move::Paper,
        'C' | 'Z' => Move::Scissors,
        _ => panic!("invalid move character {c:?}"),
    }
}

fn to_strategy(line: &str) -> Strategy {
    let mut chars = line.chars();
    let opponent_move = char_to_move(&chars.next().expect("could not parse opponent move"));
    chars.next();
    let player_move = char_to_move(&chars.next().expect("could not parse player move"));

    Strategy {
        opponent_move,
        player_move,
    }
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path)
        .expect("failed to read input");

    input.lines()
         .map(|line| to_strategy(line))
         .map(|strategy| strategy.player_move.score() + strategy.score())
         .sum::<u32>()
         .to_string()
}
