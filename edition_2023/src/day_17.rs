use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

use crate::common::Point;

pub fn part_one(heat_map: &str) -> u32 {
    let mut grid = parse_heatmap(heat_map);
    dijkstra(&mut grid, |_, _, next| next.times_in_direction <= 3);

    let lines = grid.len() - 1;
    let columns = grid[0].len() - 1;

    grid[lines][columns].distance.unwrap_or(u32::MAX)
}

pub fn part_two(heat_map: &str) -> u32 {
    let mut grid = parse_heatmap(heat_map);
    dijkstra(&mut grid, |last_direction, times_in_direction, next| {
        if next.times_in_direction > 10 {
            return false;
        }

        if *last_direction != Direction::None
            && *last_direction != next.last_direction
            && times_in_direction < 4
        {
            return false;
        }

        true
    });

    let lines = grid.len() - 1;
    let columns = grid[0].len() - 1;

    grid[lines][columns].distance.unwrap_or(u32::MAX)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    position: Point,
    heat_diss: u32,
    distance: Option<u32>,
}

impl Node {
    fn new(x: usize, y: usize, heat_diss: u32) -> Self {
        Self {
            position: Point::new_usize(x, y),
            heat_diss,
            distance: None,
        }
    }

    fn find_neighbours(&self, grid: &[Vec<Self>]) -> Vec<(usize, usize, Direction)> {
        let mut neighbours = Vec::new();
        let this_x = self.position.x as usize;
        let this_y = self.position.y as usize;

        if this_y > 0 {
            neighbours.push((this_x, this_y - 1, Direction::North));
        }
        if this_y + 1 < grid.len() {
            neighbours.push((this_x, this_y + 1, Direction::South));
        }
        if this_x > 0 {
            neighbours.push((this_x - 1, this_y, Direction::West));
        }
        if this_x + 1 < grid[this_y].len() {
            neighbours.push((this_x + 1, this_y, Direction::East));
        }

        neighbours
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}

impl Direction {
    fn is_opposite(&self, other: &Self) -> bool {
        match self {
            Direction::North => other == &Direction::South,
            Direction::South => other == &Direction::North,
            Direction::East => other == &Direction::West,
            Direction::West => other == &Direction::East,
            Direction::None => false,
        }
    }
}

// A struct to hold the state of each node in the priority queue
#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Point,
    last_direction: Direction,
    times_in_direction: usize,
}

// Implement Ord so that the min-heap becomes a max-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_heatmap(grid: &str) -> Vec<Vec<Node>> {
    grid.lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| Node::new(x, y, c.to_digit(10).unwrap()))
                .collect()
        })
        .collect()
}

fn dijkstra<F>(grid: &mut Vec<Vec<Node>>, accep_next: F)
where
    F: Fn(&Direction, usize, &State) -> bool,
{
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();

    // Initialize
    grid[0][0].distance = Some(0);
    heap.push(State {
        cost: 0,
        position: grid[0][0].position.clone(),
        last_direction: Direction::None,
        times_in_direction: 0,
    });

    let lines = grid.len() - 1;
    let columns = grid[0].len() - 1;

    while let Some(State {
        cost,
        position,
        last_direction,
        times_in_direction,
    }) = heap.pop()
    {
        let x = position.x as usize;
        let y = position.y as usize;

        if !seen.insert((position, last_direction, times_in_direction)) {
            continue;
        }

        // Explore neighbors
        for (nx, ny, direction) in grid[y][x].find_neighbours(grid) {
            if last_direction.is_opposite(&direction) {
                continue;
            }

            let neighbor = &grid[ny][nx];

            let next = State {
                cost: cost + neighbor.heat_diss,
                position: neighbor.position.clone(),
                last_direction: direction,
                times_in_direction: if last_direction == direction {
                    times_in_direction + 1
                } else {
                    1
                },
            };

            if !accep_next(&last_direction, times_in_direction, &next) {
                continue;
            }

            if next.cost < neighbor.distance.unwrap_or(u32::MAX) {
                grid[ny][nx].distance = Some(next.cost);

                if ny == lines && nx == columns {
                    return;
                }
            }
            heap.push(next);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let heat_map = "\
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533";

        assert_eq!(102, part_one(heat_map));
        assert_eq!(94, part_two(heat_map));
    }

    #[test]
    fn real() {
        let heat_map = include_str!("../res/day_17.txt");

        assert_eq!(1076, part_one(heat_map));
        assert_eq!(1219, part_two(heat_map));
    }
}
