use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Vec2<i64>,
    velocity: Vec2<i64>,
}

impl Robot {
    const fn new(position: Vec2<i64>, velocity: Vec2<i64>) -> Self {
        Self { position, velocity }
    }

    fn update(&mut self, width: i64, height: i64) {
        self.position += self.velocity;
        if self.position.x >= width {
            self.position.x -= width;
        }
        if self.position.y >= height {
            self.position.y -= height;
        }
        if self.position.x < 0 {
            self.position.x += width;
        }
        if self.position.y < 0 {
            self.position.y += height;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Vec2<T: Eq + Copy + Ord> {
    x: T,
    y: T,
}

impl<T: Eq + Copy + Ord> Vec2<T> {
    const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Eq + Copy + Ord + std::ops::AddAssign> std::ops::AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: std::ops::Add<Output = T> + Eq + Copy + Ord> std::ops::Add<Self> for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Robot>> {
    separated_list1(
        complete::newline,
        map(
            separated_pair(
                map(
                    preceded(
                        tag("p="),
                        separated_pair(complete::i64, complete::char(','), complete::i64),
                    ),
                    |(x, y)| Vec2::new(x, y),
                ),
                complete::space1,
                map(
                    preceded(
                        tag("v="),
                        separated_pair(complete::i64, complete::char(','), complete::i64),
                    ),
                    |(x, y)| Vec2::new(x, y),
                ),
            ),
            |(position, velocity)| Robot::new(position, velocity),
        ),
    )(input)
}

fn safety_factor(robots: &[Robot], width: i64, height: i64) -> i64 {
    use std::cmp::Ordering;
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for robot in robots {
        match (
            robot.position.x.cmp(&(width / 2)),
            robot.position.y.cmp(&(height / 2)),
        ) {
            (Ordering::Equal, _) | (_, Ordering::Equal) => (),
            (Ordering::Less, Ordering::Less) => q1 += 1,
            (Ordering::Less, Ordering::Greater) => q2 += 1,
            (Ordering::Greater, Ordering::Less) => q3 += 1,
            (Ordering::Greater, Ordering::Greater) => q4 += 1,
        }
    }
    q1 * q2 * q3 * q4
}

fn simulate_robots(input: &str, width: i64, height: i64) -> String {
    let (_, mut robots) = parse_input(input).expect("parsing error");
    for _ in 0..100 {
        for robot in &mut robots {
            robot.update(width, height);
        }
    }
    safety_factor(&robots, width, height).to_string()
}

pub fn part1(input: &str) -> String {
    simulate_robots(input, 101, 103)
}

fn possible_tree(robots: &[Robot]) -> bool {
    robots
        .iter()
        .filter(|robot| {
            robot.position.x >= 21
                && robot.position.x <= 51
                && robot.position.y >= 37
                && robot.position.y <= 69
        })
        .count()
        >= 250
}

fn _display(robots: &[Robot]) {
    let max_y = robots
        .iter()
        .map(|robot| robot.position.y)
        .max()
        .expect("no robots");
    let max_x = robots
        .iter()
        .map(|robot| robot.position.x)
        .max()
        .expect("no robots");
    for y in 0..=max_y {
        println!(
            "{}",
            (0..=max_x)
                .map(
                    |x| if robots.iter().any(|robot| robot.position == Vec2::new(x, y)) {
                        '*'
                    } else {
                        '.'
                    }
                )
                .collect::<String>()
        );
    }
}

pub fn part2(input: &str) -> String {
    let (_, mut robots) = parse_input(input).expect("parsing error");
    for i in 1.. {
        for robot in &mut robots {
            robot.update(101, 103);
        }
        if possible_tree(&robots) {
            return i.to_string();
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn test_part1() {
        assert_eq!(simulate_robots(DATA1, 11, 7), 12.to_string());
    }
}
