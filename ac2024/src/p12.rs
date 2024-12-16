use std::collections::HashSet;

use itertools::Itertools;

fn neighbours<T>(map: &[Vec<T>], x: usize, y: usize) -> [Option<(usize, usize)>; 4] {
    let mut out = [None; 4];
    if x >= 1 {
        out[0] = Some((x - 1, y));
    }
    if y >= 1 {
        out[1] = Some((x, y - 1));
    }
    if x < map[0].len() - 1 {
        out[2] = Some((x + 1, y));
    }
    if y < map.len() - 1 {
        out[3] = Some((x, y + 1));
    }
    out
}

fn get_group(tiles: &[Vec<char>], x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut out = HashSet::from([(x, y)]);
    let mut stack = vec![(x, y)];

    while let Some((x, y)) = stack.pop() {
        for (nx, ny) in neighbours(tiles, x, y).into_iter().flatten() {
            if !out.contains(&(nx, ny)) && tiles[y][x] == tiles[ny][nx] {
                stack.push((nx, ny));
                out.insert((nx, ny));
            }
        }
    }
    out
}

fn group_tiles(tiles: &[Vec<char>]) -> Vec<HashSet<(usize, usize)>> {
    let mut total: HashSet<(usize, usize)> = HashSet::new();
    let mut out = Vec::new();
    for (y, line) in tiles.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            if !total.contains(&(x, y)) {
                let group = get_group(tiles, x, y);
                total.extend(&group);
                out.push(group);
            }
        }
    }
    out
}

fn perimeter(
    map: &[Vec<char>],
    group: &HashSet<(usize, usize)>,
) -> Vec<((usize, usize), Direction)> {
    group
        .iter()
        .flat_map(|(x, y)| {
            neighbours(map, *x, *y)
                .into_iter()
                .zip([
                    Direction::Left,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                ])
                .filter(|(neighbour, _)| {
                    neighbour.is_none_or(|(x2, y2)| map[y2][x2] != map[*y][*x])
                })
                .map(|(_, dir)| ((*x, *y), dir))
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part1(input: &str) -> String {
    let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    group_tiles(&tiles)
        .into_iter()
        .map(|group| perimeter(&tiles, &group).len() * group.len())
        .sum::<usize>()
        .to_string()
}

fn same_line(
    ((x1, y1), dir1): ((usize, usize), Direction),
    ((x2, y2), dir2): ((usize, usize), Direction),
) -> bool {
    dir1 == dir2
        && match dir1 {
            Direction::Up | Direction::Down => y1 == y2 && x1.abs_diff(x2) == 1,
            Direction::Left | Direction::Right => x1 == x2 && y1.abs_diff(y2) == 1,
        }
}

fn reduce_fences(
    mut perimeter: Vec<((usize, usize), Direction)>,
) -> Vec<Vec<((usize, usize), Direction)>> {
    let mut out = Vec::new();
    while let Some(fence) = perimeter.pop() {
        out.push(vec![fence]);
        let current = out.last_mut().expect("wtf");
        while let Some((i, &fence2)) = perimeter
            .iter()
            .find_position(|x| current.iter().any(|y| same_line(*y, **x)))
        {
            current.push(fence2);
            perimeter.swap_remove(i);
        }
    }
    out
}

pub fn part2(input: &str) -> String {
    let tiles: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    group_tiles(&tiles)
        .into_iter()
        .map(|group| reduce_fences(perimeter(&tiles, &group)).len() * group.len())
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"AAAA
BBCD
BBCC
EEEC
";

    const DATA2: &str = r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    const DATA3: &str = r"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    const DATA4: &str = r"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    const DATA5: &str = r"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(DATA1), 140.to_string());
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(part1(DATA2), 772.to_string());
    }
    #[test]
    fn test_part1_3() {
        assert_eq!(part1(DATA3), 1930.to_string());
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(DATA1), 80.to_string());
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(part2(DATA4), 236.to_string());
    }
    #[test]
    fn test_part2_3() {
        assert_eq!(part2(DATA5), 368.to_string());
    }
    #[test]
    fn test_part2_4() {
        assert_eq!(part2(DATA3), 1206.to_string());
    }
}
