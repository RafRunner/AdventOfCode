use std::ops::RangeInclusive;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

pub fn part_one(almanac_str: &str) -> usize {
    let almanac = Almanac::parse(almanac_str);
    almanac.find_min_location(almanac.seeds.clone().into_iter())
}

pub fn part_two(almanac_str: &str) -> usize {
    let almanac = Almanac::parse(almanac_str);
    almanac.find_min_location(almanac.seeds_as_ranges())
}

#[derive(Debug, Clone)]
struct RangeMap {
    ranges: Vec<(RangeInclusive<usize>, isize)>,
}

impl RangeMap {
    fn parse(ranges_str: Vec<&str>) -> Self {
        let ranges: Vec<(RangeInclusive<usize>, isize)> = ranges_str
            .iter()
            .map(|line| {
                let parts = line
                    .split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

                (
                    (parts[1]..=(parts[1] + parts[2] - 1)),
                    parts[0] as isize - parts[1] as isize,
                )
            })
            .collect();

        Self { ranges }
    }

    fn get(&self, input: usize) -> usize {
        self.ranges
            .iter()
            .find(|(input_range, _)| input_range.contains(&input))
            .map(|(_, offset)| (input as isize + offset) as usize)
            .unwrap_or(input)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<RangeMap>,
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

        let maps: Vec<RangeMap> = sections
            .map(|text| RangeMap::parse(text.lines().skip(1).collect()))
            .collect();

        Self { seeds, maps }
    }

    fn find_min_location(&self, seeds: impl Iterator<Item = usize>) -> usize {
        let pool = ThreadPool::new(16);
        let (tx, rx) = channel();

        let mut min = usize::MAX;

        for seed in seeds {
            let tx = tx.clone();
            let maps = self.maps.clone();

            pool.execute(move || {
                let mut key = seed;

                for map in &maps {
                    key = map.get(key);
                }

                tx.send(key).unwrap();
            });
        }

        drop(tx);

        for msg in rx {
            if msg < min {
                min = msg
            }
        }

        min
    }

    fn seeds_as_ranges(&self) -> impl Iterator<Item = usize> + '_ {
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

        ranges.into_iter().flatten()
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

    #[test]
    fn real() {
        let input = include_str!("../res/day_05.txt");

        assert_eq!(457535844, part_one(input));
        // assert_eq!(46, part_two(input));
    }
}
