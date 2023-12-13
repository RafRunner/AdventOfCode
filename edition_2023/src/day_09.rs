pub fn part_one(changes: &str) -> isize {
    sum_predictions(changes, |oasis| oasis.predict_next())
}

pub fn part_two(changes: &str) -> isize {
    sum_predictions(changes, |oasis| oasis.predict_next_back())
}

fn sum_predictions(changes: &str, predition: impl Fn(&OasisSequence) -> isize) -> isize {
    changes
        .lines()
        .map(OasisSequence::parse)
        .map(|oasis| predition(&oasis))
        .sum()
}

#[derive(Debug)]
struct OasisSequence {
    derived: Vec<Vec<isize>>,
}

impl OasisSequence {
    fn parse(line: &str) -> Self {
        let sequence: Vec<isize> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let mut current_sequence = sequence.clone();
        let mut derived = vec![sequence];

        while current_sequence.iter().any(|n| *n != 0) {
            let mut sequence_iter = current_sequence.into_iter();
            let mut before = sequence_iter.next().unwrap_or(0);
            let mut new_sequence = Vec::new();

            for item in sequence_iter {
                new_sequence.push(item - before);
                before = item;
            }

            current_sequence = new_sequence.clone();
            derived.push(new_sequence);
        }

        Self { derived }
    }

    fn predict(&self, folding: impl Fn(isize, &Vec<isize>) -> isize) -> isize {
        self.derived.iter().rev().skip(1).fold(0, folding)
    }

    fn predict_next(&self) -> isize {
        self.predict(|acc, elem| acc + elem.last().unwrap())
    }

    fn predict_next_back(&self) -> isize {
        self.predict(|acc, elem| elem.first().unwrap() - acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

        let sequence = OasisSequence::parse("0 3 6 9 12 15");
        assert_eq!(18, sequence.predict_next());
        assert_eq!(-3, sequence.predict_next_back());

        let sequence = OasisSequence::parse("1 3 6 10 15 21");
        assert_eq!(28, sequence.predict_next());
        assert_eq!(0, sequence.predict_next_back());

        let sequence = OasisSequence::parse("10 13 16 21 30 45");
        assert_eq!(68, sequence.predict_next());
        assert_eq!(5, sequence.predict_next_back());

        assert_eq!(114, part_one(input));
        assert_eq!(2, part_two(input));
    }

    #[test]
    fn real() {
        let input = include_str!("../res/day_09.txt");

        assert_eq!(2043183816, part_one(input));
        assert_eq!(1118, part_two(input));
    }
}
