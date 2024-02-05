use crate::common::Point;

pub fn part_one(dig_plan: &str) -> usize {
    let plans: Vec<DigPlanLine> = dig_plan
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.trim().split(' ').collect();

            let direction = Direction::parse(parts[0].chars().nth(0).unwrap());

            DigPlanLine {
                direction,
                leght: parts[1].parse().unwrap(),
                color: String::from(parts[2]),
            }
        })
        .collect();

    let mut position = Point::new(0, 0);
    let mut points = Vec::new();
    let mut perimeter = 0;

    for plan in plans {
        points.push(position.clone());
        match plan.direction {
            Direction::Up => position.y -= plan.leght as isize,
            Direction::Down => position.y += plan.leght as isize,
            Direction::Left => position.x -= plan.leght as isize,
            Direction::Right => position.x += plan.leght as isize,
        }
        perimeter += plan.leght;
    }

    let area = Point::shoelace_area(&points) as usize;

    // Pick's Theorem
    // Area = Inside + InEdge/2  - 1
    // Inside = Area - InEdge/2  + 1

    let inside = area - perimeter / 2 + 1;

    perimeter + inside
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(char: char) -> Self {
        match char {
            'U' => Self::Up,
            'D' => Self::Down,
            'R' => Self::Right,
            'L' => Self::Left,
            _ => panic!("Unexpected char for diretion: {char}"),
        }
    }
}

#[derive(Debug, Clone)]
struct DigPlanLine {
    direction: Direction,
    leght: usize,
    color: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let dig_plan = "\
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)";

        assert_eq!(62, part_one(dig_plan));
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_18.txt");

        assert_eq!(35401, part_one(input));
    }
}
