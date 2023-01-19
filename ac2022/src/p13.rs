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

fn integer(input: &str) -> IResult<&str, Packet> {
    map(nom::character::complete::u32, Packet::Integer)(input)
}

fn list(input: &str) -> IResult<&str, Packet> {
    delimited(
        char('['),
        map(separated_list0(char(','), value), Packet::List),
        char(']'),
    )(input)
}

fn value(input: &str) -> IResult<&str, Packet> {
    alt((integer, list))(input)
}

fn pair(input: &str) -> IResult<&str, PacketPair> {
    separated_pair(value, newline, value)(input)
}

fn parse_signal(input: &str) -> IResult<&str, Vec<PacketPair>> {
    separated_list1(count(newline, 2), pair)(input)
}

type PacketPair = (Packet, Packet);

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        println!("Comparing {self} vs {other}");
        Some(match self {
            Packet::List(left) => match other {
                Packet::List(right) => left
                    .iter()
                    .zip(right.iter())
                    .map(|(x, y)| x.cmp(y))
                    .find(|ord| *ord != Ordering::Equal)
                    .unwrap_or_else(|| left.len().cmp(&right.len())),
                Packet::Integer(_) => Self::cmp(self, &Packet::List(vec![self.clone()])),
            },
            Packet::Integer(left) => match other {
                Packet::List(_) => Self::cmp(&Packet::List(vec![self.clone()]), other),
                Packet::Integer(right) => left.cmp(right),
            },
        })
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("WTF")
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Integer(x) => write!(f, "{x}"),
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
    let (_, data) = parse_signal(input).unwrap();
    // for pair in &data {
    //     println!("{}", pair.0 <= pair.1);
    //     println!();
    // }
    // println!("{}", data.len());
    data.iter()
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
