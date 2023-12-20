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
}
