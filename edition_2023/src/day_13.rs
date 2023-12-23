pub fn part_one(patterns: &str) -> usize {
    patterns
        .split("\n\n")
        .flat_map(|pattern| find_symetry(pattern))
        .map(|symetry| symetry.part_one_value())
        .sum()
}

#[derive(Debug)]
enum Symmetry {
    Vertical(usize),
    Horizontal(usize),
}

impl Symmetry {
    fn part_one_value(&self) -> usize {
        match self {
            Self::Vertical(col) => *col,
            Self::Horizontal(line) => line * 100,
        }
    }
}

type Matrix = Vec<Vec<char>>;

fn find_symetry(pattern: &str) -> Option<Symmetry> {
    let grid = pattern
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    find_symetry_lines(&grid)
        .map(Symmetry::Horizontal)
        .or_else(|| find_symetry_lines(&transpose(grid)).map(Symmetry::Vertical))
}

fn transpose(matrix: Matrix) -> Matrix {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    // Create a new matrix with dimensions swapped
    let mut transposed = vec![vec![' '; rows]; cols];

    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed[j][i] = val;
        }
    }

    transposed
}

fn find_symetry_lines(matrix: &Matrix) -> Option<usize> {
    for (i, _) in matrix.iter().enumerate().skip(1) {
        let mut is_symetrical = true;

        for index in 0..i {
            let left_index = i + index;
            if left_index >= matrix.len() {
                break;
            }

            if matrix[i - index - 1] != matrix[left_index] {
                is_symetrical = false;
                break;
            }
        }

        if is_symetrical {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let grid = "\
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#";

        assert_eq!(405, part_one(grid));
    }

    #[test]
    fn real() {
        let grid = include_str!("../res/day_13.txt");

        assert_eq!(8062, part_one(grid));
    }
}
