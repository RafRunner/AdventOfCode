use std::collections::HashSet;

use crate::common::Point;

pub fn calculate_distances(universe_str: &str, expansion: usize) -> usize {
    let universe = Universe::parse_galaxy(universe_str, expansion);

    let pairs = Point::point_pairs(&universe.galaxies);

    pairs
        .into_iter()
        .map(|(p1, p2)| p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y))
        .sum()
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Point>,
    expansion_lines: Vec<isize>,
    expansion_columns: Vec<isize>,
}

impl Universe {
    fn parse_galaxy(universe_str: &str, expansion: usize) -> Self {
        let galaxies = universe_str
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.trim().chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(Point::new_usize(x, y))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        let expansion_lines = Self::find_empty(&galaxies, |p| p.y);
        let expansion_columns = Self::find_empty(&galaxies, |p| p.x);

        let mut universe = Self {
            galaxies,
            expansion_lines,
            expansion_columns,
        };

        universe.expand(expansion);
        universe
    }

    fn find_empty(galaxies: &[Point], mapper: impl Fn(&Point) -> isize) -> Vec<isize> {
        let coordinates = galaxies.iter().map(mapper).collect::<HashSet<_>>();

        (0..coordinates.iter().max().cloned().unwrap_or(0))
            .filter(|x| !coordinates.contains(x))
            .collect()
    }

    fn expand(&mut self, amount: usize) {
        assert!(amount > 0, "Expansion amount must be positive");

        for point in self.galaxies.iter_mut() {
            let times_x = self
                .expansion_columns
                .iter()
                .filter(|x| **x < point.x)
                .count();

            let times_y = self
                .expansion_lines
                .iter()
                .filter(|y| **y < point.y)
                .count();

            point.x += (times_x * (amount - 1)) as isize;
            point.y += (times_y * (amount - 1)) as isize;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

        assert_eq!(374, calculate_distances(input, 2));
        assert_eq!(1030, calculate_distances(input, 10));
        assert_eq!(8410, calculate_distances(input, 100));
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_11.txt");

        assert_eq!(9918828, calculate_distances(input, 2));
        assert_eq!(692506533832, calculate_distances(input, 1000000));
    }
}
