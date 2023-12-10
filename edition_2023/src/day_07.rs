use std::cmp::Ordering;
use std::collections::HashMap;

pub fn solve_puzzle(hands_str: &str, jokers: bool) -> usize {
    let mut hands = CamelHand::parse_all(hands_str, jokers);
    hands.sort();

    hands
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bet)
        .sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum CamelCard {
    Ace,
    King,
    Queen,
    Jockey,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl CamelCard {
    fn parse_cards(cards: &str, jokers: bool) -> Vec<CamelCard> {
        cards
            .chars()
            .map(|card_char| match &card_char {
                'A' => Self::Ace,
                'K' => Self::King,
                'Q' => Self::Queen,
                'J' => {
                    if jokers {
                        Self::Joker
                    } else {
                        Self::Jockey
                    }
                }
                'T' => Self::Ten,
                '9' => Self::Nine,
                '8' => Self::Eight,
                '7' => Self::Seven,
                '6' => Self::Six,
                '5' => Self::Five,
                '4' => Self::Four,
                '3' => Self::Three,
                '2' => Self::Two,
                _ => panic!("No card for char {}", card_char),
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CamelHandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq)]
struct CamelHand {
    cards: Vec<CamelCard>,
    bet: usize,
    kind: CamelHandType,
}

impl CamelHand {
    fn parse_all(hands: &str, jokers: bool) -> Vec<Self> {
        hands
            .lines()
            .map(|hand| Self::parse(hand.trim(), jokers))
            .collect()
    }

    fn parse(hand_and_bet: &str, jokers: bool) -> Self {
        let parts: Vec<&str> = hand_and_bet.split_whitespace().collect();

        let cards = CamelCard::parse_cards(parts[0], jokers);
        let bet: usize = parts[1].parse().unwrap();

        let cards_clone = cards.clone();
        let mut counts = HashMap::new();

        for card in cards_clone.into_iter() {
            counts
                .entry(card)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let jokers = if counts.len() < 2 {
            None
        } else {
            counts.remove(&CamelCard::Joker)
        };

        let mut counts = counts.into_values().collect::<Vec<_>>();

        counts.sort();
        counts.reverse();

        if let Some(j) = jokers {
            let old = counts[0];
            let _ = std::mem::replace(&mut counts[0], old + j);
        }

        let kind = match (counts[0], counts.get(1).unwrap_or(&0_usize)) {
            (5, _) => CamelHandType::FiveOfAKind,
            (4, _) => CamelHandType::FourOfAKind,
            (3, 2) => CamelHandType::FullHouse,
            (3, _) => CamelHandType::ThreeOfAKind,
            (2, 2) => CamelHandType::TwoPair,
            (2, _) => CamelHandType::OnePair,
            (_, _) => CamelHandType::HighCard,
        };

        Self { cards, bet, kind }
    }
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => {
                for (this_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    match this_card.cmp(other_card) {
                        Ordering::Equal => (),
                        different => return different,
                    };
                }

                Ordering::Equal
            }
            different => different,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let hands_str = "\
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        assert_eq!(6440, solve_puzzle(hands_str, false));
        assert_eq!(5905, solve_puzzle(hands_str, true));
    }

    #[test]
    fn real() {
        let hands_str = include_str!("../res/day_07.txt");

        assert_eq!(251545216, solve_puzzle(hands_str, false));
        assert_eq!(250384185, solve_puzzle(hands_str, true));
    }
}
