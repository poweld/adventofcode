use std::cmp::Ordering;
use Ordering::{Less, Equal, Greater};

static CARD_CHARS: &str = "0123456789TJQKA";

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

#[derive(Debug, Eq)]
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
        // dbg!(&cards);
        // bleh.  .take_while(...)  ?
        let mut previous_card = None;
        let mut matching_cards = vec![];
        let mut current_matching_cards = vec![];
        for card in cards.iter().rev() {
            if let Some(previous_card) = previous_card {
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
        matching_cards.push(current_matching_cards);
        // TODO i think this sort is bad. if they're equal length the order they're gonna be in is not known
        matching_cards.sort_by(|a, b| {
            match a.len().cmp(&b.len()) {
                Equal => a[0].cmp(&b[0]),
                x => x,
            }
        });
        dbg!(&matching_cards);
        let mut longest_two = matching_cards.iter().rev().take(2);
        match [longest_two.next(), longest_two.next()] {
            [Some(longest), Some(second_longest)] if longest.len() == 3 && second_longest.len() == 2 => {
                FullHouse(longest[0].clone(), second_longest[0].clone())
            },
            [Some(longest), Some(second_longest)] if longest.len() == 2 && second_longest.len() == 2 => {
                TwoPair(longest[0].clone(), second_longest[0].clone())
            },
            [Some(longest), None] => FiveOfAKind(longest[0].clone()),
            [Some(longest), _] => match longest.len() {
                4 => FourOfAKind(longest[0].clone()),
                3 => ThreeOfAKind(longest[0].clone()),
                2 => OnePair(longest[0].clone()),
                1 => HighCard(longest[0].clone()),
                _ => panic!("encountered unexpected length for longest matching card: {longest:?}"),
            },
            _ => panic!("could not find longest"),
        }
    }
}
impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        // Ordering::{Less, Equal, Greater}
        let cmp2 = |(most_sig_ordering, least_sig_ordering): (&Ordering, &Ordering)| -> Ordering {
            match most_sig_ordering {
                Equal => *least_sig_ordering,
                _ => *most_sig_ordering,
            }
        };
        match self {
            HighCard(self_card) => match other {
                HighCard(other_card) => self_card.cmp(&other_card),
                _ => Less,
            },
            OnePair(self_card) => match other {
                HighCard(_) => Greater,
                OnePair(other_card) => self_card.cmp(&other_card),
                _ => Less,
            },
            TwoPair(self_most_sig_card, self_least_sig_card) => match other {
                OnePair(_) | HighCard(_) => Greater,
                TwoPair(other_most_sig_card, other_least_sig_card) =>
                    cmp2((&self_most_sig_card.cmp(self_least_sig_card), &other_most_sig_card.cmp(other_least_sig_card))),
                _ => Less,
            },
            ThreeOfAKind(self_card) => match other {
                OnePair(_) | HighCard(_) | TwoPair(..) => Greater,
                ThreeOfAKind(other_card) => self_card.cmp(&other_card),
                _ => Less,
            },
            FullHouse(self_most_sig_card, self_least_sig_card) => match other {
                OnePair(_) | HighCard(_) | TwoPair(..) | ThreeOfAKind(_) => Greater,
                FullHouse(other_most_sig_card, other_least_sig_card) =>
                    cmp2((&self_most_sig_card.cmp(self_least_sig_card), &other_most_sig_card.cmp(other_least_sig_card))),
                _ => Less,
            },
            FourOfAKind(self_card) => match other {
                OnePair(_) | HighCard(_) | TwoPair(..) | ThreeOfAKind(_) | FullHouse(..) => Greater,
                FourOfAKind(other_card) => self_card.cmp(&other_card),
                _ => Less,
            },
            FiveOfAKind(self_card) => match other {
                FiveOfAKind(other_card) => self_card.cmp(&other_card),
                _ => Greater,
            },
        }
    }
}

// Implement From instead of Into since Into comes for free with From
// static hand_type_value_order: [HandType; 7] = [
//     HandType::HighCard,
//     HandType::OnePair,
//     HandType::TwoPair,
//     HandType::ThreeOfAKind,
//     HandType::FullHouse,
//     HandType::FourOfAKind,
//     HandType::FiveOfAKind,
// ];
// impl From<u64> for HandType {
//     fn from(value: u64) -> Self {
//         // hand_type_value_order.iter()
//         //     .nth(value as usize)
//         //     .expect(&format!("could not convert value to hand type: {value}"))
//         //     .clone()
//         match value {
//             0 => Self::HighCard,
//             1 => Self::OnePair,
//             2 => Self::TwoPair,
//             3 => Self::ThreeOfAKind,
//             4 => Self::FullHouse,
//             5 => Self::FourOfAKind,
//             6 => Self::FiveOfAKind,
//             _ => panic!("unmapped value: {value}")
//         }
//     }
// }

#[derive(Debug, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}
impl Hand {
    fn from(cards: &[Card; 5]) -> Self {
        Self { cards: (*cards).clone(), hand_type: HandType::from(cards) }
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
// TODO can we just derive PartialOrd if we define Ord?
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // Ordering::{Less, Equal, Greater}
        self.hand_type.cmp(&other.hand_type)
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
    // dbg!(&hands_and_bets);
    hands_and_bets.iter()
        .enumerate()
        .map(|(index, (hand, bet))| {
            let rank = (index as u64) + 1;
            dbg!(&hand, &rank, &bet);
            // TODO OnePairs are coming in out of order!
            dbg!(bet * rank)
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
        assert_eq!(result, 6440.to_string());
    }
}
