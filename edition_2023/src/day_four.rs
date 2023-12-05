use std::{cell::RefCell, collections::HashSet};

#[derive(Debug)]
struct Card {
    number: usize,
    copies: usize,
    matches: usize,
}

pub fn part_one(file: &str) -> usize {
    parse_cards(file).iter().map(calculate_points).sum()
}

pub fn part_two(file: &str) -> usize {
    let cards_references: Vec<RefCell<Card>> = parse_cards(file)
        .into_iter()
        .map(|card| RefCell::new(card))
        .collect();

    let mut total = cards_references.len();

    for card_rc in &cards_references {
        let card = card_rc.borrow();

        if card.matches == 0 {
            continue;
        }

        for i in card.number..(card.number + card.matches) {
            if let Some(copy) = cards_references.get(i) {
                copy.borrow_mut().copies += card.copies;
                total += card.copies;
            } else {
                break;
            }
        }
    }

    total
}

fn parse_cards(file: &str) -> Vec<Card> {
    file.lines()
        .enumerate()
        .map(|(i, line)| {
            let line = line.trim();
            let card_and_numbers = line.split(':').collect::<Vec<_>>();
            let numbers = card_and_numbers[1].trim();

            let winning_and_mine = numbers.split('|').collect::<Vec<_>>();

            let winning = parse_numbers(winning_and_mine[0]);
            let mine = parse_numbers(winning_and_mine[1]);
            let matches = mine.intersection(&winning).count();

            Card {
                number: i + 1,
                copies: 1,
                matches,
            }
        })
        .collect()
}

fn calculate_points(card: &Card) -> usize {
    match card.matches {
        0 => 0,
        number => 2_usize.pow((number - 1) as u32),
    }
}

fn parse_numbers(numbers: &str) -> HashSet<usize> {
    numbers
        .trim()
        .split_whitespace()
        .map(|number| number.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(13, part_one(input));
        assert_eq!(30, part_two(input));
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_four.txt");

        assert_eq!(23847, part_one(input));
        assert_eq!(8570000, part_two(input));
    }
}