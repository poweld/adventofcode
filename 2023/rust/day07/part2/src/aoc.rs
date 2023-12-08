use std::cmp::Ordering;
use std::collections::HashMap;

static CARD_CHARS: &str = "J0123456789TQKA";
fn card_to_value(c: &char) -> u8 {
    CARD_CHARS.chars().position(|card_char| card_char == *c)
        .expect(&format!("could not find card char: '{c}'"))
        .try_into().unwrap()
}

fn parse(input: &str) -> Vec<(String, u64)> {
    input.lines()
        .map(|line| line.split_whitespace())
        .map(|mut cards_and_bet_strs| {
            let cards = String::from(cards_and_bet_strs.next().unwrap());
            let bet_str = cards_and_bet_strs.last().unwrap();
            let bet = bet_str.parse::<u64>().expect(&format!("failed to parse to u64: {bet_str}"));
            (cards, bet)
        })
        .collect::<Vec<_>>()
}

fn score_hand(counter: &HashMap<char, u8>) -> u8 {
    let mut maybe_best: Option<u8> = None;
    let jokers = counter.get(&'J').or(Some(&0)).unwrap();
    if jokers == &5 {
        return 10;
    }
    for (card, count) in counter.iter() {
        if card == &'J' {
            continue;
        }
        let score = count * 2;  // Double the highest card count to be able to handle other hands
        if let Some(best) = maybe_best {
            if best == 6 && score == 4 || best == 4 && score == 6 {
                maybe_best = Some(7); // Nudge up by one for full house
            } else if best == 4 && score == 4 {
                maybe_best = Some(5); // Nudge up by one for two pair
            } else if score > best {
                maybe_best = Some(score);
            }
        } else {
            maybe_best = Some(score);
        }
    }
    maybe_best
        .map(|best| std::cmp::min(10, best + (jokers * 2)))
        .expect("did not find the best score")
}

#[derive(Debug)]
struct CardsBetScore {
    cards: String,
    bet: u64,
    score: u8,
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let cards_and_bets = parse(&input);
    let mut cards_bets_scores = cards_and_bets.iter()
        .map(|(cards, bet)| {
            let mut counter: HashMap<char, u8> = HashMap::new();
            for card in cards.chars() {
                counter.entry(card).and_modify(|count| *count += 1).or_insert(1);
            }
            let score = score_hand(&counter);
            CardsBetScore { cards: cards.to_string(), bet: *bet, score }
        })
        .collect::<Vec<_>>();
    cards_bets_scores.sort_by(|a, b| {
        match a.score.cmp(&b.score) {
            Ordering::Equal => {
                a.cards.chars()
                    .zip(b.cards.chars())
                    .find_map(|(card_a, card_b)| {
                        match card_to_value(&card_a).cmp(&card_to_value(&card_b)) {
                            Ordering::Equal => None,
                            ordering => Some(ordering),
                        }
                    })
                    .unwrap()
            }
            ordering => ordering,
        }
    });
    cards_bets_scores.iter()
        .enumerate()
        .map(|(index, card_bet_score)| {
            let rank = index + 1;
            let winnings = (rank as u64) * card_bet_score.bet;
            // dbg!(&rank, &card_bet_score, &winnings);
            winnings
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
        assert_eq!(result, 5905.to_string());
    }
}
