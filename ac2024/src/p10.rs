use std::collections::HashSet;

use nom::{
    character::complete,
    combinator::{all_consuming, map_opt},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

fn parse_tile(input: &str) -> IResult<&str, u8> {
    map_opt(complete::one_of("0123456789"), |x| {
        x.to_digit(10).map(u8::try_from).and_then(Result::ok)
    })(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    all_consuming(terminated(
        separated_list1(complete::newline, many1(parse_tile)),
        complete::newline,
    ))(input)
}

fn evaluate_trailheads(map: &[Vec<u8>]) -> Vec<usize> {
    map.iter()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, height)| **height == 0)
                .map(move |(x, _)| {
                    list_trailhead_endings(map, y, x)
                        .into_iter()
                        .collect::<HashSet<(usize, usize)>>()
                        .len()
                })
        })
        .collect()
}

fn list_trailhead_endings(map: &[Vec<u8>], y: usize, x: usize) -> Vec<(usize, usize)> {
    let mut total = vec![];
    if let Some(tile2) = map.get(y.wrapping_sub(1)).and_then(|line| line.get(x)) {
        if *tile2 == map[y][x] + 1 {
            if *tile2 == 9 {
                total.push((y - 1, x));
            } else {
                total.extend(list_trailhead_endings(map, y - 1, x));
            }
        }
    }
    if let Some(tile2) = map.get(y + 1).and_then(|line| line.get(x)) {
        if *tile2 == map[y][x] + 1 {
            if *tile2 == 9 {
                total.push((y + 1, x));
            } else {
                total.extend(list_trailhead_endings(map, y + 1, x));
            }
        }
    }
    if let Some(tile2) = map.get(y).and_then(|line| line.get(x.wrapping_sub(1))) {
        if *tile2 == map[y][x] + 1 {
            if *tile2 == 9 {
                total.push((y, x - 1));
            } else {
                total.extend(list_trailhead_endings(map, y, x - 1));
            }
        }
    }
    if let Some(tile2) = map.get(y).and_then(|line| line.get(x + 1)) {
        if *tile2 == map[y][x] + 1 {
            if *tile2 == 9 {
                total.push((y, x + 1));
            } else {
                total.extend(list_trailhead_endings(map, y, x + 1));
            }
        }
    }
    total
}

pub fn part1(input: &str) -> String {
    let (_, map) = parse_input(input).expect("parsing error");
    evaluate_trailheads(&map)
        .into_iter()
        .sum::<usize>()
        .to_string()
}

fn trailhead_ratings(map: &[Vec<u8>]) -> Vec<usize> {
    map.iter()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, height)| **height == 0)
                .map(move |(x, _)| count_trailheads(map, y, x))
        })
        .collect()
}

fn count_trailheads(map: &[Vec<u8>], y: usize, x: usize) -> usize {
    let mut total = 0;
    if let Some(tile2) = map.get(y.wrapping_sub(1)).and_then(|line| line.get(x)) {
        if *tile2 == map[y][x] + 1 {
            if *tile2 == 9 {
                total += 1;
            } else {
                total += count_trailheads(map, y - 1, x);
            }
        }
    }
    if let Some(tile2) = map.get(y + 1).and_then(|line| line.get(x)) {
        if *tile2 == map[y][x] + 1 {
            if *tile2 == 9 {
                total += 1;
            } else {
                total += count_trailheads(map, y + 1, x);
            }
        }
    }
    if let Some(tile2) = map.get(y).and_then(|line| line.get(x.wrapping_sub(1))) {
        if *tile2 == map[y][x] + 1 {
            if *tile2 == 9 {
                total += 1;
            } else {
                total += count_trailheads(map, y, x - 1);
            }
        }
    }
    if let Some(tile2) = map.get(y).and_then(|line| line.get(x + 1)) {
        if *tile2 == map[y][x] + 1 {
            if *tile2 == 9 {
                total += 1;
            } else {
                total += count_trailheads(map, y, x + 1);
            }
        }
    }
    total
}

pub fn part2(input: &str) -> String {
    let (_, map) = parse_input(input).expect("parsing error");
    trailhead_ratings(&map)
        .into_iter()
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 36.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 81.to_string());
    }
}
