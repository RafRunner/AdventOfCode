pub fn part_one(input: &str) -> usize {
    input.split(',').map(hash).map(|byte| byte as usize).sum()
}

fn hash(string: &str) -> u8 {
    string.as_bytes().into_iter().fold(0, |acc, &cur| {
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

        assert_eq!(1320, part_one("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"));
    }

    #[test]
    fn real() {
        assert_eq!(1, part_one(include_str!("../res/day_15.txt")));
    }
}