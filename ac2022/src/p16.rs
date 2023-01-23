use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

fn valve(input: &str) -> IResult<&str, Valve> {
    let (input, name) = preceded(tag("Valve "), take(2usize))(input)?;
    let (input, flow_rate) =
        preceded(tag(" has flow rate="), nom::character::complete::u32)(input)?;
    let (input, tunnels) = preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), map(take(2usize), ToString::to_string)),
    )(input)?;
    Ok((
        input,
        Valve {
            name: name.to_string(),
            flow_rate,
            tunnels,
        },
    ))
}

fn cave_system(input: &str) -> IResult<&str, HashMap<String, Valve>> {
    map(separated_list1(newline, valve), |mut valves| {
        valves
            .drain(..)
            .map(|valve| (valve.name.clone(), valve))
            .collect()
    })(input)
}

pub fn part1(input: &str) -> f64 {
    let (_, data) = cave_system(input).unwrap();
    println!("{data:?}");
    data.values().map(|x| x.tunnels.len() as f64).sum::<f64>() / (data.len() as f64)
}

pub fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1(DATA1), 26);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(DATA1), 56_000_011);
    }
}
