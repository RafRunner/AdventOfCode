use std::ops::Range;

pub fn part_one() -> usize {
    // Time:        56     71     79     99
    // Distance:   334   1135   1350   2430
    let races = vec![
        Race::new(56, 334),
        Race::new(71, 1135),
        Race::new(79, 1350),
        Race::new(99, 2430),
    ];

    races
        .into_iter()
        .map(|r| r.winning_range().map(|range| range.len()).unwrap_or(0))
        .product::<usize>()
}

pub fn part_two() -> usize {
    let race = Race::new(56717999, 334113513502430);
    race.winning_range().unwrap().len()
}

struct Race {
    time_limit: usize,
    distance_to_beat: usize,
}

impl Race {
    fn new(time_limit: usize, distance_to_beat: usize) -> Self {
        Self {
            time_limit,
            distance_to_beat,
        }
    }

    fn winning_range(&self) -> Option<Range<usize>> {
        // x(t) = - t^2 + time_limit * t - distance_to_beat
        let a = -1_f64;
        let b = self.time_limit as f64;
        // We have to beat the time, so we add 1
        let c = -((self.distance_to_beat as f64) + 1.0);

        let delta = b.powf(2.0) - 4.0 * a * c;

        if delta <= 0.0 {
            None
        } else {
            let delta = (delta).sqrt();

            let lower_limit = (-b + delta) / (2.0 * a);
            let upper_limit = (-b - delta) / (2.0 * a);

            if upper_limit < 0.0 {
                None
            } else {
                // We can't be lower than lower, so ceil it
                // We can't be higher than upper, so floor it
                // + 1 cause excluding range
                Some((lower_limit.ceil() as usize)..(upper_limit.floor() as usize) + 1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rage_calculation() {
        let race = Race::new(7, 9);
        assert_eq!(Some(2..6), race.winning_range());

        let race = Race::new(15, 40);
        assert_eq!(Some(4..12), race.winning_range());

        let race = Race::new(30, 200);
        assert_eq!(Some(11..20), race.winning_range());

        let race = Race::new(71530, 940200);
        assert_eq!(Some(14..71517), race.winning_range());
    }

    #[test]
    fn part_one_test() {
        assert_eq!(211904, part_one());
    }

    #[test]
    fn part_two_test() {
        assert_eq!(43364472, part_two());
    }
}
