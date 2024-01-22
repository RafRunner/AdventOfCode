use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
};

use crate::common::turn_anticlock;

pub fn part_one(rocks_str: &str) -> usize {
    let mut rocks = parse(rocks_str);
    fall_north(&mut rocks);

    calculate_value(&rocks)
}

pub fn part_two(rocks_str: &str) -> usize {
    let mut rocks = parse(rocks_str);
    let mut sequence = Vec::new();
    let mut hashes = Vec::new();

    let loops = 1000000000;

    for index in 0..=loops {
        for _ in 0..4 {
            fall_north(&mut rocks);
            rocks = turn_anticlock(rocks);
        }
        let last_value = calculate_value(&rocks);
        sequence.push(last_value);
        let mut hasher = DefaultHasher::new();
        rocks.hash(&mut hasher);
        let hash = hasher.finish();

        if let Some((repetition, _)) = hashes.iter().rev().enumerate().find(|(_, h)| **h == hash) {
            let repetition_size = repetition + 1;
            let true_size = loops - (index - repetition_size);
            let seq_index = true_size % repetition_size;

            return sequence[index - repetition_size + seq_index - 1];
        }

        hashes.push(hash);
    }

    *sequence.last().unwrap()
}

fn parse(rocks_str: &str) -> Vec<Vec<char>> {
    rocks_str
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn fall_north(rocks: &mut [Vec<char>]) {
    for j in 0..rocks[0].len() {
        let mut new_rock_position = 0;
        let mut new_column = HashSet::new();
        let mut immovebles = HashSet::new();

        for (i, line) in rocks.iter().enumerate() {
            let rock = line[j];

            if rock == 'O' {
                new_column.insert(new_rock_position);
                new_rock_position += 1;
            } else if rock == '#' {
                immovebles.insert(i);
                new_rock_position = i + 1;
            }
        }

        for (i, line) in rocks.iter_mut().enumerate() {
            if new_column.contains(&i) {
                line[j] = 'O';
            } else if immovebles.contains(&i) {
                line[j] = '#';
            } else {
                line[j] = '.';
            }
        }
    }
}

fn calculate_value(rocks: &[Vec<char>]) -> usize {
    rocks
        .iter()
        .rev()
        .enumerate()
        .map(|(i, line)| (i + 1) * line.iter().filter(|&&c| c == 'O').count())
        .sum()
}

#[allow(dead_code)]
fn print_rocks(rocks: &[Vec<char>]) {
    for line in rocks {
        for char in line {
            print!("{char}");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let rocks_str = "\
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....";

        let mut rocks = parse(rocks_str);
        fall_north(&mut rocks);

        assert_eq!(136, calculate_value(&rocks));
        assert_eq!(64, part_two(rocks_str));
    }

    #[test]
    fn real() {
        let rocks_str = include_str!("../res/day_14.txt");

        let mut rocks = parse(rocks_str);
        fall_north(&mut rocks);

        assert_eq!(110128, calculate_value(&rocks));
        assert_eq!(103861, part_two(rocks_str));
    }
}
