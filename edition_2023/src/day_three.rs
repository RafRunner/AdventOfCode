use std::ops::Range;

#[derive(Debug)]
pub struct PartNumber {
    number: usize,
    line: usize,
    columns: Range<usize>,
    is_part_number: bool,
}

impl PartNumber {
    pub fn is_touching(&self, symbol: &Symbol) -> bool {
        let range_plus_one =
            self.columns.start.saturating_sub(1)..self.columns.end.saturating_add(1);
        symbol.line.abs_diff(self.line) < 2 && range_plus_one.contains(&symbol.column)
    }
}

#[derive(Debug)]
pub struct Symbol {
    char: char,
    line: usize,
    column: usize,
}

#[derive(Debug)]
// both vec are on order of encounter
pub struct EngineSchema {
    parts: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

impl EngineSchema {
    pub fn sum_parts(&self) -> usize {
        self.parts
            .iter()
            .filter(|p| p.is_part_number)
            .map(|p| p.number)
            .sum()
    }

    pub fn filter_active_parts(&mut self) {
        for part in self.parts.iter_mut() {
            if self.symbols.iter().any(|s| part.is_touching(s)) {
                part.is_part_number = true;
            }
        }
    }
}

pub fn parse_engine_schema(schema_str: &str) -> EngineSchema {
    let mut parts = Vec::<PartNumber>::new();
    let mut symbols = Vec::<Symbol>::new();

    let lines = schema_str.lines();

    for (line_number, line) in lines.enumerate() {
        let column_iter: Vec<(usize, char)> = line.char_indices().collect();
        let mut i = 0;

        loop {
            let (column_number, char) = column_iter[i];

            match char {
                char if char.is_ascii_digit() => {
                    let mut number_str = String::new();
                    number_str.push(char);

                    let mut j = i + 1;

                    while let Some((_, char)) = column_iter.get(j) {
                        if !char.is_numeric() {
                            break;
                        }

                        number_str.push(*char);
                        j += 1;
                    }

                    parts.push(PartNumber {
                        number: number_str.parse().unwrap(),
                        line: line_number,
                        columns: (i..j),
                        is_part_number: false,
                    });

                    i += number_str.len() - 1;
                }
                '.' => (),
                _ => {
                    symbols.push(Symbol {
                        char,
                        line: line_number,
                        column: column_number,
                    });
                }
            }

            i += 1;
            if i >= line.len() {
                break;
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
    fn example_one() {
        let input = "467..114..
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
    }

    #[test]
    fn edge_cases_one() {
        let input = ".../.............*........../......................*..............658..........*718..........*136.....................503.899....889.498....
....691........341.262..36.549...........386........437.............................662...........848............#......*...................
.......................*..........936...*...............................-...........*......516....%......358....707..535...........841......";

        let schema = parse_engine_schema(input);

        assert_eq!(
            Some(true),
            schema
                .parts
                .iter()
                .find(|p| p.number == 386)
                .map(|p| p.is_part_number)
        );
    }

    #[test]
    fn full_schema_one() {
        let input = include_str!("../res/day_three.txt");

        let schema = parse_engine_schema(input);

        assert_eq!(533775, schema.sum_parts());
    }
}
