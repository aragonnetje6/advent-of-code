use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use nom::{
    branch::alt,
    character::complete::{char, newline},
    combinator::map,
    multi::{count, separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

fn value(input: &str) -> IResult<&str, Packet> {
    alt((
        map(nom::character::complete::u32, Packet::Number),
        delimited(
            char('['),
            map(separated_list0(char(','), value), Packet::List),
            char(']'),
        ),
    ))(input)
}

fn parse_signal(input: &str) -> IResult<&str, Vec<PacketPair>> {
    separated_list1(count(newline, 2), separated_pair(value, newline, value))(input)
}

type PacketPair = (Packet, Packet);

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(left), Packet::List(right)) => left.cmp(right),
            (Packet::List(left), Packet::Number(_)) => left.cmp(&vec![other.clone()]),
            (Packet::Number(_), Packet::List(right)) => vec![self.clone()].cmp(right),
            (Packet::Number(left), Packet::Number(right)) => left.cmp(right),
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Number(x) => write!(f, "{x}"),
            Packet::List(arr) => write!(
                f,
                "[{}]",
                arr.iter()
                    .map(Packet::to_string)
                    .reduce(|acc, elem| format!("{acc},{elem}"))
                    .unwrap_or_default()
            ),
        }
    }
}

pub fn part1(input: &str) -> usize {
    parse_signal(input)
        .unwrap()
        .1
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| if left <= right { Some(i + 1) } else { None })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 13);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(DATA1), 29);
    }
}
