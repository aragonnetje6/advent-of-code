use std::collections::HashMap;

use nom::{
    branch::alt,
    nom::bytes::complete::{tag, take},
    nom::character::complete::newline,
    nom::combinator::map,
    nom::multi::separated_list1,
    nom::sequence::preceded,
    nom::IResult,
};
use pathfinding::prelude::dijkstra_all;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl Valve {
    fn successors(&self, valves: &HashMap<String, Valve>) -> Vec<(Valve, u32)> {
        self.tunnels
            .iter()
            .map(|name| (valves[name].clone(), 1))
            .collect()
    }
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

fn relative_valve_costs(valves: &HashMap<String, Valve>) -> HashMap<Valve, HashMap<Valve, u32>> {
    valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate > 0 || valve.name == "AA")
        .map(|(_, valve)| {
            (
                valve.clone(),
                dijkstra_all(valve, |v| v.successors(valves))
                    .iter()
                    .filter_map(|(valve, (_, cost))| {
                        if valve.flow_rate > 0 || valve.name == "AA" {
                            Some((valve.clone(), *cost + 1))
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        })
        .collect()
}

fn optimal_valve_order(valves: &HashMap<Valve, HashMap<Valve, u32>>) -> (Vec<Valve>, u32) {
    let path = vec![valves.keys().find(|v| v.name == "AA").unwrap().clone()];
    best_move_given(valves, &path, 30, 0)
}

fn best_move_given(
    valves: &HashMap<Valve, HashMap<Valve, u32>>,
    path: &[Valve],
    time_remaining: u32,
    flow_so_far: u32,
) -> (Vec<Valve>, u32) {
    let current_valve = path.last().unwrap();
    let current_rate: u32 = path.iter().map(|x| x.flow_rate).sum();
    dbg!(current_valve);
    valves[current_valve]
        .iter()
        .inspect(|x| {
            dbg!(&x.0.name);
        })
        .filter(|(destination, &cost)| !path.contains(destination) && time_remaining >= cost)
        .inspect(|x| {
            dbg!(&x.0.name);
        })
        .map(|(destination, cost)| {
            let mut new_path = path.to_vec();
            new_path.push(destination.clone());
            best_move_given(
                valves,
                &new_path,
                time_remaining - cost,
                flow_so_far + cost * current_rate,
            )
        })
        .max_by_key(|(_, flow)| *flow)
        .unwrap_or((path.to_vec(), flow_so_far))
}

pub fn part1(input: &str) -> String {
    let (_, data) = cave_system(input).unwrap();
    let useful_valve_paths = relative_valve_costs(&data);
    let (path, flow) = optimal_valve_order(&useful_valve_paths);
    dbg!(path.iter().map(|x| &x.name).collect::<Vec<_>>());
    dbg!(flow);
    flow.to_string()
}

pub fn part2(input: &str) -> String {
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
    fn test_part1() {
        assert_eq!(part1(DATA1), "1651");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(DATA1), "56000011");
    }
}
