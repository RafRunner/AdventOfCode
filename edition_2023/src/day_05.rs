use std::ops::Range;

pub fn part_one(almanac_str: &str) -> usize {
    let almanac = Almanac::parse(almanac_str);
    almanac.find_min_location(almanac.seeds.clone().into_iter())
}

pub fn part_two(almanac_str: &str) -> usize {
    let almanac = Almanac::parse(almanac_str);
    almanac.find_min_location_back(almanac.seeds_as_ranges())
}

#[derive(Debug, Clone)]
struct RangeMap {
    input_range: Range<usize>,
    deviation: isize,
}

impl RangeMap {
    fn try_map(&self, n: usize) -> Option<usize> {
        if self.input_range.contains(&n) {
            Some(self.add_deviation(n))
        } else {
            None
        }
    }

    fn try_reverse_map(&self, n: usize) -> Option<usize> {
        if n >= self.add_deviation(self.input_range.start)
            && n < self.add_deviation(self.input_range.end)
        {
            Some((n as isize - self.deviation) as usize)
        } else {
            None
        }
    }

    fn add_deviation(&self, n: usize) -> usize {
        (n as isize + self.deviation) as usize
    }
}

#[derive(Debug, Clone)]
struct Mapping {
    ranges: Vec<RangeMap>,
}

impl Mapping {
    fn parse(ranges_str: Vec<&str>) -> Self {
        let mut ranges: Vec<RangeMap> = ranges_str
            .iter()
            .map(|line| {
                let parts = line
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

                RangeMap {
                    input_range: (parts[1]..(parts[1] + parts[2])),
                    deviation: parts[0] as isize - parts[1] as isize,
                }
            })
            .collect();

        ranges.sort_by(|a, b| a.input_range.start.cmp(&b.input_range.start));

        Self { ranges }
    }

    fn get(&self, input: usize) -> usize {
        self.ranges
            .iter()
            .find_map(|range| range.try_map(input))
            .unwrap_or(input)
    }

    fn reverse_get(&self, input: usize) -> usize {
        self.ranges
            .iter()
            .find_map(|range| range.try_reverse_map(input))
            .unwrap_or(input)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Mapping>,
}

impl Almanac {
    fn parse(almanac_str: &str) -> Self {
        let almanac_str = almanac_str.replace('\r', "");
        let mut sections = almanac_str.split("\n\n");

        let seeds: Vec<usize> = sections
            .next()
            .unwrap()
            .replace("seeds: ", "")
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let maps: Vec<Mapping> = sections
            .map(|text| Mapping::parse(text.lines().skip(1).collect()))
            .collect();

        Self { seeds, maps }
    }

    fn find_min_location(&self, seeds: impl Iterator<Item = usize>) -> usize {
        let mut min = usize::MAX;

        for seed in seeds {
            let mut key = seed;

            for map in &self.maps {
                key = map.get(key);
            }

            if key < min {
                min = key
            }
        }

        min
    }

    fn find_min_location_back(&self, seeds: Vec<Range<usize>>) -> usize {
        let mut n = 0;
        loop {
            let mut key = n;

            for map in self.maps.iter().rev() {
                key = map.reverse_get(key)
            }

            if seeds.iter().any(|r| r.contains(&key)) {
                return n;
            }
            n += 1;
        }
    }

    fn seeds_as_ranges(&self) -> Vec<Range<usize>> {
        let mut ranges = self
            .seeds
            .chunks_exact(2)
            .map(|start_size| {
                let start = start_size[0];
                let size = start_size[1];

                start..(start + size)
            })
            .collect::<Vec<_>>();

        ranges.sort_by(|a, b| a.start.cmp(&b.start));

        ranges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(35, part_one(input));
        assert_eq!(46, part_two(input));
    }

    #[ignore = "Takes over a minute"]
    #[test]
    fn real() {
        let input = include_str!("../res/day_05.txt");

        assert_eq!(457535844, part_one(input));
        assert_eq!(41222968, part_two(input));
    }
}
