use num::integer::lcm;
use std::collections::HashMap;

pub fn part_one(map_str: &str) -> usize {
    let map = GhostMap::parse_map(map_str);

    map.find_node(0, map.find_aaa_index(), |node| node.name == "ZZZ")
}

pub fn part_two(map_str: &str) -> usize {
    let map = GhostMap::parse_map(map_str);

    map.nodes
        .iter()
        .filter(|n| n.name.ends_with('A'))
        .map(|n| map.find_node(0, n.id, |node| node.name.ends_with('Z')))
        .reduce(lcm)
        .unwrap()
}

#[derive(Debug)]
enum Command {
    Left,
    Right,
}

impl Command {
    fn parse(command_str: char) -> Self {
        match command_str {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid command {}", command_str),
        }
    }
}

#[derive(Debug)]
struct GhostMap<'a> {
    commands: Vec<Command>,
    nodes: Vec<Node<'a>>,
    cache: HashMap<String, usize>,
}

impl<'a> GhostMap<'a> {
    fn parse_map(map_str: &'a str) -> Self {
        let mut lines = map_str.lines();
        let first_line = lines.next().unwrap();

        let commands = first_line.chars().map(Command::parse).collect::<Vec<_>>();
        lines.next();

        let mut cache = HashMap::new();

        let nodes = lines
            .enumerate()
            .map(|(id, line)| {
                let node = Node::parse(line, id);
                cache.entry(node.name.to_owned()).or_insert(node.id);
                node
            })
            .collect::<Vec<_>>();

        Self {
            commands,
            nodes,
            cache,
        }
    }

    fn find_aaa_index(&self) -> usize {
        *self.cache.get("AAA").unwrap()
    }

    fn find_node(
        &self,
        current_step: usize,
        start_index: usize,
        check_finished: impl Fn(&Node<'a>) -> bool,
    ) -> usize {
        let mut current_node = &self.nodes[start_index];
        let mut next_index = start_index;
        let mut steps = current_step;

        for command in &self.commands {
            let next = match command {
                Command::Left => current_node.left,
                Command::Right => current_node.right,
            };

            next_index = *self.cache.get(next).unwrap();
            current_node = &self.nodes[next_index];
            steps += 1;
        }

        if check_finished(current_node) {
            steps
        } else {
            self.find_node(steps, next_index, check_finished)
        }
    }
}

#[derive(Debug, Clone)]
struct Node<'a> {
    name: &'a str,
    id: usize,
    left: &'a str,
    right: &'a str,
}

impl<'a> Node<'a> {
    fn parse(line: &'a str, id: usize) -> Self {
        let words = line.split_whitespace().collect::<Vec<_>>();

        let name = words[0];
        let left = &words[2][1..words[2].len() - 1];
        let right = &words[3][0..words[3].len() - 1];

        Self {
            name,
            id,
            left,
            right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_one() {
        let map_str = "\
        LLR

        BBB = (AAA, ZZZ)
        AAA = (BBB, BBB)
        ZZZ = (ZZZ, ZZZ)";

        assert_eq!(6, part_one(map_str));

        let map_str = "\
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

        assert_eq!(2, part_one(map_str));
    }

    #[test]
    fn example_part_two() {
        let map_str = "\
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

        assert_eq!(6, part_two(map_str));
    }

    #[test]
    fn real() {
        let map_str = include_str!("../res/day_08.txt");

        assert_eq!(16043, part_one(map_str));
        assert_eq!(15726453850399, part_two(map_str));
    }
}
