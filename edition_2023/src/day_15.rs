pub fn part_one(input: &str) -> usize {
    input.split(',').map(hash).map(|byte| byte as usize).sum()
}

pub fn part_two(input: &str) -> usize {
    let mut deer_hash_map = DeerHashMap::new();

    input
        .split(',')
        .map(Command::parse)
        .for_each(|command| command.apply(&mut deer_hash_map));

    deer_hash_map.calculate_power()
}

#[derive(Debug)]
enum Command {
    Minus { label: String },
    Equals { label: String, focal_length: usize },
}

impl Command {
    fn parse(cmd: &str) -> Self {
        if cmd.contains('=') {
            let mut parts = cmd.trim().split('=');

            return Self::Equals {
                label: parts.next().unwrap().to_owned(),
                focal_length: parts.next().unwrap().parse().unwrap(),
            };
        } else if cmd.ends_with('-') {
            return Self::Minus {
                label: cmd.trim().replace('-', ""),
            };
        }

        panic!("Unknow command {cmd}");
    }

    fn apply(&self, map: &mut DeerHashMap) {
        match self {
            Self::Minus { label } => {
                let lens_box = &mut map.lens_boxes[hash(label) as usize];

                if let Some((index, _)) = lens_box.find_lens(label) {
                    lens_box.lenses.remove(index);
                }
            }
            Self::Equals {
                label,
                focal_length,
            } => {
                let lens = Lens {
                    label: label.to_owned(),
                    focal_length: *focal_length,
                };

                let lens_box = &mut map.lens_boxes[hash(label) as usize];

                if let Some((index, _)) = lens_box.find_lens(label) {
                    let _ = std::mem::replace(&mut lens_box.lenses[index], lens);
                } else {
                    lens_box.lenses.push(lens);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[derive(Debug)]
struct LensBox {
    lenses: Vec<Lens>,
}

impl LensBox {
    fn find_lens(&self, label: &str) -> Option<(usize, &Lens)> {
        self.lenses
            .iter()
            .enumerate()
            .find(|(_, l)| l.label == label)
    }
}

#[derive(Debug)]
struct DeerHashMap {
    lens_boxes: Vec<LensBox>,
}

impl DeerHashMap {
    fn new() -> Self {
        let mut boxes = Vec::with_capacity(256);

        for _ in 0..256 {
            boxes.push(LensBox { lenses: Vec::new() });
        }

        Self { lens_boxes: boxes }
    }

    fn calculate_power(&self) -> usize {
        self.lens_boxes
            .iter()
            .enumerate()
            .fold(0, |acc, (box_number, lens_box)| {
                if lens_box.lenses.is_empty() {
                    return acc;
                }

                let partial = lens_box
                    .lenses
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (i, lens)| acc + (i + 1) * lens.focal_length);

                acc + (box_number + 1) * partial
            })
    }
}

fn hash(string: &str) -> u8 {
    string.as_bytes().iter().fold(0, |acc, &cur| {
        let acc = acc as u64 + cur as u64;
        let acc = acc * 17;
        let acc = acc % 256;
        acc as u8
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(52, hash("HASH"));

        let commands = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(1320, part_one(commands));
        assert_eq!(145, part_two(commands));
    }

    #[test]
    fn real() {
        let commands = include_str!("../res/day_15.txt");
        assert_eq!(501680, part_one(commands));
        assert_eq!(241094, part_two(commands));
    }
}
