use std::{cell::RefCell, rc::Rc, vec};

pub fn part_one(maze: &str) -> usize {
    let pipe_world = parse_pipe_world(maze);
    find_connections_and_distances(&pipe_world);

    let max = pipe_world
        .iter()
        .flatten()
        .filter_map(|p| p.borrow().distance)
        .max()
        .unwrap_or(0);

    (max + 1) / 2
}

pub fn part_two(maze: &str) -> usize {
    let pipe_world = parse_pipe_world(maze);
    let vertices = find_connections_and_distances(&pipe_world);
    let lines = pipe_world.len();
    let columns = pipe_world[0].len();

    let points = vertices
        .iter()
        .map(|pipe| Point::new(columns - pipe.borrow().column, lines - pipe.borrow().line))
        .collect::<Vec<_>>();

    let area = shoelace_area(&points);

    // Pick's Theorem
    // Area = Inside + InEdge/2  - 1
    // Inside = Area - InEdge/2  + 1

    (area - vertices.len() as f64 / 2.0 + 1.0) as usize
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }
}

fn shoelace_area(points: &[Point]) -> f64 {
    let sum = points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(p1, p2)| p1.x * p2.y - p2.x * p1.y)
        .sum::<isize>()
        .abs();

    sum as f64 / 2.0
}

#[derive(Debug, PartialEq, Eq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPosition,
}

impl PipeType {
    fn parse(char: char) -> Self {
        match char {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::StartingPosition,
            _ => panic!("Unknow pipe {}", char),
        }
    }

    // North, East, South, West
    fn connections(&self) -> (bool, bool, bool, bool) {
        match self {
            Self::Vertical => (true, false, true, false),
            Self::Horizontal => (false, true, false, true),
            Self::NorthEast => (true, true, false, false),
            Self::NorthWest => (true, false, false, true),
            Self::SouthWest => (false, false, true, true),
            Self::SouthEast => (false, true, true, false),
            Self::StartingPosition => (true, true, true, true),
            _ => (false, false, false, false),
        }
    }
}

#[derive(Debug)]
struct Connection {
    pipe: Rc<RefCell<Pipe>>,
}

impl Connection {
    fn new(pipe: Rc<RefCell<Pipe>>) -> Self {
        Self { pipe }
    }
}

#[derive(Debug)]
struct Pipe {
    kind: PipeType,
    line: usize,
    column: usize,
    distance: Option<usize>,
    connections: Option<(Connection, Connection)>,
}

impl Pipe {
    fn new(kind: PipeType, line: usize, column: usize) -> Self {
        Self {
            kind,
            line,
            column,
            distance: None,
            connections: None,
        }
    }

    fn get_up(&self, pipe_world: &PipeWorld) -> Option<Connection> {
        if self.line == 0 {
            None
        } else {
            let pipe = Rc::clone(&pipe_world[self.line - 1][self.column]);

            if pipe.borrow().kind.connections().2 {
                Some(Connection::new(pipe))
            } else {
                None
            }
        }
    }

    fn get_down(&self, pipe_world: &PipeWorld) -> Option<Connection> {
        pipe_world
            .get(self.line + 1)
            .map(|l| &l[self.column])
            .filter(|pipe| pipe.borrow().kind.connections().0)
            .map(|pipe| Connection::new(Rc::clone(pipe)))
    }

    fn get_left(&self, pipe_world: &PipeWorld) -> Option<Connection> {
        if self.column == 0 {
            None
        } else {
            let pipe = Rc::clone(&pipe_world[self.line][self.column - 1]);

            if pipe.borrow().kind.connections().1 {
                Some(Connection::new(pipe))
            } else {
                None
            }
        }
    }

    fn get_right(&self, pipe_world: &PipeWorld) -> Option<Connection> {
        pipe_world[self.line]
            .get(self.column + 1)
            .filter(|pipe| pipe.borrow().kind.connections().3)
            .map(|pipe| Connection::new(Rc::clone(pipe)))
    }

    fn connect(&mut self, pipe_world: &PipeWorld) {
        let mut connections = Vec::new();
        let directions = self.kind.connections();

        if directions.0 {
            connections.push(self.get_up(pipe_world));
        }
        if directions.1 {
            connections.push(self.get_right(pipe_world));
        }
        if directions.2 {
            connections.push(self.get_down(pipe_world));
        }
        if directions.3 {
            connections.push(self.get_left(pipe_world))
        }

        let mut filtered = connections.into_iter().flatten().collect::<Vec<_>>();

        if filtered.len() >= 2 {
            self.connections = Some((filtered.remove(0), filtered.remove(0)));
        }
    }
}

impl PartialEq for Pipe {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }
}

type PipeWorld = Vec<Vec<Rc<RefCell<Pipe>>>>;

fn parse_pipe_world(input: &str) -> PipeWorld {
    input
        .lines()
        .enumerate()
        .map(|(line_number, line)| {
            line.trim()
                .chars()
                .enumerate()
                .map(|(column_number, column)| {
                    let kind = PipeType::parse(column);

                    Rc::new(RefCell::new(Pipe::new(kind, line_number, column_number)))
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_connections_and_distances(pipe_world: &PipeWorld) -> Vec<Rc<RefCell<Pipe>>> {
    let starting = pipe_world
        .iter()
        .flatten()
        .find(|p| p.borrow().kind == PipeType::StartingPosition);

    let starting = starting.unwrap();
    visit_connections(Rc::clone(starting), pipe_world)
}

fn visit_connections(
    starting: Rc<RefCell<Pipe>>,
    pipe_world: &PipeWorld,
) -> Vec<Rc<RefCell<Pipe>>> {
    starting.borrow_mut().distance = Some(0);

    let mut pipe_loop = Vec::new();
    let mut stack = vec![starting];

    while let Some(current) = stack.pop() {
        pipe_loop.push(Rc::clone(&current));
        let distance = current
            .borrow()
            .distance
            .expect("My distance should be known...");
        current.borrow_mut().connect(pipe_world);

        let connections = &current.borrow().connections;

        let (ref con1, ref con2) = connections.as_ref().expect("I should have connections");
        let pipe1 = Rc::clone(&con1.pipe);
        let pipe2 = Rc::clone(&con2.pipe);

        if pipe1.borrow().distance.is_none() {
            pipe1.borrow_mut().distance = Some(distance + 1);
            stack.push(pipe1);
        } else if pipe2.borrow().distance.is_none() {
            pipe2.borrow_mut().distance = Some(distance + 1);
            stack.push(pipe2);
        }
    }

    pipe_loop
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "\
        .....
        .S-7.
        .|.|.
        .L-J.
        .....";

        assert_eq!(4, part_one(input));
        assert_eq!(1, part_two(input));

        let input = "\
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...";

        assert_eq!(8, part_one(input));
        assert_eq!(1, part_two(input));
    }

    #[test]
    fn test_shoelace() {
        assert_eq!(
            16.5,
            shoelace_area(&vec![
                Point::new(1, 6),
                Point::new(3, 1),
                Point::new(7, 2),
                Point::new(4, 4),
                Point::new(8, 5)
            ])
        );
    }

    #[test]
    fn example_two() {
        let input = "\
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";

        assert_eq!(4, part_two(input));

        let input = "\
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";

        assert_eq!(8, part_two(input));
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_10.txt");

        assert_eq!(6856, part_one(input));
        assert_eq!(501, part_two(input));
    }
}
