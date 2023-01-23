use std::collections::{HashSet, VecDeque};

fn parse_heightmap(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn list_valid_neighbours<T>(map: &Vec<Vec<T>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut out = vec![];
    if x > 0 {
        out.push((x - 1, y));
    }
    if y > 0 {
        out.push((x, y - 1));
    }
    if x < map.len() - 1 {
        out.push((x + 1, y));
    }
    if y < map[0].len() - 1 {
        out.push((x, y + 1));
    }
    out
}

fn list_low_points(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.iter()
                .enumerate()
                .filter(|(y, height)| {
                    list_valid_neighbours(map, x, *y)
                        .iter()
                        .all(|(x2, y2)| map[*x2][*y2] > **height)
                })
                .map(|(y, _)| (x, y))
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    let map = parse_heightmap(input);
    list_low_points(&map)
        .iter()
        .map(|(x, y)| map[*x][*y] + 1)
        .sum::<u32>()
        .to_string()
}

fn get_basin_size(low_point: &(usize, usize), map: &Vec<Vec<u32>>) -> usize {
    let mut checked = HashSet::new();
    let mut tbd = VecDeque::from(vec![*low_point]);
    while !tbd.is_empty() {
        let (x, y) = tbd.pop_front().unwrap();
        if map[x][y] < 9 {
            checked.insert((x, y));
            if x > 0 && !checked.contains(&(x - 1, y)) {
                tbd.push_back((x - 1, y));
            }
            if y > 0 && !checked.contains(&(x, y - 1)) {
                tbd.push_back((x, y - 1));
            }
            if x < map.len() - 1 && !checked.contains(&(x + 1, y)) {
                tbd.push_back((x + 1, y));
            }
            if y < map[0].len() - 1 && !checked.contains(&(x, y + 1)) {
                tbd.push_back((x, y + 1));
            }
        }
    }
    checked.len()
}

pub fn part2(input: &str) -> String {
    let map = parse_heightmap(input);
    let low_points = list_low_points(&map);
    let mut basins: Vec<usize> = low_points
        .iter()
        .map(|point| get_basin_size(point, &map))
        .collect();
    basins.sort_unstable();
    basins.reverse();
    basins.drain(..).take(3).product::<usize>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "15");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), "1134");
    }
}
