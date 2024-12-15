#![allow(clippy::copy_iterator)]
use std::collections::HashMap;

use nom::{character::complete, multi::separated_list1, IResult};

fn parse_stones(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(complete::space1, complete::u64)(input)
}

fn update_stones(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut result = HashMap::new();
    for (k, v) in stones {
        if k == 0 {
            *result.entry(1).or_default() += v;
            continue;
        }
        let digits = k.ilog10() + 1;
        if digits % 2 == 0 {
            *result.entry(k / 10u64.pow(digits / 2)).or_default() += v;
            *result.entry(k % 10u64.pow(digits / 2)).or_default() += v;
        } else {
            *result.entry(k * 2024).or_default() += v;
        }
    }
    result
}

fn to_amounts(stones: &[u64]) -> HashMap<u64, u64> {
    let mut amounts = HashMap::new();
    for stone in stones {
        *amounts.entry(*stone).or_default() += 1;
    }
    amounts
}

pub fn part1(input: &str) -> String {
    let (_, stones) = parse_stones(input).expect("parsing error");
    let mut stone_counts = to_amounts(&stones);
    for _ in 0..25 {
        stone_counts = update_stones(stone_counts);
    }
    stone_counts.values().sum::<u64>().to_string()
}

pub fn part2(input: &str) -> String {
    let (_, stones) = parse_stones(input).expect("parsing error");
    let mut stone_counts = to_amounts(&stones);
    for _ in 0..75 {
        stone_counts = update_stones(stone_counts);
    }
    stone_counts.values().sum::<u64>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 55312.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 65601038650482u64.to_string());
    }
}
