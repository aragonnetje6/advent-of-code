use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

use nom::bytes::complete::tag;
use nom::{
    character::complete::{char, newline},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(nom::character::complete::u16, usize::from)(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(parse_usize, char(','), parse_usize),
        Point::from,
    )(input)
}

fn path(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), point)(input)
}

fn formation(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(newline, path)(input)
}

#[derive(Debug, Clone)]
struct CaveWall {
    grid: Vec<Vec<Tile>>,
    x_min: usize,
    y_min: usize,
    x_max: usize,
    y_max: usize,
    has_floor: bool,
}

impl CaveWall {
    fn add_path(&mut self, path: &[Point]) {
        for line in path.windows(2) {
            if let [start, end] = line {
                if start.x == end.x {
                    for y in min(start.y, end.y)..=max(start.y, end.y) {
                        self[(start.x, y)] = Tile::Rock;
                    }
                } else if start.y == end.y {
                    for x in min(start.x, end.x)..=max(start.x, end.x) {
                        self[(x, start.y)] = Tile::Rock;
                    }
                } else {
                    panic!("non-straight line")
                }
            }
        }
    }

    fn drop_sand(&mut self, mut x: usize, mut y: usize) -> bool {
        if self[(x, y)] == Tile::Sand {
            return false;
        }
        loop {
            if y == self.y_max {
                return if self.has_floor {
                    self[(x, y)] = Tile::Sand;
                    true
                } else {
                    false
                };
            } else if self[(x, y + 1)] == Tile::Air {
                y += 1;
            } else if x == self.x_min {
                return false;
            } else if self[(x - 1, y + 1)] == Tile::Air {
                y += 1;
                x -= 1;
            } else if x == self.x_max {
                return false;
            } else if self[(x + 1, y + 1)] == Tile::Air {
                y += 1;
                x += 1;
            } else {
                self[(x, y)] = Tile::Sand;
                return true;
            }
        }
    }

    fn count(&self, tile: Tile) -> usize {
        self.grid.iter().flatten().filter(|x| **x == tile).count()
    }
}

impl Index<(usize, usize)> for CaveWall {
    type Output = Tile;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.grid[y - self.y_min][x - self.x_min]
    }
}

impl IndexMut<(usize, usize)> for CaveWall {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.grid[y - self.y_min][x - self.x_min]
    }
}

impl TryFrom<&[Vec<Point>]> for CaveWall {
    type Error = String;
    fn try_from(formation: &[Vec<Point>]) -> Result<Self, Self::Error> {
        let x_min = 0;
        let y_min = 0;
        let x_max = 1000;
        let y_max = *formation
            .iter()
            .flatten()
            .map(|Point { y, .. }| y)
            .max()
            .ok_or_else(|| "invalid".to_string())?
            + 1;
        let grid = vec![vec![Tile::Air; x_max - x_min + 1]; y_max - y_min + 1];
        let mut result = Self {
            grid,
            x_min,
            y_min,
            x_max,
            y_max,
            has_floor: false,
        };
        for path in formation {
            result.add_path(path);
        }
        Ok(result)
    }
}

impl Display for CaveWall {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.grid
                .iter()
                .map(|row| row.iter().map(Tile::to_string).collect::<String>())
                .reduce(|x, y| format!("{x}\n{y}"))
                .unwrap()
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Air => '.',
                Self::Rock => '#',
                Self::Sand => 'O',
            }
        )
    }
}

pub fn part1(input: &str) -> String {
    let (_, data) = formation(input).unwrap();
    let mut wall = CaveWall::try_from(&*data).unwrap();
    while wall.drop_sand(500, 0) {}
    wall.count(Tile::Sand).to_string()
}

pub fn part2(input: &str) -> String {
    let (_, data) = formation(input).unwrap();
    let mut wall = CaveWall::try_from(&*data).unwrap();
    wall.has_floor = true;
    while wall.drop_sand(500, 0) {}
    wall.count(Tile::Sand).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), "24");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), "93");
    }
}
