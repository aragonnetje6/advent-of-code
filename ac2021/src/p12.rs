use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::char,
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Node {
    Start,
    End,
    SmallCave(String),
    BigCave(String),
}

fn start(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("start")(input)?;
    Ok((input, Node::Start))
}

fn end(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("end")(input)?;
    Ok((input, Node::End))
}

fn big_cave(input: &str) -> IResult<&str, Node> {
    let (input, name) = take_while1(move |c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(c))(input)?;
    Ok((input, Node::BigCave(name.to_string())))
}

fn small_cave(input: &str) -> IResult<&str, Node> {
    let (input, name) = take_while1(move |c| "abcdefghijklmnopqrstuvwxyz".contains(c))(input)?;
    Ok((input, Node::SmallCave(name.to_string())))
}

fn connection(input: &str) -> IResult<&str, (Node, Node)> {
    let (input, side1) = alt((start, end, small_cave, big_cave))(input)?;
    let (input, _) = char('-')(input)?;
    let (input, side2) = alt((start, end, small_cave, big_cave))(input)?;
    Ok((input, (side1, side2)))
}

fn parse_network(input: &str) -> IResult<&str, Vec<(Node, Node)>> {
    separated_list1(char('\n'), connection)(input)
}

fn network_map(connections: &[(Node, Node)]) -> HashMap<Node, Vec<Node>> {
    let nodes: Vec<Node> = connections
        .iter()
        .flat_map(|(node1, node2)| [node1.clone(), node2.clone()])
        .collect();
    nodes
        .iter()
        .map(|node| {
            (
                node.clone(),
                connections
                    .iter()
                    .filter_map(|(node1, node2)| {
                        if node == node1 {
                            Some(node2.clone())
                        } else {
                            None
                        }
                    })
                    .chain(connections.iter().filter_map(|(node1, node2)| {
                        if node == node2 {
                            Some(node1.clone())
                        } else {
                            None
                        }
                    }))
                    .collect(),
            )
        })
        .collect()
}

fn valid_partial_path_1(path: &[Node]) -> bool {
    let small_caves: Vec<&Node> = path
        .iter()
        .filter(|x| matches!(x, Node::SmallCave(_)))
        .collect();
    let unique_small_caves: HashSet<&Node> = small_caves.iter().copied().collect();
    unique_small_caves.len() == small_caves.len()
        && path.iter().filter(|x| x == &&Node::Start).count() <= 1
        && path.iter().filter(|x| x == &&Node::End).count() <= 1
}

fn valid_partial_path_2(path: &[Node]) -> bool {
    let occurrences = path
        .iter()
        .filter(|x| matches!(x, Node::SmallCave(_)))
        .collect::<HashSet<&Node>>()
        .iter()
        .map(|cave| path.iter().filter(|cave2| cave == cave2).count())
        .collect::<Vec<usize>>();

    occurrences.iter().all(|x| *x <= 2)
        && occurrences.iter().filter(|x| **x == 2).count() <= 1
        && path.iter().filter(|x| x == &&Node::Start).count() <= 1
        && path.iter().filter(|x| x == &&Node::End).count() <= 1
}

fn list_paths<F: Fn(&[Node]) -> bool + Clone>(
    map: &HashMap<Node, Vec<Node>>,
    path_so_far: &[Node],
    validator: F,
) -> Vec<Vec<Node>> {
    let start = path_so_far.last().unwrap().clone();

    if !validator(path_so_far) {
        return vec![];
    }

    map[&start]
        .iter()
        .map(|option| {
            let mut new_path = path_so_far.to_vec();
            new_path.push(option.clone());
            new_path
        })
        .filter(|x| validator(x))
        .flat_map(|new_path| {
            if new_path.last().unwrap() == &Node::End {
                vec![new_path]
            } else {
                list_paths(map, &new_path, validator.clone())
            }
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    let (_, data) = parse_network(input).unwrap();
    let map = network_map(&data);
    let paths = list_paths(&map, &[Node::Start], valid_partial_path_1);
    paths.len()
}

pub fn part2(input: &str) -> usize {
    let (_, data) = parse_network(input).unwrap();
    let map = network_map(&data);
    let paths = list_paths(&map, &[Node::Start], valid_partial_path_2);
    paths.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
    const DATA2: &str = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

    const DATA3: &str = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 10);
        assert_eq!(part1(DATA2), 19);
        assert_eq!(part1(DATA3), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 36);
        assert_eq!(part2(DATA2), 103);
        assert_eq!(part2(DATA3), 3509);
    }
}
