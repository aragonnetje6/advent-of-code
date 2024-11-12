use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, newline},
    combinator::{all_consuming, map, value},
    multi::{count, many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
}

fn direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Right, char('R')),
        value(Direction::Left, char('L')),
    ))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn node(input: &str) -> IResult<&str, Node> {
    map(
        separated_pair(
            alphanumeric1,
            tag(" = "),
            delimited(
                char('('),
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                char(')'),
            ),
        ),
        |(name, (left, right))| Node { name, left, right },
    )(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<Direction>, HashMap<&str, Node>)> {
    all_consuming(terminated(
        separated_pair(
            many1(direction),
            count(newline, 2),
            map(separated_list1(newline, node), |nodes| {
                nodes.into_iter().map(|n| (n.name, n)).collect()
            }),
        ),
        newline,
    ))(input)
}

pub fn part1(input: &str) -> String {
    let (_, (directions, nodes)) = parse(input).expect("parsing failure");
    let mut node = nodes.get("AAA").expect("no start");
    let end = nodes.get("ZZZ").expect("no ending");
    for (i, dir) in directions.iter().cycle().enumerate() {
        node = nodes
            .get(match dir {
                Direction::Right => node.right,
                Direction::Left => node.left,
            })
            .expect("Node not found");
        if node == end {
            return (i + 1).to_string();
        }
    }
    unreachable!()
}

fn path_len(start: &Node, map: &HashMap<&str, Node>, directions: &[Direction]) -> usize {
    let mut node = start;
    for (i, dir) in directions.iter().cycle().enumerate() {
        node = map
            .get(match dir {
                Direction::Right => node.right,
                Direction::Left => node.left,
            })
            .expect("Node not found");
        if node.name.ends_with('Z') {
            return i + 1;
        }
    }
    unreachable!()
}

struct Primes {
    results: Vec<usize>,
}

impl Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.results.last().copied();
        for i in self.results.last()? + 1.. {
            if self
                .results
                .iter()
                .take_while(|p| **p * **p < i)
                .all(|p| i % p != 0)
            {
                self.results.push(i);
                break;
            }
        }
        next
    }
}

impl Primes {
    fn new() -> Self {
        Self { results: vec![2] }
    }
}

fn lcm(mut input: Vec<usize>) -> usize {
    let mut factors = vec![];
    let primes = Primes::new();
    for prime in primes {
        while input.iter().any(|x| x % prime == 0) {
            input = input
                .into_iter()
                .map(|x| if x % prime == 0 { x / prime } else { x })
                .collect();
            factors.push(prime);
        }
        if input.iter().all(|x| x == &1) {
            break;
        }
    }
    factors.iter().product()
}

pub fn part2(input: &str) -> String {
    let (_, (directions, nodes)) = parse(input).expect("parsing failure");
    let lengths: Vec<usize> = nodes
        .values()
        .filter(|n| n.name.ends_with('A'))
        .map(|node| path_len(node, &nodes, &directions))
        .collect();
    lcm(lengths).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";
    const DATA2: &str = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
    const DATA3: &str = r"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(DATA1), 2.to_string());
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(DATA2), 6.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA3), 6.to_string());
    }
}
