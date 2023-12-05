use std::ops::Range;

pub fn part_one(engine_schema_str: &str) -> usize {
    let schema = parse_engine_schema(engine_schema_str);
    schema.sum_parts()
}

pub fn part_two(engine_schema_str: &str) -> usize {
    let schema = parse_engine_schema(engine_schema_str);
    schema.gear_power()
}

#[derive(Debug, PartialEq)]
struct PartNumber {
    number: usize,
    line: usize,
    columns: Range<usize>,
    is_part_number: bool,
}

impl PartNumber {
    fn is_touching(&self, symbol: &Symbol) -> bool {
        symbol.line.abs_diff(self.line) < 2 && self.range_of_contact().contains(&symbol.column)
    }

    fn range_of_contact(&self) -> Range<usize> {
        self.columns.start.saturating_sub(1)..(self.columns.end + 1)
    }
}

#[derive(Debug)]
struct Symbol {
    char: char,
    line: usize,
    column: usize,
    gear_power: Option<usize>,
}

#[derive(Debug)]
// both vecs are on order of encounter
struct EngineSchema {
    parts: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

impl EngineSchema {
    fn sum_parts(&self) -> usize {
        self.parts
            .iter()
            .filter(|p| p.is_part_number)
            .map(|p| p.number)
            .sum()
    }

    fn gear_power(&self) -> usize {
        self.symbols.iter().filter_map(|s| s.gear_power).sum()
    }

    fn filter_active_parts(&mut self) {
        for symbol in self.symbols.iter_mut() {
            let part_neighbors = self
                .parts
                .iter_mut()
                .take_while(|p| p.line < symbol.line + 2)
                .filter(|p| p.is_touching(symbol))
                .collect::<Vec<_>>();

            if symbol.char == '*' && part_neighbors.len() == 2 {
                symbol.gear_power = Some(part_neighbors[0].number * part_neighbors[1].number);
            }
            for part in part_neighbors {
                part.is_part_number = true;
            }
        }
    }
}

fn parse_engine_schema(schema_str: &str) -> EngineSchema {
    let mut parts = Vec::<PartNumber>::new();
    let mut symbols = Vec::<Symbol>::new();

    for (line_number, line) in schema_str.lines().enumerate() {
        let mut column_iter = line.chars().enumerate().peekable();

        while let Some((column_number, char)) = column_iter.next() {
            match char {
                char if char.is_ascii_digit() => {
                    let mut number_str = String::new();
                    number_str.push(char);

                    while let Some((_, char)) =
                        column_iter.peek().filter(|(_, c)| c.is_ascii_digit())
                    {
                        number_str.push(*char);
                        column_iter.next();
                    }

                    parts.push(PartNumber {
                        number: number_str.parse().unwrap(),
                        line: line_number,
                        columns: (column_number..(column_number + number_str.len())),
                        is_part_number: false,
                    });
                }
                '.' => (),
                _ => {
                    symbols.push(Symbol {
                        char,
                        line: line_number,
                        column: column_number,
                        gear_power: None,
                    });
                }
            }
        }
    }

    let mut schema = EngineSchema { parts, symbols };
    schema.filter_active_parts();
    schema
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let schema = parse_engine_schema(input);

        assert_eq!(6, schema.symbols.len());
        assert_eq!(4361, schema.sum_parts());
        assert_eq!(467835, schema.gear_power());
    }

    #[test]
    fn example_unofficial() {
        let input = "\
12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";

        let schema = parse_engine_schema(input);

        assert_eq!(925, schema.sum_parts());
        assert_eq!(6756, schema.gear_power());
    }

    #[test]
    fn edge_cases_one() {
        let input = "\
.../.............*........../......................*..............658..........*718..........*136.....................503.899....889.498....
....691........341.262..36.549...........386........437.............................662...........848............#......*...................
.......................*..........936...*...............................-...........*......516....%......358....707..535...........841......";

        let schema = parse_engine_schema(input);

        assert_eq!(
            Some(&PartNumber {
                number: 386,
                line: 1,
                columns: (41..44),
                is_part_number: true
            }),
            schema.parts.get(12)
        );
        assert_eq!(
            Some(&PartNumber {
                number: 936,
                line: 2,
                columns: (34..37),
                is_part_number: false
            }),
            schema.parts.get(16)
        );
        assert_eq!(
            Some(&PartNumber {
                number: 535,
                line: 2,
                columns: (117..120),
                is_part_number: true
            }),
            schema.parts.get(20)
        );
    }

    #[test]
    fn full_schema() {
        let input = include_str!("../res/day_three.txt");

        let schema = parse_engine_schema(input);

        assert_eq!(533775, schema.sum_parts());
        assert_eq!(78236071, schema.gear_power());
    }
}
