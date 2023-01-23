use std::collections::HashSet;
use std::str::FromStr;

use nom::character::complete::{alpha1, char, digit1, newline};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => panic!("Invalid character {s}"),
        })
    }
}

fn direction(input: &str) -> IResult<&str, Direction> {
    map_res(alpha1, str::parse)(input)
}

#[derive(Debug, Clone, Copy)]
struct Motion {
    direction: Direction,
    amount: u8,
}

fn motion(input: &str) -> IResult<&str, Motion> {
    let (input, direction) = direction(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, amount) = map_res(digit1, str::parse)(input)?;
    Ok((input, Motion { direction, amount }))
}

fn motions(input: &str) -> IResult<&str, Vec<Motion>> {
    separated_list1(newline, motion)(input)
}

type Point = (i32, i32);

struct Path {
    knots: Vec<Point>,
    tail_visited: HashSet<Point>,
}

impl Path {
    fn new(knots: usize) -> Self {
        Self {
            knots: vec![(0, 0); knots],
            tail_visited: HashSet::from_iter(vec![(0, 0)]),
        }
    }

    fn execute_motion(&mut self, motion: Motion) {
        for _ in 0..motion.amount {
            self.step(motion.direction);
        }
    }

    fn step(&mut self, direction: Direction) {
        let change = match direction {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        self.knots[0].0 += change.0;
        self.knots[0].1 += change.1;
        for i in 1..self.knots.len() {
            self.move_knot(i);
        }
        self.tail_visited.insert(*self.knots.last().unwrap());
    }

    fn move_knot(&mut self, index: usize) {
        if (self.knots[index - 1].0.abs_diff(self.knots[index].0) == 2)
            && (self.knots[index - 1].1.abs_diff(self.knots[index].1) == 2)
        {
            self.knots[index].0 = (self.knots[index].0 + self.knots[index - 1].0) / 2;
            self.knots[index].1 = (self.knots[index].1 + self.knots[index - 1].1) / 2;
        } else if self.knots[index - 1].0.abs_diff(self.knots[index].0) == 2 {
            self.knots[index].0 = (self.knots[index].0 + self.knots[index - 1].0) / 2;
            self.knots[index].1 = self.knots[index - 1].1;
        } else if self.knots[index - 1].1.abs_diff(self.knots[index].1) == 2 {
            self.knots[index].1 = (self.knots[index].1 + self.knots[index - 1].1) / 2;
            self.knots[index].0 = self.knots[index - 1].0;
        }
    }

    const fn get_tail_visited_points(&self) -> &HashSet<Point> {
        &self.tail_visited
    }
}

pub fn part1(input: &str) -> String {
    let (_, data) = motions(input).unwrap();
    let mut path = Path::new(2);
    for motion in data {
        path.execute_motion(motion);
    }
    path.get_tail_visited_points().len().to_string()
}

pub fn part2(input: &str) -> String {
    let (_, data) = motions(input).unwrap();
    let mut path = Path::new(10);
    for motion in data {
        path.execute_motion(motion);
    }
    path.get_tail_visited_points().len().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
    const DATA2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), "13");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), "1");
        assert_eq!(part2(DATA2), "36");
    }
}
