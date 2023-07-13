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

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(nom::character::complete::u32, Packet::Number),
        delimited(
            char('['),
            map(separated_list0(char(','), packet), Packet::List),
            char(']'),
        ),
    ))(input)
}

fn parse_signal(input: &str) -> IResult<&str, Vec<PacketPair>> {
    separated_list1(count(newline, 2), separated_pair(packet, newline, packet))(input)
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
            (Self::List(left), Self::List(right)) => left.cmp(right),
            (Self::List(left), Self::Number(_)) => left.cmp(&vec![other.clone()]),
            (Self::Number(_), Self::List(right)) => vec![self.clone()].cmp(right),
            (Self::Number(left), Self::Number(right)) => left.cmp(right),
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{x}"),
            Self::List(arr) => write!(
                f,
                "[{}]",
                arr.iter()
                    .map(Self::to_string)
                    .reduce(|acc, elem| format!("{acc},{elem}"))
                    .unwrap_or_default()
            ),
        }
    }
}

pub fn part1(input: &str) -> String {
    parse_signal(input)
        .unwrap()
        .1
        .iter()
        .enumerate()
        .filter_map(|(i, (left, right))| if left <= right { Some(i + 1) } else { None })
        .sum::<usize>()
        .to_string()
}

fn transform_data(data: Vec<PacketPair>) -> Vec<Packet> {
    data.into_iter().flat_map(|(p1, p2)| [p1, p2]).collect()
}

pub fn part2(input: &str) -> String {
    let mut signal = transform_data(parse_signal(input).unwrap().1);
    let divs = vec![packet("[[2]]").unwrap().1, packet("[[6]]").unwrap().1];
    signal.append(&mut divs.clone());
    signal.sort_unstable();
    divs.iter()
        .map(|div| {
            signal
                .iter()
                .enumerate()
                .find_map(|(i, x)| if x == div { Some(i + 1) } else { None })
                .unwrap()
        })
        .product::<usize>()
        .to_string()
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
        assert_eq!(part1(DATA1), "13");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), "140");
    }
}
