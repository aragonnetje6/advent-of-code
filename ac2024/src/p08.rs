#![allow(clippy::cast_possible_wrap)]
use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete,
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::pair,
    IResult,
};
use nom_locate::{position, LocatedSpan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Antenna {
    x: isize,
    y: isize,
    freq: char,
}

type Span<'a> = LocatedSpan<&'a str>;

fn parse_antenna(input: Span) -> IResult<Span, Option<Antenna>> {
    alt((
        value(None, complete::char('.')),
        map(
            pair(position::<Span, _>, complete::none_of("\n.")),
            |(pos, freq)| {
                Some(Antenna {
                    x: pos.get_column() as isize - 1,
                    y: pos.location_line() as isize - 1,
                    freq,
                })
            },
        ),
    ))(input)
}

fn parse_file(input: Span) -> IResult<Span, Vec<Antenna>> {
    map(
        separated_list1(complete::newline, many1(parse_antenna)),
        |antennae| antennae.into_iter().flatten().flatten().collect(),
    )(input)
}

fn split_groups(antennae: &[Antenna]) -> BTreeMap<char, Vec<Antenna>> {
    let mut freqs: BTreeMap<char, Vec<Antenna>> = BTreeMap::new();
    for antenna in antennae {
        if let Some(item) = freqs.get_mut(&antenna.freq) {
            item.push(*antenna);
        } else {
            freqs.insert(antenna.freq, vec![*antenna]);
        }
    }
    freqs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Antinode {
    x: isize,
    y: isize,
}

const fn antinodes_pair(a1: Antenna, a2: Antenna) -> [Antinode; 2] {
    [
        Antinode {
            x: a1.x + a1.x - a2.x,
            y: a1.y + a1.y - a2.y,
        },
        Antinode {
            x: a2.x + a2.x - a1.x,
            y: a2.y + a2.y - a1.y,
        },
    ]
}

fn antinodes_group(antennae: &[Antenna]) -> impl Iterator<Item = Antinode> + use<'_> {
    antennae
        .iter()
        .combinations(2)
        .flat_map(|ae| antinodes_pair(*ae[0], *ae[1]))
}

pub fn part1(input: &str) -> String {
    let (_, antennae) = parse_file(Span::new(input)).expect("parsing failure");
    let max_x = input.lines().count() as isize - 1;
    let max_y = input.lines().next().expect("no lines").len() as isize - 1;
    let min_x = 0;
    let min_y = 0;
    let groups: BTreeMap<char, Vec<Antenna>> = split_groups(&antennae);
    groups
        .values()
        .flat_map(|ae| antinodes_group(ae))
        .filter(|an| an.x <= max_x && an.x >= min_x && an.y >= min_y && an.y <= max_y)
        .collect::<BTreeSet<Antinode>>()
        .len()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (_, antennae) = parse_file(Span::new(input)).expect("parsing failure");
    let max_x = input.lines().count() as isize - 1;
    let max_y = input.lines().next().expect("no lines").len() as isize - 1;
    let min_x = 0;
    let min_y = 0;
    let groups: BTreeMap<char, Vec<Antenna>> = split_groups(&antennae);
    groups
        .values()
        .flat_map(|ae| antinodes_line_group(ae, min_x, max_x, min_y, max_y))
        .collect::<BTreeSet<Antinode>>()
        .len()
        .to_string()
}

fn antinodes_line_group(
    antennae: &[Antenna],
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
) -> impl Iterator<Item = Antinode> + use<'_> {
    antennae
        .iter()
        .combinations(2)
        .flat_map(move |ae| antinodes_line_pair(*ae[0], *ae[1], min_x, max_x, min_y, max_y))
}

const fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        (b, a) = (a % b, b);
    }
    a
}

fn antinodes_line_pair(
    a1: Antenna,
    a2: Antenna,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
) -> impl Iterator<Item = Antinode> {
    let x_diff = a1.x - a2.x;
    let y_diff = a1.y - a2.y;
    let x_offset = x_diff / gcd(x_diff, y_diff);
    let y_offset = y_diff / gcd(x_diff, y_diff);
    (0..)
        .map(move |i| Antinode {
            x: i * x_offset + a2.x,
            y: i * y_offset + a2.y,
        })
        .take_while(move |an| an.x <= max_x && an.x >= min_x && an.y >= min_y && an.y <= max_y)
        .chain(
            (0..)
                .map(move |i| Antinode {
                    x: -i * x_offset + a1.x,
                    y: -i * y_offset + a1.y,
                })
                .take_while(move |an| {
                    an.x <= max_x && an.x >= min_x && an.y >= min_y && an.y <= max_y
                }),
        )
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 14.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 34.to_string());
    }
}
