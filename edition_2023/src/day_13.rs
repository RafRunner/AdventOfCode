pub fn part_one(patterns: &str) -> usize {
    solve(patterns, 0)
}

pub fn part_two(patterns: &str) -> usize {
    solve(patterns, 1)
}

fn solve(patterns: &str, target: usize) -> usize {
    patterns
        .split("\n\n")
        .flat_map(|pattern| find_symetry(pattern, target))
        .map(|symetry| symetry.value())
        .sum()
}

#[derive(Debug)]
enum Symmetry {
    Vertical(usize),
    Horizontal(usize),
}

impl Symmetry {
    fn value(&self) -> usize {
        match self {
            Self::Vertical(col) => *col,
            Self::Horizontal(line) => line * 100,
        }
    }
}

type Matrix = Vec<Vec<char>>;

fn find_symetry(pattern: &str, target: usize) -> Option<Symmetry> {
    let grid = pattern
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    find_symetry_lines(&grid, target)
        .map(Symmetry::Horizontal)
        .or_else(|| find_symetry_lines(&transpose(grid), target).map(Symmetry::Vertical))
}

fn transpose(matrix: Matrix) -> Matrix {
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

fn find_symetry_lines(matrix: &Matrix, target: usize) -> Option<usize> {
    for line in 1..matrix.len() {
        let mut differences = 0;

        for index in 0..line {
            let left_index = line + index;
            if left_index >= matrix.len() {
                break;
            }

            differences += count_diferences(&matrix[line - index - 1], &matrix[left_index]);
            if differences > target {
                break;
            }
        }

        if differences == target {
            return Some(line);
        }
    }

    None
}

fn count_diferences(list_a: &[char], list_b: &[char]) -> usize {
    list_a
        .iter()
        .zip(list_b.iter())
        .map(|(a, b)| if a == b { 0 } else { 1 })
        .sum()
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
        assert_eq!(400, part_two(grid));
    }

    #[test]
    fn real() {
        let grid = include_str!("../res/day_13.txt");

        assert_eq!(36041, part_one(grid));
        assert_eq!(35915, part_two(grid));
    }
}
