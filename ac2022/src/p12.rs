use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::rc::Rc;

use nom::character::complete::{anychar, newline};
use nom::character::is_alphabetic;
use nom::combinator::verify;
use nom::multi::{many1, separated_list1};
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Normal(u32),
    Start,
    End,
}

fn tile(input: &str) -> IResult<&str, Tile> {
    let (input, c) = verify(anychar, |x| is_alphabetic(*x as u8))(input)?;
    let tile = match c {
        'S' => Tile::Start,
        'E' => Tile::End,
        x => Tile::Normal(u32::from(x)),
    };
    Ok((input, tile))
}

fn line(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(tile)(input)
}

fn grid(input: &str) -> IResult<&str, Grid> {
    separated_list1(newline, line)(input)
}

type Grid = Vec<Vec<Tile>>;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Point(usize, usize);

#[derive(Debug, Clone)]
struct PathPoint {
    point: Point,
    cost: usize,
    end: Point,
    parent: Option<Rc<PathPoint>>,
}

impl PathPoint {
    fn new(point: Point, end: Point, parent: Option<Rc<PathPoint>>) -> Self {
        Self {
            point,
            end,
            parent: parent.clone(),
            cost: if let Some(maybe_parent) = parent {
                maybe_parent.cost + 1
            } else {
                0
            },
        }
    }

    const fn total_cost(&self) -> usize {
        self.cost + self.end.0.abs_diff(self.point.0) + self.end.1.abs_diff(self.point.1)
    }
}

impl Pointy for PathPoint {
    fn get_point(&self) -> Point {
        self.point
    }
}

impl Nested for PathPoint {
    fn get_parent(&self) -> Option<Self> {
        self.parent.clone().map(|x| (*x).clone())
    }
}

impl PartialEq<Self> for PathPoint {
    fn eq(&self, other: &Self) -> bool {
        self.total_cost() == other.total_cost()
    }
}

impl Eq for PathPoint {}

impl PartialOrd<Self> for PathPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total_cost().partial_cmp(&other.total_cost())
    }
}

impl Ord for PathPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_cost().cmp(&other.total_cost())
    }
}

fn get_neighbours(point: &Rc<PathPoint>, grid: &Grid, end: Point) -> Vec<Rc<PathPoint>> {
    let mut out: Vec<Point> = vec![];
    if point.point.0 > 0 {
        out.push(Point(point.point.0 - 1, point.point.1));
    }
    if point.point.0 < grid.len() - 1 {
        out.push(Point(point.point.0 + 1, point.point.1));
    }
    if point.point.1 > 0 {
        out.push(Point(point.point.0, point.point.1 - 1));
    }
    if point.point.1 < grid[0].len() - 1 {
        out.push(Point(point.point.0, point.point.1 + 1));
    }
    out.drain(..)
        .map(|x| PathPoint::new(x, end, Some(point.clone())))
        .filter(|neighbour| {
            i64::from(neighbour.get_height(grid)) - i64::from(point.get_height(grid)) <= 1
        })
        .map(Rc::new)
        .collect()
}

fn a_star(grid: &Grid, start: Point, end: Point) -> Vec<Point> {
    let mut queue: BinaryHeap<Reverse<Rc<PathPoint>>> =
        BinaryHeap::from_iter(vec![Reverse(Rc::new(PathPoint::new(start, end, None)))]);
    let mut closed: Vec<Rc<PathPoint>> = vec![];
    loop {
        let current = queue.pop().unwrap().0;
        let neighbours = get_neighbours(&current, grid, end);
        for neighbour in neighbours {
            if neighbour.point == end {
                return neighbour.get_path();
            } else if !queue
                .iter()
                .any(|x| x.0.point == neighbour.point && x.0.cost <= neighbour.cost)
                && !closed
                    .iter()
                    .any(|x| x.point == neighbour.point && x.cost <= neighbour.cost)
            {
                queue.push(Reverse(neighbour));
            }
        }
        closed.retain(|x| x.point != current.point || *x < current);
        closed.push(current);
    }
}

fn find_tile(grid: &Grid, target: &Tile) -> Option<Point> {
    grid.iter().enumerate().find_map(|(x, row)| {
        row.iter().enumerate().find_map(|(y, tile)| {
            if *tile == *target {
                Some(Point(x, y))
            } else {
                None
            }
        })
    })
}

pub fn part1(input: &str) -> usize {
    let (_, data) = grid(input).unwrap();
    let start = find_tile(&data, &Tile::Start).unwrap();
    let end = find_tile(&data, &Tile::End).unwrap();
    let path = a_star(&data, start, end);
    path.len() - 1
}

#[derive(Debug, Clone)]
struct SimplePathPoint {
    point: Point,
    parent: Option<Box<SimplePathPoint>>,
}

impl SimplePathPoint {
    fn new(point: Point, parent: Option<Box<SimplePathPoint>>) -> Self {
        Self { point, parent }
    }
}

impl Pointy for SimplePathPoint {
    fn get_point(&self) -> Point {
        self.point
    }
}

impl Nested for SimplePathPoint {
    fn get_parent(&self) -> Option<Self> {
        self.parent.clone().map(|x| *x)
    }
}

trait Pointy {
    fn get_point(&self) -> Point;
}

trait Nested: Sized {
    fn get_parent(&self) -> Option<Self>;
}

trait Height: Pointy {
    fn get_height(&self, grid: &Grid) -> u32 {
        match *grid[self.get_point().0].get(self.get_point().1).unwrap() {
            Tile::Normal(x) => x,
            Tile::Start => u32::from('a'),
            Tile::End => u32::from('z'),
        }
    }
}

impl<T: Pointy> Height for T {}

trait Path: Pointy + Nested {
    fn get_path(&self) -> Vec<Point> {
        if let Some(parent) = &self.get_parent() {
            let mut out = parent.get_path();
            out.push(self.get_point());
            out
        } else {
            vec![self.get_point()]
        }
    }
}

impl<T: Pointy + Nested> Path for T {}

fn get_neighbours2(point: &SimplePathPoint, grid: &Grid) -> Vec<SimplePathPoint> {
    let mut out: Vec<Point> = vec![];
    if point.point.0 > 0 {
        out.push(Point(point.point.0 - 1, point.point.1));
    }
    if point.point.0 < grid.len() - 1 {
        out.push(Point(point.point.0 + 1, point.point.1));
    }
    if point.point.1 > 0 {
        out.push(Point(point.point.0, point.point.1 - 1));
    }
    if point.point.1 < grid[0].len() - 1 {
        out.push(Point(point.point.0, point.point.1 + 1));
    }
    out.drain(..)
        .map(|x| SimplePathPoint::new(x, Some(Box::new(point.clone()))))
        .filter(|neighbour| {
            i64::from(point.get_height(grid)) - i64::from(neighbour.get_height(grid)) <= 1
        })
        .collect()
}

fn bfs(grid: &Grid, start: Point, goal: u32) -> Vec<Point> {
    let mut queue = vec![SimplePathPoint::new(start, None)];
    let mut closed = HashSet::new();
    loop {
        let current = queue.pop().unwrap();
        let neighbours = get_neighbours2(&current, grid);
        for neighbour in neighbours {
            if neighbour.get_height(grid) == goal {
                return neighbour.get_path();
            } else if !queue.iter().any(|x| x.point == neighbour.point)
                && !closed.contains(&neighbour.point)
            {
                queue.insert(0, neighbour);
            }
        }
        closed.insert(current.point);
    }
}

pub fn part2(input: &str) -> usize {
    let (_, data) = grid(input).unwrap();
    let end = find_tile(&data, &Tile::End).unwrap();
    let path = bfs(&data, end, u32::from('a'));
    path.len() - 1
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 29);
    }
}
