#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn new_usize(x: usize, y: usize) -> Self {
        Self::new(x as isize, y as isize)
    }

    pub fn shoelace_area(points: &[Self]) -> f64 {
        let sum = points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .map(|(p1, p2)| p1.x * p2.y - p2.x * p1.y)
            .sum::<isize>()
            .abs();

        sum as f64 / 2.0
    }

    pub fn point_pairs(points: &[Self]) -> Vec<(Self, Self)> {
        let mut pairs = Vec::new();

        for i in 0..points.len() {
            for j in i + 1..points.len() {
                pairs.push((points[i].clone(), points[j].clone()));
            }
        }

        pairs
    }
}

pub fn transpose<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed = Vec::with_capacity(cols);
    for _ in 0..cols {
        transposed.push(Vec::with_capacity(rows));
    }

    for row in matrix.into_iter() {
        for (j, item) in row.into_iter().enumerate() {
            transposed[j].push(item);
        }
    }

    transposed
}

pub fn turn_anticlock<T>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut turned: Vec<Vec<T>> = Vec::with_capacity(cols);
    for _ in 0..cols {
        turned.push(Vec::with_capacity(rows));
    }

    for row in matrix.into_iter().rev() {
        for (j, item) in row.into_iter().enumerate() {
            turned[j].push(item);
        }
    }

    turned
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shoelace() {
        assert_eq!(
            16.5,
            Point::shoelace_area(&vec![
                Point::new(1, 6),
                Point::new(3, 1),
                Point::new(7, 2),
                Point::new(4, 4),
                Point::new(8, 5)
            ])
        );
    }

    #[test]
    fn test_point_pairs() {
        let points = vec![
            Point { x: 1, y: 2 },
            Point { x: 3, y: 4 },
            Point { x: 5, y: 6 },
        ];

        let expected_pairs = vec![
            (Point { x: 1, y: 2 }, Point { x: 3, y: 4 }),
            (Point { x: 1, y: 2 }, Point { x: 5, y: 6 }),
            (Point { x: 3, y: 4 }, Point { x: 5, y: 6 }),
        ];

        let actual_pairs = Point::point_pairs(&points);

        assert_eq!(actual_pairs, expected_pairs);
    }

    #[test]
    fn test_transposed() {
        let matrix = vec![
            vec![1, 2, 3, 10],
            vec![4, 5, 6, 11],
            vec![7, 8, 9, 12],
        ];
        let clone = matrix.clone();

        let transposed = transpose(matrix);

        let expected = vec![
            vec![1, 4, 7],
            vec![2, 5, 8],
            vec![3, 6, 9],
            vec![10, 11, 12],
        ];

        assert_eq!(expected, transposed);
        assert_eq!(clone, transpose(transposed));
    }

    #[test]
    fn test_turn_anticlock() {
        let matrix = vec![vec![1, 2, 3, 10], vec![4, 5, 6, 11], vec![7, 8, 9, 12]];
        let clone = matrix.clone();

        let turned = turn_anticlock(matrix);

        let expected = vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
            vec![12, 11, 10],
        ];

        assert_eq!(expected, turned);
        assert_eq!(
            clone,
            turn_anticlock(turn_anticlock(turn_anticlock(turned)))
        );
    }
}
