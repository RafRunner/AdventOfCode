use std::{char, collections::HashSet};

pub fn part_one(input: &str) -> usize {
    let mut map = parse(input);
    cast_rays(&mut map, 0, 0, RayDirection::Rightward);

    count_energized(&map)
}

pub fn part_two(input: &str) -> usize {
    let mut map = parse(input);
    let mut max = 0;

    let rows = map.len();
    let cols = map[0].len();

    let directions = [
        (RayDirection::Downward, (0..rows, 0..1)),
        (RayDirection::Rightward, (0..1, 0..cols)),
        (RayDirection::Upward, (0..rows, (cols - 1)..cols)),
        (RayDirection::Leftward, ((rows - 1)..rows, 0..cols)),
    ];

    for (direction, (row_range, col_range)) in directions {
        for i in row_range {
            for j in col_range.clone() {
                cast_rays(&mut map, i, j, direction.clone());
                let result = count_energized(&map);
                if result > max {
                    max = result;
                }
                reset_rays(&mut map);
            }
        }
    }

    max
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum RayDirection {
    Upward,
    Downward,
    Rightward,
    Leftward,
}

impl RayDirection {
    fn move_ray(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Self::Upward => y.checked_sub(1).map(|new_y| (x, new_y)),
            Self::Downward => Some((x, y + 1)),
            Self::Rightward => Some((x + 1, y)),
            Self::Leftward => x.checked_sub(1).map(|new_x| (new_x, y)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum MirrorType {
    Foward,
    Back,
}

impl MirrorType {
    fn parse(char: char) -> Option<Self> {
        match char {
            '/' => Some(Self::Foward),
            '\\' => Some(Self::Back),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum SplitterType {
    Horizontal,
    Vertical,
}

impl SplitterType {
    fn parse(char: char) -> Option<Self> {
        match char {
            '-' => Some(Self::Horizontal),
            '|' => Some(Self::Vertical),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum TileType {
    Empty,
    Mirror(MirrorType),
    Splitter(SplitterType),
}

impl TileType {
    fn parse(char: char) -> Self {
        if let Some(mirror) = MirrorType::parse(char) {
            return Self::Mirror(mirror);
        }

        if let Some(splitter) = SplitterType::parse(char) {
            return Self::Splitter(splitter);
        }

        Self::Empty
    }
}

#[derive(Debug)]
struct Tile {
    rays: HashSet<RayDirection>,
    kind: TileType,
}

impl Tile {
    fn parse(char: char) -> Self {
        Self {
            rays: HashSet::new(),
            kind: TileType::parse(char),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(Tile::parse).collect())
        .collect()
}

fn cast_rays(
    map: &mut [Vec<Tile>],
    starting_x: usize,
    starting_y: usize,
    starting_direction: RayDirection,
) {
    let mut direction = starting_direction;
    let mut x = starting_x;
    let mut y = starting_y;

    while let Some(tile) = map.get_mut(y).and_then(|line| line.get_mut(x)) {
        if !tile.rays.insert(direction.clone()) {
            // Already calculated this route
            break;
        }

        match &tile.kind {
            TileType::Empty => (),
            TileType::Mirror(mirror) => match mirror {
                MirrorType::Foward => {
                    direction = match &direction {
                        RayDirection::Upward => RayDirection::Rightward,
                        RayDirection::Downward => RayDirection::Leftward,
                        RayDirection::Rightward => RayDirection::Upward,
                        RayDirection::Leftward => RayDirection::Downward,
                    };
                }
                MirrorType::Back => {
                    direction = match &direction {
                        RayDirection::Upward => RayDirection::Leftward,
                        RayDirection::Downward => RayDirection::Rightward,
                        RayDirection::Rightward => RayDirection::Downward,
                        RayDirection::Leftward => RayDirection::Upward,
                    };
                }
            },
            TileType::Splitter(splitter) => match splitter {
                SplitterType::Horizontal => match direction {
                    RayDirection::Upward | RayDirection::Downward => {
                        direction = RayDirection::Leftward;
                        cast_rays(map, x + 1, y, RayDirection::Rightward);
                    }
                    RayDirection::Rightward | RayDirection::Leftward => (),
                },
                SplitterType::Vertical => match direction {
                    RayDirection::Upward | RayDirection::Downward => (),
                    RayDirection::Rightward | RayDirection::Leftward => {
                        direction = RayDirection::Upward;
                        cast_rays(map, x, y + 1, RayDirection::Downward);
                    }
                },
            },
        }

        if let Some((new_x, new_y)) = direction.move_ray(x, y) {
            x = new_x;
            y = new_y;
        } else {
            break;
        }
    }
}

fn reset_rays(map: &mut [Vec<Tile>]) {
    for line in map {
        for tile in line {
            tile.rays.clear()
        }
    }
}

fn count_energized(map: &[Vec<Tile>]) -> usize {
    map.iter()
        .flatten()
        .filter(|tile| !tile.rays.is_empty())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|...."#;

        let mut map = parse(input);
        cast_rays(&mut map, 0, 0, RayDirection::Rightward);
        assert_eq!(46, part_one(input));
        assert_eq!(51, part_two(input));
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_16.txt");
        assert_eq!(7392, part_one(input));
        assert_eq!(7665, part_two(input));
    }
}
