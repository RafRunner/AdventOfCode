use core::str;
use std::{collections::HashSet, vec};

use crate::common::Point;

pub fn part_one(map: &str) -> usize {
    let garden = Garden::parse(map);
    let possible = garden.find_possible_positions(64);

    possible.len()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GardenTileType {
    Starting,
    Rock,
    Plot,
}

impl GardenTileType {
    fn parse(tile: char) -> Self {
        match tile {
            '#' => Self::Rock,
            'S' => Self::Starting,
            '.' => Self::Plot,
            _ => panic!("unknow tile char {tile}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GardenTile {
    kind: GardenTileType,
    position: Point,
}

#[derive(Debug)]
struct Garden {
    tiles: Vec<Vec<GardenTile>>,
}

impl Garden {
    fn parse(map: &str) -> Self {
        Self {
            tiles: map
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.trim()
                        .chars()
                        .enumerate()
                        .map(|(x, tile)| GardenTile {
                            kind: GardenTileType::parse(tile),
                            position: Point::new_usize(x, y),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn find_possible_positions(&self, steps: usize) -> HashSet<GardenTile> {
        let mut visited = HashSet::new();
        let mut possible = HashSet::new();

        let starting = self
            .tiles
            .iter()
            .flatten()
            .find(|t| t.kind == GardenTileType::Starting)
            .expect("No starting tile");

        let mut stack = vec![(starting.clone(), steps)];

        while let Some((tile, remaining)) = stack.pop() {
            // Just a rock
            if tile.kind == GardenTileType::Rock {
                continue;
            }

            // No more steps. It is possible to get here
            if remaining == 0 {
                possible.insert(tile.clone());
                continue;
            }

            // Already visited with this amount of steps
            if !visited.insert((tile.clone(), remaining)) {
                continue;
            }

            let (x, y) = (tile.position.x as usize, tile.position.y as usize);

            // We take a step
            if y > 0 {
                stack.push((self.tiles[y - 1][x].clone(), remaining - 1));
            }
            if y < self.tiles.len() - 1 {
                stack.push((self.tiles[y + 1][x].clone(), remaining - 1));
            }
            if x > 0 {
                stack.push((self.tiles[y][x - 1].clone(), remaining - 1));
            }
            if x < self.tiles[y].len() - 1 {
                stack.push((self.tiles[y][x + 1].clone(), remaining - 1));
            }
        }

        possible
    }

    #[allow(dead_code)]
    fn print(&self, possible: &HashSet<GardenTile>) {
        for line in &self.tiles {
            for tile in line {
                if possible.contains(tile) {
                    print!("O");
                } else {
                    print!(
                        "{}",
                        match tile.kind {
                            GardenTileType::Starting => 'S',
                            GardenTileType::Rock => '#',
                            GardenTileType::Plot => '.',
                        }
                    );
                }
            }

            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "\
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........";

        let garden = Garden::parse(input);
        let possible = garden.find_possible_positions(6);

        garden.print(&possible);
        assert_eq!(16, possible.len());
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_21.txt");

        assert_eq!(3816, part_one(input));
    }
}
