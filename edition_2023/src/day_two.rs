use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Game {
    id: usize,
    guesses: Vec<Guess>,
}

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Guess {
    reds: usize,
    greens: usize,
    blues: usize,
}

impl Guess {
    pub fn new(reds: usize, greens: usize, blues: usize) -> Self {
        Self {
            reds,
            greens,
            blues,
        }
    }

    pub fn is_compatible(&self, other: &Self) -> bool {
        self.reds <= other.reds && self.greens <= other.greens && self.blues <= other.blues
    }

    pub fn power(&self) -> usize {
        self.reds * self.greens * self.blues
    }
}

pub fn find_possible_games(correct_guess: &Guess, games: &[Game]) -> Vec<usize> {
    games
        .iter()
        .filter_map(|game| {
            if game
                .guesses
                .iter()
                .all(|guess| guess.is_compatible(correct_guess))
            {
                Some(game.id)
            } else {
                dbg!(game);
                None
            }
        })
        .collect()
}

pub fn find_smallest_possible_guess(games: &[Game]) -> Vec<Guess> {
    games
        .iter()
        .map(|game| {
            let mut smallest = Guess::default();

            for guess in &game.guesses {
                if guess.reds > smallest.reds {
                    smallest.reds = guess.reds;
                }
                if guess.greens > smallest.greens {
                    smallest.greens = guess.greens;
                }
                if guess.blues > smallest.blues {
                    smallest.blues = guess.blues;
                }
            }

            smallest
        })
        .collect()
}

pub fn read_file(file: &str) -> Vec<Game> {
    let id_regex = Regex::new(r"^Game (?<id>\d+): ").unwrap();
    let guess_regex = Regex::new(r"(?<quantity>\d+) (?<color>\w+)").unwrap();

    file.lines()
        .filter_map(|line| {
            let line = line.trim();
            let captures = id_regex.captures(line)?;
            let id = str::parse(&captures["id"]).ok()?;
            let guesses_str = id_regex.replace(line, "");

            let guesses: Vec<Guess> = guesses_str
                .split(';')
                .map(|guess_str| {
                    let mut guess = Guess::default();

                    for color in guess_regex.captures_iter(guess_str) {
                        let number: usize = str::parse(&color["quantity"]).unwrap();

                        match &color["color"] {
                            "red" => guess.reds = number,
                            "blue" => guess.blues = number,
                            "green" => guess.greens = number,
                            _ => panic!("Unexpected color: {}", &color[1]),
                        }
                    }

                    guess
                })
                .collect();

            Some(Game { id, guesses })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exemple_one() {
        let games = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = read_file(games);
        let mut iter = games.iter();

        assert_eq!(
            &Game {
                id: 1,
                guesses: vec![
                    Guess::new(4, 0, 3),
                    Guess::new(1, 2, 6),
                    Guess::new(0, 2, 0)
                ]
            },
            iter.next().unwrap()
        );
        assert_eq!(
            &Game {
                id: 2,
                guesses: vec![
                    Guess::new(0, 2, 1),
                    Guess::new(1, 3, 4),
                    Guess::new(0, 1, 1)
                ]
            },
            iter.next().unwrap()
        );
        assert_eq!(
            &Game {
                id: 3,
                guesses: vec![
                    Guess::new(20, 8, 6),
                    Guess::new(4, 13, 5),
                    Guess::new(1, 5, 0)
                ]
            },
            iter.next().unwrap()
        );
        assert_eq!(
            &Game {
                id: 4,
                guesses: vec![
                    Guess::new(3, 1, 6),
                    Guess::new(6, 3, 0),
                    Guess::new(14, 3, 15)
                ]
            },
            iter.next().unwrap()
        );
        assert_eq!(
            &Game {
                id: 5,
                guesses: vec![Guess::new(6, 3, 1), Guess::new(1, 2, 2)]
            },
            iter.next().unwrap()
        );
        assert_eq!(None, iter.next());

        let correct_guess = Guess {
            reds: 12,
            blues: 14,
            greens: 13,
        };

        assert!([1, 2, 5]
            .iter()
            .eq(find_possible_games(&correct_guess, &games).iter()));
    }

    #[test]
    fn full_one() {
        let games = include_str!("../res/day_two.txt");

        let games = read_file(games);

        let correct_guess = Guess {
            reds: 12,
            blues: 14,
            greens: 13,
        };

        assert_eq!(100, games.len());
        assert_eq!(
            2685 as usize,
            find_possible_games(&correct_guess, &games).iter().sum()
        );
    }

    #[test]
    fn exemple_two() {
        let games = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = read_file(games);
        let powers: Vec<usize> = find_smallest_possible_guess(&games)
            .iter()
            .map(Guess::power)
            .collect();

        assert!([48, 12, 1560, 630, 36].iter().eq(powers.iter()));
    }

    #[test]
    fn full_two() {
        let games = include_str!("../res/day_two.txt");

        let games = read_file(games);
        let powers: usize = find_smallest_possible_guess(&games)
            .iter()
            .map(Guess::power)
            .sum();

        assert_eq!(83707, powers);
    }
}
