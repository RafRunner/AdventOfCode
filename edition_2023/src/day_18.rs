use crate::common::Point;

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

#[derive(Debug, Clone)]
enum Tile {
    Dig,
    NotDig,
}

#[derive(Debug, Clone)]
struct DigGrid {
    lines: Vec<Vec<Tile>>,
}

impl DigGrid {
    fn new() -> Self {
        DigGrid {
            lines: vec![vec![Tile::NotDig]],
        }
    }

    fn extend_line(&mut self, lines: usize, up: bool) {
        let line_size = self.lines.get(0).map(|l| l.len()).unwrap_or(0);

        for _ in 0..lines {
            let new_line = vec![Tile::NotDig; line_size];
            if up {
                self.lines.insert(0, new_line);
            } else {
                self.lines.push(new_line);
            }
        }
    }

    fn extend_column(&mut self, cols: usize, right: bool) {
        let column_size = self.lines.len();

        for _ in 0..cols {
            for i in 0..column_size {
                if right {
                    self.lines[i].push(Tile::NotDig);
                } else {
                    self.lines[i].insert(0, Tile::NotDig);
                }
            }
        }
    }

    fn lines_len(&self) -> usize {
        self.lines.len()
    }

    fn cols(&self) -> usize {
        self.lines[0].len()
    }
}

fn build_grid(dig_plan: &str) -> DigGrid {
    let mut dig_grid = DigGrid::new();

    let plans: Vec<DigPlanLine> = dig_plan
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.trim().split(' ').collect();

            let direction = Direction::parse(parts[0].chars().collect::<Vec<_>>()[0]);

            DigPlanLine {
                direction,
                leght: parts[1].parse().unwrap(),
                color: String::from(parts[2]),
            }
        })
        .collect();

    // dbg!(plans);

    let mut position = Point::new(0, 0);
    let mut points = Vec::new();

    for plan in plans {
        points.push(position.clone());
        match plan.direction {
            Direction::Up => position = Point::new(position.x, position.y - plan.leght as isize),
            Direction::Down => position = Point::new(position.x, position.y + plan.leght as isize),
            Direction::Left => position = Point::new(position.x - plan.leght as isize, position.y),
            Direction::Right => position = Point::new(position.x + plan.leght as isize, position.y),
        }
    }

    dbg!(&points);
    dbg!(Point::shoelace_area(&points));

    dig_grid.extend_line(1, true);
    dig_grid.extend_column(2, true);

    dig_grid
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

        build_grid(dig_plan);
    }
}
