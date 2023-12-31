pub fn part_one(calibration_text: &str) -> usize {
    find_calibration_sum(calibration_text, find_numbers)
}

pub fn part_two(calibration_text: &str) -> usize {
    find_calibration_sum(calibration_text, find_numbers_with_names)
}

fn find_calibration_sum<NumberFinder>(text: &str, f: NumberFinder) -> usize
where
    NumberFinder: Fn(&str) -> Vec<usize>,
{
    text.lines()
        .map(f)
        .map(combine_to_two_digit)
        .fold(0, |acc, number| acc + number.unwrap_or(0))
}

fn find_numbers(line: &str) -> Vec<usize> {
    line.chars()
        .filter(|c| c.is_numeric())
        .map(|n| (n as usize) - ('0' as usize))
        .collect()
}

const NUMBER_WORDS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

fn find_numbers_with_names(line: &str) -> Vec<usize> {
    let mut replaced = line.to_owned();

    for (number, digit) in NUMBER_WORDS {
        replaced = replaced.replace(number, digit);
    }

    find_numbers(&replaced)
}

fn combine_to_two_digit(numbers: Vec<usize>) -> Option<usize> {
    let first_and_last = numbers.first().map(|f| (f, numbers.last().unwrap()));

    first_and_last.map(|(f, l)| f * 10 + l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_digits() {
        let text = "\
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        let result = part_one(text);
        assert_eq!(142, result);
    }

    #[test]
    fn full_digits() {
        let text = include_str!("../res/day_01.txt");

        let result = part_one(text);
        assert_eq!(54916, result);
    }

    #[test]
    fn simple_digits_and_numbers() {
        let text = "\
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        let result = part_two(text);
        assert_eq!(281, result);
    }

    #[test]
    fn digits_and_numbers_edge() {
        assert_eq!(55, part_two("five"));
        assert_eq!(33, part_two("threefivethree"));
        assert_eq!(82, part_two("eightwo"));
        assert_eq!(58, part_two("fiveeight3sppjtccnineeighteightnffgtlsdj"));
        assert_eq!(33, part_two("threethreetwothree"));
    }

    #[test]
    fn full_digits_and_numbers() {
        let text = include_str!("../res/day_01.txt");

        let result = part_two(text);
        assert_eq!(54728, result);
    }
}
