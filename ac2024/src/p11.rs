#![allow(clippy::copy_iterator)]
use nom::{character::complete, multi::separated_list1, IResult};

fn parse_stones(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(complete::space1, complete::u64)(input)
}

#[derive(Debug, Clone, Copy)]
enum MaybeSplit {
    None,
    One(u64),
    Two(u64, u64),
}

impl Iterator for MaybeSplit {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match *self {
            Self::One(x) => {
                *self = Self::None;
                Some(x)
            }
            Self::Two(x, y) => {
                *self = Self::One(y);
                Some(x)
            }
            Self::None => None,
        }
    }
}

fn update_stones(stones: Vec<u64>) -> Vec<u64> {
    stones
        .into_iter()
        .flat_map(|x| {
            if x == 0 {
                return MaybeSplit::One(1);
            }
            let digits = x.ilog10() + 1;
            if digits % 2 == 0 {
                MaybeSplit::Two(x / 10u64.pow(digits / 2), x % 10u64.pow(digits / 2))
            } else {
                MaybeSplit::One(x * 2024)
            }
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    let (_, mut stones) = parse_stones(input).expect("parsing error");
    for _ in 0..25 {
        stones = update_stones(stones);
    }
    stones.len().to_string()
}

pub fn part2(input: &str) -> String {
    let (_, mut stones) = parse_stones(input).expect("parsing error");
    for _ in 0..75 {
        stones = update_stones(stones);
    }
    stones.len().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 55312.to_string());
    }
}
