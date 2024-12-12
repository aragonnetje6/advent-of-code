use std::{
    cmp::{max, min},
    collections::HashSet,
};

use nom::{
    branch::alt,
    character::complete,
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};
use nom_locate::{position, LocatedSpan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Wall {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    const fn next(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall(Wall),
    Empty,
    Guard(Guard),
}

type Span<'a> = LocatedSpan<&'a str>;

fn parse_tile(input: Span) -> IResult<Span, Tile> {
    alt((
        value(Tile::Empty, complete::char('.')),
        map(preceded(complete::char('^'), position), |pos: Span| {
            Tile::Guard(Guard {
                x: pos.get_column(),
                y: pos.location_line() as usize,
                direction: Direction::Down,
            })
        }),
        map(preceded(complete::char('#'), position), |pos: Span| {
            Tile::Wall(Wall {
                x: pos.get_column(),
                y: pos.location_line() as usize,
            })
        }),
    ))(input)
}

fn parse_file(input: Span) -> IResult<Span, Vec<Tile>> {
    map(
        separated_list1(complete::newline, many1(parse_tile)),
        |lines| lines.concat(),
    )(input)
}

fn extract(tiles: &[Tile]) -> (Guard, Vec<Wall>) {
    (
        tiles
            .iter()
            .find_map(|x| match x {
                Tile::Guard(guard) => Some(*guard),
                Tile::Wall(_) | Tile::Empty => None,
            })
            .expect("no guard"),
        tiles
            .iter()
            .filter_map(|x| match x {
                Tile::Wall(wall) => Some(*wall),
                Tile::Empty | Tile::Guard(_) => None,
            })
            .collect(),
    )
}

fn find_wall(walls: &[Wall], guard: Guard) -> Option<Wall> {
    match guard.direction {
        Direction::Up => walls
            .iter()
            .filter(|wall| wall.x == guard.x && wall.y > guard.y)
            .min_by_key(|wall| wall.y)
            .copied(),
        Direction::Down => walls
            .iter()
            .filter(|wall| wall.x == guard.x && wall.y < guard.y)
            .max_by_key(|wall| wall.y)
            .copied(),
        Direction::Left => walls
            .iter()
            .filter(|wall| wall.y == guard.y && wall.x < guard.x)
            .max_by_key(|wall| wall.x)
            .copied(),
        Direction::Right => walls
            .iter()
            .filter(|wall| wall.y == guard.y && wall.x > guard.x)
            .min_by_key(|wall| wall.x)
            .copied(),
    }
}

fn walk1(walls: &[Wall], guard: Guard) -> (bool, Guard) {
    find_wall(walls, guard).map_or_else(
        || {
            (
                false,
                match guard.direction {
                    Direction::Up => Guard {
                        x: guard.x,
                        y: walls.iter().max_by_key(|wall| wall.y).expect("no walls").y,
                        direction: guard.direction.next(),
                    },
                    Direction::Down => Guard {
                        x: guard.x,
                        y: walls.iter().min_by_key(|wall| wall.y).expect("no walls").y,
                        direction: guard.direction.next(),
                    },
                    Direction::Left => Guard {
                        y: guard.y,
                        x: walls.iter().min_by_key(|wall| wall.x).expect("no walls").x,
                        direction: guard.direction.next(),
                    },
                    Direction::Right => Guard {
                        y: guard.y,
                        x: walls.iter().max_by_key(|wall| wall.x).expect("no walls").x,
                        direction: guard.direction.next(),
                    },
                },
            )
        },
        |wall| {
            (
                true,
                match guard.direction {
                    Direction::Up => Guard {
                        x: wall.x,
                        y: wall.y - 1,
                        direction: guard.direction.next(),
                    },
                    Direction::Down => Guard {
                        x: wall.x,
                        y: wall.y + 1,
                        direction: guard.direction.next(),
                    },
                    Direction::Left => Guard {
                        x: wall.x + 1,
                        y: wall.y,
                        direction: guard.direction.next(),
                    },
                    Direction::Right => Guard {
                        x: wall.x - 1,
                        y: wall.y,
                        direction: guard.direction.next(),
                    },
                },
            )
        },
    )
}

#[derive(Debug)]
enum WalkResult {
    Cycle,
    Exit(Vec<Guard>),
}

fn walk(walls: &[Wall], mut guard: Guard) -> WalkResult {
    let mut path = vec![guard];
    loop {
        let (hit_wall, next_guard) = walk1(walls, guard);
        path.push(next_guard);
        if !hit_wall {
            return WalkResult::Exit(path);
        }
        if path
            .split_last()
            .expect("empty path")
            .1
            .contains(&next_guard)
        {
            return WalkResult::Cycle;
        }
        guard = next_guard;
    }
}

fn expand_path(positions: &[Guard]) -> HashSet<(usize, usize)> {
    positions
        .windows(2)
        .flat_map(|window| {
            if window[0].x == window[1].x {
                (min(window[0].y, window[1].y)..=max(window[0].y, window[1].y))
                    .map(|y| (window[0].x, y))
                    .collect::<Vec<_>>()
            } else {
                (min(window[0].x, window[1].x)..=max(window[0].x, window[1].x))
                    .map(|x| (x, window[0].y))
                    .collect::<Vec<_>>()
            }
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    let (_, tiles) = parse_file(Span::new(input)).expect("parsing error");
    let (guard, walls) = extract(&tiles);
    match walk(&walls, guard) {
        WalkResult::Cycle => panic!("cycle in main path"),
        WalkResult::Exit(path) => expand_path(&path).len().to_string(),
    }
}

fn find_blockers(walls: &mut Vec<Wall>, guard: Guard) -> usize {
    match walk(walls, guard) {
        WalkResult::Cycle => panic!("cycle in main path"),
        WalkResult::Exit(path) => expand_path(&path),
    }
    .into_iter()
    .map(|(x, y)| Wall { x, y })
    .filter(|wall| {
        walls.push(*wall);
        let result = matches!(walk(walls, guard), WalkResult::Cycle);
        walls.pop();
        result
    })
    .count()
}

pub fn part2(input: &str) -> String {
    let (_, tiles) = parse_file(Span::new(input)).expect("parsing error");
    let (guard, mut walls) = extract(&tiles);
    find_blockers(&mut walls, guard).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 41.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 6.to_string());
    }
}
