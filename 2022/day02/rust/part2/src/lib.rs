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

enum RPSResult {
    Win,
    Lose,
    Draw,
}

fn move_for_result(opponent_move: &Move, result: &RPSResult) -> Move {
    match opponent_move {
        Move::Rock => match result {
            RPSResult::Win => Move::Paper,
            RPSResult::Lose => Move::Scissors,
            RPSResult::Draw => Move::Rock,
        },
        Move::Paper => match result {
            RPSResult::Win => Move::Scissors,
            RPSResult::Lose => Move::Rock,
            RPSResult::Draw => Move::Paper,
        },
        Move::Scissors => match result {
            RPSResult::Win => Move::Rock,
            RPSResult::Lose => Move::Paper,
            RPSResult::Draw => Move::Scissors,
        },
    }
}

fn char_to_move(c: &char) -> Move {
    match c {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => panic!("invalid move character {c:?}"),
    }
}

fn char_to_result(c: &char) -> RPSResult {
    match c {
        'X' => RPSResult::Lose,
        'Y' => RPSResult::Draw,
        'Z' => RPSResult::Win,
        _ => panic!("invalid result character {c:?}"),
    }
}

fn to_strategy(line: &str) -> Strategy {
    let mut chars = line.chars();
    let opponent_move = char_to_move(&chars.next().expect("could not find opponent move char"));
    chars.next();
    let desired_result = char_to_result(&chars.next().expect("could not find desired result char"));

    Strategy {
        player_move: move_for_result(&opponent_move, &desired_result),
        opponent_move,
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
