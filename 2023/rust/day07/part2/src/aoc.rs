use std::cmp::Ordering;
use Ordering::{Less, Equal, Greater};

// Could probably put J at the bottom and get rid of 0 and 1 since we're not doing scoring properly, but ¯\_(ツ)_/¯
static CARD_CHARS: &str = "J123456789TQKA";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Card(u8);
impl Card {
    fn from(c: char) -> Self {
        let card_value = CARD_CHARS.chars()
            .position(|card_char| card_char == c)
            .expect(&format!("failed to find card value for char '{c}'"));
        Card(card_value as u8)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    HighCard(Card),
    OnePair(Card),
    TwoPair(Card, Card),
    ThreeOfAKind(Card),
    FullHouse(Card, Card),  // It sure would be neat if this could instead contain a ThreeOfAKind and OnePair!
    FourOfAKind(Card),
    FiveOfAKind(Card),
}
use HandType::*;
impl HandType {
    fn from(cards: &[Card; 5]) -> Self {
        let mut cards = cards.clone();
        cards.sort();
        let mut previous_card = None;
        let mut matching_cards = vec![];
        let mut current_matching_cards = vec![];
        let mut jokers = vec![];
        for card in cards.iter().rev() {
            if let Card(0) = card {
                jokers.push(card);
            } else if let Some(previous_card) = previous_card {
                if previous_card == card {
                    current_matching_cards.push(card);
                } else {
                    matching_cards.push(current_matching_cards);
                    current_matching_cards = vec![card];
                }
            } else {
                current_matching_cards.push(card);
            }
            previous_card = Some(card);
        }
        if jokers.len() == 5 {
            return FiveOfAKind(jokers[0].clone());
        }
        matching_cards.push(current_matching_cards);
        matching_cards.sort_by(|a, b| {
            match a.len().cmp(&b.len()) {
                Equal => a[0].cmp(&b[0]),
                x => x,
            }
        });
        let mut longest_two = matching_cards.iter().rev().take(2);
        let longest = longest_two.next();
        let longest_len = longest.map(|longest| std::cmp::min(5, longest.len() + jokers.len()));
        let second_longest = longest_two.next();
        let second_longest_len = second_longest.map(|second_longest| second_longest.len());
        match [longest, second_longest] {
            [Some(longest), Some(second_longest)] if longest_len == Some(3) && second_longest_len == Some(2) => {
                FullHouse(longest[0].clone(), second_longest[0].clone())
            },
            [Some(longest), Some(second_longest)] if longest_len == Some(2) && second_longest_len == Some(2) => {
                TwoPair(longest[0].clone(), second_longest[0].clone())
            },
            [Some(longest), None] => FiveOfAKind(longest[0].clone()),
            [Some(longest), _] => match longest_len {
                Some(5) => FiveOfAKind(longest[0].clone()),
                Some(4) => FourOfAKind(longest[0].clone()),
                Some(3) => ThreeOfAKind(longest[0].clone()),
                Some(2) => OnePair(longest[0].clone()),
                Some(1) => HighCard(longest[0].clone()),
                _ => panic!("encountered unexpected length for longest matching card: {longest:?}"),
            },
            _ => panic!("could not find longest"),
        }
    }
}
impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            HighCard(_) => match other {
                HighCard(_) => Equal,
                _ => Less,
            },
            OnePair(_) => match other {
                HighCard(_) => Greater,
                OnePair(_) => Equal,
                _ => Less,
            },
            TwoPair(..) => match other {
                OnePair(_) | HighCard(_) => Greater,
                TwoPair(..) => Equal,
                _ => Less,
            },
            ThreeOfAKind(_) => match other {
                OnePair(_) | HighCard(_) | TwoPair(..) => Greater,
                ThreeOfAKind(_) => Equal,
                _ => Less,
            },
            FullHouse(..) => match other {
                OnePair(_) | HighCard(_) | TwoPair(..) | ThreeOfAKind(_) => Greater,
                FullHouse(..) => Equal,
                _ => Less,
            },
            FourOfAKind(_) => match other {
                OnePair(_) | HighCard(_) | TwoPair(..) | ThreeOfAKind(_) | FullHouse(_, _) => Greater,
                FourOfAKind(_) => Equal,
                _ => Less,
            },
            FiveOfAKind(_) => match other {
                FiveOfAKind(_) => Equal,
                _ => Greater,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}
impl Hand {
    fn from(cards: &[Card; 5]) -> Self {
        Self { cards: (*cards).clone(), hand_type: HandType::from(cards) }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Equal => {
                self.cards.iter()
                    .zip(other.cards.iter())
                    .find(|(a, b)| a.cmp(&b) != Equal)
                    .map(|(a, b)| a.cmp(&b))
                    .or(Some(Equal))
                    .unwrap()
            },
            x => x,
        }
    }
}

fn parse(input: &str) -> Vec<(Hand, u64)> {
    input.lines()
        .map(|line| line.split_whitespace())
        .map(|mut cards_and_bet_strs| {
            let cards_str = cards_and_bet_strs.next().unwrap();
            let cards: [Card; 5] = cards_str.chars()
                .map(Card::from)
                .collect::<Vec<_>>()
                .try_into().expect(&format!("failed to convert into 5 cards: {cards_str}"));
            let hand = Hand::from(&cards);
            let bet_str = cards_and_bet_strs.last().unwrap();
            let bet = bet_str.parse::<u64>().expect(&format!("failed to parse to u64: {bet_str}"));
            (hand, bet)
        })
        .collect::<Vec<_>>()
}

pub fn solve(input_path: &str) -> String {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut hands_and_bets = parse(&input);
    hands_and_bets.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(&hand2));
    hands_and_bets.iter()
        .enumerate()
        .map(|(index, (_, bet))| {
            let rank = (index as u64) + 1;
            // dbg!(&hand, &rank, &bet);
            bet * rank
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
