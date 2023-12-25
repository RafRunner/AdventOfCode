use std::collections::HashSet;

pub fn part_one(rocks_str: &str) -> usize {
    let mut rocks = parse(rocks_str);
    fall_north(&mut rocks);

    calculate_value(&rocks)
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

        for i in 0..rocks.len() {
            if new_column.contains(&i) {
                rocks[i][j] = 'O';
            } else if immovebles.contains(&i) {
                rocks[i][j] = '#';
            } else {
                rocks[i][j] = '.';
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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn print_rocks(rocks: &[Vec<char>]) {
        for line in rocks {
            for char in line {
                print!("{char}");
            }
            println!();
        }
    }

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
        // print_rocks(&rocks);

        assert_eq!(136, calculate_value(&rocks));
    }

    #[test]
    fn real() {
        let rocks_str = include_str!("../res/day_14.txt");

        let mut rocks = parse(rocks_str);
        fall_north(&mut rocks);

        // print_rocks(&rocks);

        assert_eq!(110128, calculate_value(&rocks));
    }
}
