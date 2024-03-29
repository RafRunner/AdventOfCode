use crate::common::Point;

pub fn part_one(dig_plan: &str) -> usize {
    calculare_area(dig_plan, |direction, length, _| DigPlanLine {
        direction,
        length,
    })
}

pub fn part_two(dig_plan: &str) -> usize {
    calculare_area(dig_plan, |_, _, color| {
        let length_str = &color[2..color.len() - 2];
        let direction_char = color.chars().nth(color.len() - 2).unwrap();

        let direction = Direction::parse_part_two(direction_char);

        DigPlanLine {
            direction,
            length: usize::from_str_radix(length_str, 16).unwrap(),
        }
    })
}

fn calculare_area<F>(dig_plan: &str, plan_builder: F) -> usize
where
    F: Fn(Direction, usize, String) -> DigPlanLine,
{
    let plans: Vec<DigPlanLine> = dig_plan
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.trim().split(' ').collect();
            let direction = Direction::parse(parts[0].chars().next().unwrap());
            let color = String::from(parts[2]);

            plan_builder(direction, parts[1].parse().unwrap(), color)
        })
        .collect();

    let mut position = Point::new(0, 0);
    let mut points = Vec::new();
    let mut perimeter = 0;

    for plan in plans {
        points.push(position.clone());
        match plan.direction {
            Direction::Up => position.y -= plan.length as isize,
            Direction::Down => position.y += plan.length as isize,
            Direction::Left => position.x -= plan.length as isize,
            Direction::Right => position.x += plan.length as isize,
        }
        perimeter += plan.length;
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

    fn parse_part_two(char: char) -> Self {
        match char {
            '0' => Self::Right,
            '1' => Self::Down,
            '2' => Self::Left,
            '3' => Self::Up,
            _ => panic!("Invalid direction character"),
        }
    }
}

#[derive(Debug, Clone)]
struct DigPlanLine {
    direction: Direction,
    length: usize,
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
        assert_eq!(952408144115, part_two(dig_plan));
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_18.txt");

        assert_eq!(35401, part_one(input));
        assert_eq!(48020869073824, part_two(input));
    }
}
