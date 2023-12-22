#[derive(Debug)]
enum SpringType {
    Functional,
    Broken,
    Unknown,
}

impl SpringType {
    fn parse(spring_char: char) -> Self {
        match spring_char {
            '.' => Self::Functional,
            '#' => Self::Broken,
            '?' => Self::Unknown,
            _ => panic!("Unknow spring type {}", spring_char),
        }
    }
}

#[derive(Debug)]
struct Record {
    springs: Vec<SpringType>,
    groups: Vec<usize>,
}

impl Record {
    fn parse(diagrams: &str) -> Vec<Self> {
        diagrams
            .lines()
            .into_iter()
            .map(|line| {
                let line = line.trim();

                let mut parts = line.split_whitespace();

                let springs = parts
                    .next()
                    .unwrap()
                    .chars()
                    .map(SpringType::parse)
                    .collect::<Vec<_>>();
                let groups = parts
                    .next()
                    .unwrap()
                    .split(',')
                    .flat_map(|number| number.parse())
                    .collect::<Vec<usize>>();

                Self { springs, groups }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let diagram = "\
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

        let records = Record::parse(diagram);

        dbg!(&records);
    }
}
