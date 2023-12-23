use std::{collections::HashMap, sync::Mutex};

pub fn part_one(diagrams: &str) -> usize {
    let records = Record::parse(diagrams);

    records
        .into_iter()
        .map(|record| record.count_possible())
        .sum()
}

pub fn part_two(diagrams: &str) -> usize {
    let records = Record::parse(diagrams);

    records
        .into_iter()
        .map(|record| {
            let mut five_springs = Vec::new();

            for i in 0..5 {
                five_springs.extend(record.springs.clone());
                if i != 4 {
                    five_springs.push(SpringType::Unknown);
                }
            }

            Record {
                springs: five_springs,
                groups: record
                    .groups
                    .iter()
                    .cycle()
                    .take(record.groups.len() * 5)
                    .cloned()
                    .collect(),
            }
        })
        .map(|record| record.count_possible())
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
            _ => panic!("Invalid spring type {}", spring_char),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Record {
    springs: Vec<SpringType>,
    groups: Vec<usize>,
}

lazy_static! {
    static ref CACHE: Mutex<HashMap<Record, usize>> = Mutex::new(HashMap::new());
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
    fn count_possible(&self) -> usize {
        // Handle base cases
        match (self.springs.is_empty(), self.groups.is_empty()) {
            (true, true) => return 1,  // We're done
            (true, false) => return 0, // No possible spring to match the group
            // No more groups to match a broken spring. Else we have 1 possibility: all functional
            (false, true) => {
                return if self.springs.contains(&SpringType::Broken) {
                    0
                } else {
                    1
                }
            }
            _ => (),
        }

        // Check cache
        if let Some(&total) = CACHE.lock().unwrap().get(self) {
            return total;
        }

        let mut total = 0;
        let spring = &self.springs[0];

        // Functional or Unknown spring case. Just check the rest
        if *spring == SpringType::Functional || *spring == SpringType::Unknown {
            let partial = Self {
                springs: self.springs.iter().skip(1).cloned().collect(),
                groups: self.groups.clone(),
            };
            total += partial.count_possible();
        }

        // Broken or Unknown spring case. Check if the group could match and then check the rest removing the group
        if (*spring == SpringType::Broken || *spring == SpringType::Unknown)
            && self.groups[0] <= self.springs.len()
            && !self
                .springs
                .iter()
                .take(self.groups[0])
                .any(|s| *s == SpringType::Functional)
            && self
                .springs
                .get(self.groups[0])
                .map_or(true, |s| *s != SpringType::Broken)
        {
            let partial = Self {
                springs: self
                    .springs
                    .iter()
                    .skip(self.groups[0] + 1)
                    .cloned()
                    .collect(),
                groups: self.groups.iter().skip(1).cloned().collect(),
            };
            total += partial.count_possible();
        }

        // Update cache
        CACHE.lock().unwrap().insert(self.clone(), total);
        total
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
        assert_eq!(525152, part_two(diagram));
    }

    #[test]
    fn real() {
        let diagram = include_str!("../res/day_12.txt");

        assert_eq!(7674, part_one(diagram));
    }

    #[test]
    fn real_part_two() {
        let diagram = include_str!("../res/day_12.txt");

        assert_eq!(4443895258186, part_two(diagram));
    }
}
