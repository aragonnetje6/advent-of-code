use std::cmp::{max, max_by_key, min, min_by_key};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(',').map(|x| x.parse::<usize>().unwrap());
        Ok(Self {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
        })
    }
}

impl Point {
    const fn same_axis(&self, other: &Self) -> bool {
        self.x == other.x || self.y == other.y
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ").map(|x| x.parse::<Point>().unwrap());
        Ok(Self {
            start: points.next().unwrap(),
            end: points.next().unwrap(),
        })
    }
}

impl Line {
    const fn straight(&self) -> bool {
        self.start.same_axis(&self.end)
    }

    fn max_coord(&self) -> usize {
        max(max(self.start.x, self.end.x), max(self.start.y, self.end.y))
    }

    fn get_points(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            (min(self.start.y, self.end.y)..=max(self.start.y, self.end.y))
                .map(|y| Point { x: self.start.x, y })
                .collect()
        } else if self.start.y == self.end.y {
            (min(self.start.x, self.end.x)..=max(self.start.x, self.end.x))
                .map(|x| Point { y: self.start.y, x })
                .collect()
        } else {
            let lower = min_by_key(self.start, self.end, |point| point.x);
            let higher = max_by_key(self.start, self.end, |point| point.x);
            if lower.y < higher.y {
                (lower.x..=higher.x)
                    .zip(lower.y..=higher.y)
                    .map(|(x, y)| Point { x, y })
                    .collect()
            } else {
                (lower.x..=higher.x)
                    .zip((higher.y..=lower.y).rev())
                    .map(|(x, y)| Point { x, y })
                    .collect()
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Seabed {
    field: Vec<Vec<u32>>,
}

impl Seabed {
    fn new(lines: &Vec<Line>) -> Self {
        let size: usize = lines.iter().map(Line::max_coord).max().unwrap();
        let mut field = vec![vec![0; size + 1]; size + 1];
        for line in lines {
            line.get_points()
                .iter()
                .for_each(|point| field[point.x][point.y] += 1);
        }
        Self { field }
    }

    fn get_greater_than(&self, n: u32) -> usize {
        self.field
            .iter()
            .map(|line| line.iter().filter(|x| **x > n).count())
            .sum()
    }
}

fn process_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| line.parse::<Line>().unwrap())
        .collect()
}

pub fn part1(input: &str) -> String {
    let data = process_input(input);
    let straight_data = data.iter().filter(|x| x.straight()).cloned().collect();
    let seabed = Seabed::new(&straight_data);
    seabed.get_greater_than(1).to_string()
}

pub fn part2(input: &str) -> String {
    let data = process_input(input);
    let seabed = Seabed::new(&data);
    seabed.get_greater_than(1).to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "5");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), "12");
    }
}
