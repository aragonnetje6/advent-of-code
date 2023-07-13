use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::newline,
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};
use pathfinding::prelude::dijkstra_all;

type Path = Vec<String>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

impl Valve {
    fn successors(&self, valves: &HashMap<String, Self>) -> Vec<(Self, u32)> {
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
    map(separated_list1(newline, valve), |valves| {
        valves
            .into_iter()
            .map(|valve| (valve.name.clone(), valve))
            .collect()
    })(input)
}

fn relative_valve_costs(valves: &HashMap<String, Valve>) -> HashMap<Valve, HashMap<String, u32>> {
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
                            Some((valve.name.clone(), *cost + 1))
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        })
        .collect()
}

fn find_valve(network: &HashMap<Valve, HashMap<String, u32>>, valve_name: &str) -> Valve {
    network
        .iter()
        .find(|(v, _)| v.name == *valve_name)
        .expect("guaranteed success")
        .0
        .clone()
}

fn evaluate_path(
    path: &[String],
    network: &HashMap<Valve, HashMap<String, u32>>,
    remaining_time: u32,
) -> u32 {
    if path.is_empty() {
        0
    } else if path.len() == 1 {
        find_valve(network, &path[0]).flow_rate * remaining_time
    } else {
        let (first, rest) = path.split_first().expect("guaranteed by checks");
        let valve = find_valve(network, first);
        valve.flow_rate * remaining_time
            + evaluate_path(rest, network, remaining_time - network[&valve][&rest[0]])
    }
}

fn optimal_path(network: &HashMap<Valve, HashMap<String, u32>>) -> (Path, u32) {
    all_paths_starting_at(network, 30, &vec!["AA".to_string()])
        .into_iter()
        .map(|path| (path.clone(), evaluate_path(&path, network, 30)))
        .max_by_key(|(_, score)| *score)
        .expect("no paths found")
}

fn all_paths_starting_at(
    network: &HashMap<Valve, HashMap<String, u32>>,
    remaining_time: u32,
    path: &Path,
) -> Vec<Path> {
    let valve = find_valve(network, path.last().expect("empty path"));
    let mut out = vec![path.clone()];
    out.extend(
        network[&valve]
            .iter()
            .filter(|(valve2_name, dist)| **dist <= remaining_time && !path.contains(*valve2_name))
            .flat_map(|(valve2_name, dist)| {
                let mut new_path = path.clone();
                new_path.push(valve2_name.clone());
                all_paths_starting_at(network, remaining_time - dist, &new_path)
            }),
    );
    out
}

pub fn part1(input: &str) -> String {
    let (_, data) = cave_system(input).unwrap();
    let useful_valve_paths = relative_valve_costs(&data);
    let (_, flow) = optimal_path(&useful_valve_paths);
    flow.to_string()
}

fn optimal_path_pair(network: &HashMap<Valve, HashMap<String, u32>>) -> (Path, Path, u32) {
    let paths = all_paths_starting_at(network, 26, &vec!["AA".to_string()]);
    let scored_paths: Vec<(Path, u32)> = paths
        .into_iter()
        .map(|x| {
            let score = evaluate_path(&x, network, 26);
            (x, score)
        })
        .collect();
    scored_paths
        .iter()
        .flat_map(|(path1, score1)| {
            scored_paths
                .iter()
                .filter(|(path2, _)| no_overlap(&path1[1..], &path2[1..]))
                .map(|(path2, score2)| (path1.clone(), path2.clone(), *score1 + *score2))
        })
        .max_by_key(|(_, _, score)| *score)
        .expect("no paths found")
}

fn no_overlap<T: Eq>(path1: &[T], path2: &[T]) -> bool {
    path1.iter().all(|x| !path2.contains(x))
}

pub fn part2(input: &str) -> String {
    let (_, data) = cave_system(input).unwrap();
    let useful_valve_paths = relative_valve_costs(&data);
    let (_, _, flow) = optimal_path_pair(&useful_valve_paths);
    flow.to_string()
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
    fn test_part2() {
        assert_eq!(part2(DATA1), "1707");
    }
}
