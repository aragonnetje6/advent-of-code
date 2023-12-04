use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space0},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

fn number(input: &str) -> IResult<&str, u32> {
    preceded(space0, map_res(digit1, str::parse::<u32>))(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<u32>> {
    many1(number)(input)
}

#[derive(Debug)]
struct Card {
    id: u32,
    nums: Vec<u32>,
    winning: Vec<u32>,
    gives: std::ops::RangeInclusive<u32>,
}

fn id(input: &str) -> IResult<&str, u32> {
    preceded(tag("Card "), number)(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    map(
        separated_pair(id, tag(": "), separated_pair(numbers, tag(" | "), numbers)),
        |(id, (nums, winning))| Card::new(id, nums, winning),
    )(input)
}

fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(newline, card)(input)
}

impl Card {
    fn score(&self) -> u32 {
        match self
            .nums
            .iter()
            .filter(|x| self.winning.contains(*x))
            .count()
        {
            0 => 0,
            x => (2u32).pow(u32::try_from(x).expect("out of bounds") - 1),
        }
    }

    fn new(id: u32, nums: Vec<u32>, winning: Vec<u32>) -> Self {
        let gives = id + 1
            ..=u32::try_from(nums.iter().filter(|x| winning.contains(*x)).count())
                .expect("out of bounds")
                + id;
        Self {
            id,
            nums,
            winning,
            gives,
        }
    }
}

pub fn part1(input: &str) -> String {
    let (_, cards) = cards(input).expect("Parsing failed");
    cards.iter().map(Card::score).sum::<u32>().to_string()
}

pub fn part2(input: &str) -> String {
    let (_, cards) = cards(input).expect("Parsing failed");
    let mut card_counts: HashMap<u32, usize> = cards.iter().map(|x| (x.id, 1)).collect();
    for card in cards {
        let count = card_counts[&card.id];
        for other_card in card.gives.clone() {
            *card_counts.get_mut(&other_card).expect("ID not found") += count;
        }
    }
    card_counts.values().sum::<usize>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 13.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 30.to_string());
    }
}
