use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space0},
    combinator::{complete, map, map_res, value},
    multi::separated_list1,
    IResult,
};

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug)]
enum Colour {
    Red,
    Green,
    Blue,
}

fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    let lines = input.lines().count();
    let (input, games) = complete(separated_list1(line_ending, game))(input)?;
    assert!(games
        .iter()
        .enumerate()
        .all(|(i, (id, _))| i + 1 == *id as usize));
    assert_eq!(games.len(), lines);
    Ok((input, games.into_iter().map(|(_, game)| game).collect()))
}

type Game = Vec<BTreeMap<Colour, u32>>;

fn game(input: &str) -> IResult<&str, (u32, Game)> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, |x: &str| x.parse::<u32>())(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, map) = map(separated_list1(tag("; "), hand), |x| {
        x.into_iter().collect()
    })(input)?;
    Ok((input, (id, map)))
}

type Hand = BTreeMap<Colour, u32>;

fn hand(input: &str) -> IResult<&str, Hand> {
    map(separated_list1(tag(", "), cube), |x| {
        x.into_iter().collect()
    })(input)
}

fn cube(input: &str) -> IResult<&str, (Colour, u32)> {
    let (input, amount) = map_res(digit1, |x: &str| x.parse::<u32>())(input)?;
    let (input, _) = space0(input)?;
    let (input, colour) = alt((
        value(Colour::Blue, tag("blue")),
        value(Colour::Red, tag("red")),
        value(Colour::Green, tag("green")),
    ))(input)?;
    Ok((input, (colour, amount)))
}

pub fn part1(input: &str) -> String {
    let (_, result) = parse(input).expect("parsing failed");
    result
        .iter()
        .enumerate()
        .filter(|(_, game)| {
            game.iter().all(|hand| {
                hand.get(&Colour::Red).unwrap_or(&0) <= &12
                    && hand.get(&Colour::Green).unwrap_or(&0) <= &13
                    && hand.get(&Colour::Blue).unwrap_or(&0) <= &14
            })
        })
        .map(|(i, _)| i + 1)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (_, result) = parse(input).expect("parsing failed");
    result
        .iter()
        .map(|game| {
            game.iter()
                .cloned()
                .reduce(|acc, item| {
                    [Colour::Red, Colour::Green, Colour::Blue]
                        .iter()
                        .map(|k| {
                            (
                                *k,
                                *std::cmp::max(acc.get(k).unwrap_or(&0), item.get(k).unwrap_or(&0)),
                            )
                        })
                        .collect()
                })
                .expect("empty hand")
                .values()
                .copied()
                .product::<u32>()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 8.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 2286.to_string());
    }
}
