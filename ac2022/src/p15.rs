#![allow(clippy::cast_possible_wrap)]

use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
    sequence::separated_pair, IResult,
};

fn location(input: &str) -> IResult<&str, (i32, i32)> {
    let (input, _) = tag("x=")(input)?;
    separated_pair(
        nom::character::complete::i32,
        tag(", y="),
        nom::character::complete::i32,
    )(input)
}

fn beacon(input: &str) -> IResult<&str, Beacon> {
    let (input, _) = tag("closest beacon is at ")(input)?;
    map(location, |(x, y)| Beacon { x, y })(input)
}

fn sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, (x, y)) = location(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, beacon) = beacon(input)?;
    Ok((input, Sensor { x, y, beacon }))
}

fn parse_data(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(newline, sensor)(input)
}

trait Location {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn hamming_dist(&self, other: &impl Location) -> u32 {
        self.x().abs_diff(other.x()) + self.y().abs_diff(other.y())
    }
}

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    beacon: Beacon,
}

impl Location for Sensor {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

impl Sensor {
    fn range(&self) -> u32 {
        self.hamming_dist(&self.beacon)
    }
}

#[derive(Debug)]
struct Beacon {
    x: i32,
    y: i32,
}

impl Location for Beacon {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

pub fn part1(input: &str) -> usize {
    let (_, sensors) = parse_data(input).unwrap();
    let min_x = sensors
        .iter()
        .map(|sensor| sensor.x - sensor.range() as i32)
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|sensor| sensor.x + sensor.range() as i32)
        .max()
        .unwrap();
    #[cfg(not(test))]
    let y = 2_000_000;
    #[cfg(test)]
    let y = 10;
    (min_x..=max_x)
        .filter(|&x| {
            sensors
                .iter()
                .any(|sensor| sensor.range() >= sensor.hamming_dist(&Beacon { x, y }))
                && !sensors
                    .iter()
                    .any(|Sensor { beacon, .. }| beacon.x == x && beacon.y == y)
        })
        .count()
}

pub fn part2(input: &str) -> i64 {
    let (_, sensors) = parse_data(input).unwrap();
    #[cfg(not(test))]
    let max = 4_000_000;
    #[cfg(test)]
    let max = 20;
    let (x, y) = find_gap(&sensors, max);
    i64::from(x) * 4_000_000 + i64::from(y)
}

fn find_gap(sensors: &[Sensor], max: i32) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    while let Some(sensor) = sensors
        .iter()
        .find(|sensor| sensor.range() >= sensor.hamming_dist(&Beacon { x, y }))
    {
        x += 1 + sensor.x - x + (sensor.range() - y.abs_diff(sensor.y)) as i32;
        if x > max {
            y += 1;
            x = 0;
        }
    }
    (x, y)
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 56_000_011);
    }
}
