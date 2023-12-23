pub fn part_one(diagrams: &str) -> usize {
    let records = Record::parse(diagrams);

    records
        .into_iter()
        .map(|record| {
            record
                .generate_possibilities()
                .into_iter()
                .filter(|rec| rec.is_possible())
                .count()
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<SpringType>,
    groups: Vec<usize>,
}

impl Record {
    fn parse(diagrams: &str) -> Vec<Self> {
        diagrams
            .lines()
            .flat_map(|line| {
                let line = line.trim();

                let mut parts = line.split_whitespace();

                let springs = parts
                    .next()?
                    .chars()
                    .map(SpringType::parse)
                    .collect::<Vec<_>>();
                let groups = parts
                    .next()?
                    .split(',')
                    .flat_map(|number| number.parse())
                    .collect::<Vec<usize>>();

                Some(Self { springs, groups })
            })
            .collect()
    }

    fn is_possible(&self) -> bool {
        let mut checked_groups = Vec::new();
        let mut current_count = 0_usize;

        for spring in &self.springs {
            if *spring == SpringType::Broken {
                current_count += 1;
            } else if current_count > 0 {
                checked_groups.push(current_count);
                current_count = 0;
            }
        }

        if current_count > 0 {
            checked_groups.push(current_count);
        }

        checked_groups.eq(&self.groups)
    }

    fn generate_possibilities(&self) -> Vec<Self> {
        if self.springs.contains(&SpringType::Unknown) {
            let mut variations = Vec::new();

            let first_unknow = self
                .springs
                .iter()
                .position(|s| *s == SpringType::Unknown)
                .unwrap();

            for possibility in [SpringType::Broken, SpringType::Functional] {
                let mut new_springs = self.springs.clone();
                let _ = std::mem::replace(&mut new_springs[first_unknow], possibility);

                let new_record = Self {
                    springs: new_springs,
                    groups: self.groups.clone(),
                };

                variations.extend(new_record.generate_possibilities());
            }

            variations
        } else {
            vec![self.clone()]
        }
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

        assert_eq!(21, part_one(diagram));
    }

    #[ignore = "Takes over a minute"]
    #[test]
    fn real() {
        let diagram = include_str!("../res/day_12.txt");

        assert_eq!(7674, part_one(diagram));
    }
}
