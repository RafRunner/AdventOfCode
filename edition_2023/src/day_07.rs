use std::cmp::Ordering;
use std::collections::HashMap;

pub fn part_one(hands_str: &str) -> usize {
    let mut hands = CamelHand::parse_all(hands_str);
    hands.sort();
    hands.reverse();

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bet)
        .sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum CamelCard {
    Ace,
    King,
    Queen,
    Joker,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl CamelCard {
    fn parse_cards(cards: &str) -> Vec<CamelCard> {
        cards
            .chars()
            .map(|card_char| match &card_char {
                'A' => Self::Ace,
                'K' => Self::King,
                'Q' => Self::Queen,
                'J' => Self::Joker,
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
    fn parse_all(hands: &str) -> Vec<Self> {
        hands.lines().map(|hand| Self::parse(hand.trim())).collect()
    }

    fn parse(hand_and_bet: &str) -> Self {
        let parts: Vec<&str> = hand_and_bet.split_whitespace().collect();

        let cards = CamelCard::parse_cards(parts[0]);
        let bet: usize = parts[1].parse().unwrap();

        let cards_clone = cards.clone();
        let mut counts = HashMap::new();

        for card in cards_clone.into_iter() {
            counts
                .entry(card.clone())
                .or_insert_with(|| cards.iter().filter(|c| **c == card).count());
        }

        let mut counts = counts.values().into_iter().collect::<Vec<_>>();

        counts.sort();
        counts.reverse();

        let kind = match (counts[0], counts.get(1).unwrap_or(&&0_usize)) {
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
        match self.kind.partial_cmp(&other.kind) {
            None => None,
            Some(Ordering::Equal) => {
                let mut i = 0;

                while let Some((this, other)) = self.cards.get(i).zip(other.cards.get(i)) {
                    i += 1;
                    match this.partial_cmp(other) {
                        Some(Ordering::Equal) | None => continue,
                        Some(other) => return Some(other),
                    };
                }

                None
            }
            Some(other) => Some(other),
        }
    }
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
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

        assert_eq!(6440, part_one(hands_str));
    }

    #[test]
    fn real() {
        let hands_str = include_str!("../res/day_07.txt");

        assert_eq!(251545216, part_one(hands_str));
    }
}
