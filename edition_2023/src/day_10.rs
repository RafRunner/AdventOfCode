use std::{cell::RefCell, rc::Rc, vec};

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
            _ => (false, false, false, false),
        }
    }
}

#[derive(Debug)]
struct Pipe {
    kind: PipeType,
    line: usize,
    column: usize,
    distance: Option<usize>,
    connections: Option<(Rc<RefCell<Pipe>>, Rc<RefCell<Pipe>>)>,
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

    fn get_up(&self, pipe_world: &PipeWorld) -> Option<Rc<RefCell<Self>>> {
        if let Some(pipe) = pipe_world
            .get(self.line - 1)
            .map(|l| Rc::clone(&l[self.column]))
        {
            if pipe.borrow().kind.connections().2 {
                Some(pipe)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_down(&self, pipe_world: &PipeWorld) -> Option<Rc<RefCell<Self>>> {
        if let Some(pipe) = pipe_world
            .get(self.line + 1)
            .map(|l| Rc::clone(&l[self.column]))
        {
            if pipe.borrow().kind.connections().2 {
                Some(pipe)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_left(&self, pipe_world: &PipeWorld) -> Option<Rc<RefCell<Self>>> {
        if let Some(pipe) = pipe_world[self.line].get(self.column - 1).map(Rc::clone) {
            if pipe.borrow().kind.connections().1 {
                Some(pipe)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_right(&self, pipe_world: &PipeWorld) -> Option<Rc<RefCell<Self>>> {
        if let Some(pipe) = pipe_world[self.line].get(self.column + 1).map(Rc::clone) {
            if pipe.borrow().kind.connections().3 {
                Some(pipe)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn connect(&mut self, pipe_world: &PipeWorld) {
        let connections = match self.kind {
            PipeType::Horizontal => self.get_left(pipe_world).zip(self.get_right(pipe_world)),
            PipeType::Vertical => self.get_up(pipe_world).zip(self.get_down(pipe_world)),
            PipeType::NorthEast => self.get_up(pipe_world).zip(self.get_right(pipe_world)),
            PipeType::NorthWest => self.get_up(pipe_world).zip(self.get_left(pipe_world)),
            PipeType::SouthWest => self.get_down(pipe_world).zip(self.get_left(pipe_world)),
            PipeType::SouthEast => self.get_down(pipe_world).zip(self.get_right(pipe_world)),
            PipeType::StartingPosition => {
                let cons = vec![
                    self.get_up(pipe_world),
                    self.get_right(pipe_world),
                    self.get_down(pipe_world),
                    self.get_left(pipe_world),
                ];

                let mut filtered = cons.into_iter().flatten().collect::<Vec<_>>();

                Some((filtered.remove(0), filtered.remove(0)))
            }
            _ => None,
        };

        self.connections = connections;
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

fn find_connections_and_distances(pipe_world: &PipeWorld) {
    let starting = pipe_world
        .iter()
        .filter_map(|line| {
            line.iter()
                .find(|p| p.borrow().kind == PipeType::StartingPosition)
        })
        .take(1)
        .collect::<Vec<_>>();

    let starting = starting.first().unwrap();
    starting.borrow_mut().connect(pipe_world);

    dbg!(starting);
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

        let pipe_world = parse_pipe_world(input);
        find_connections_and_distances(&pipe_world);
    }
}
