use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{all_consuming, map, map_res, value},
    multi::{count, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tag("seeds: "), separated_list1(space1, number))(input)
}

#[derive(Debug, Clone)]
struct MapRange {
    dst_start: u64,
    src_start: u64,
    length: u64,
}

impl MapRange {
    fn apply(&self, x: u64) -> Option<u64> {
        (x >= self.src_start && x < self.src_start + self.length)
            .then(|| x - self.src_start + self.dst_start)
    }

    const fn src_end(&self) -> u64 {
        self.src_start + self.length - 1
    }

    fn apply_range(&self, x: &SeedRange) -> (Vec<SeedRange>, Vec<SeedRange>) {
        if x.end() < self.src_start || x.start > self.src_end() {
            (vec![], vec![x.clone()])
        } else if x.start >= self.src_start && x.end() <= self.src_end() {
            (
                vec![SeedRange {
                    start: x.start - self.src_start + self.dst_start,
                    length: x.length,
                }],
                vec![],
            )
        } else if x.start < self.src_start && x.end() > self.src_end() {
            (
                vec![SeedRange {
                    start: self.dst_start,
                    length: self.length,
                }],
                vec![
                    SeedRange {
                        start: x.start,
                        length: self.src_start - x.start,
                    },
                    SeedRange {
                        start: self.src_end() + 1,
                        length: x.end() - self.src_end(),
                    },
                ],
            )
        } else if x.start < self.src_start {
            (
                vec![SeedRange {
                    start: self.dst_start,
                    length: x.end() - self.src_start + 1,
                }],
                vec![SeedRange {
                    start: x.start,
                    length: self.src_start - x.start,
                }],
            )
        } else {
            (
                vec![SeedRange {
                    start: self.dst_start + x.start - self.src_start,
                    length: x.end() - self.src_start + 1,
                }],
                vec![SeedRange {
                    start: self.src_end() + 1,
                    length: x.end() - self.src_end(),
                }],
            )
        }
    }
}

fn map_range(input: &str) -> IResult<&str, MapRange> {
    map(
        separated_pair(number, space1, separated_pair(number, space1, number)),
        |(dst_start, (src_start, length))| MapRange {
            dst_start,
            src_start,
            length,
        },
    )(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Property {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

fn property(input: &str) -> IResult<&str, Property> {
    alt((
        value(Property::Seed, tag("seed")),
        value(Property::Soil, tag("soil")),
        value(Property::Fertilizer, tag("fertilizer")),
        value(Property::Water, tag("water")),
        value(Property::Light, tag("light")),
        value(Property::Temperature, tag("temperature")),
        value(Property::Humidity, tag("humidity")),
        value(Property::Location, tag("location")),
    ))(input)
}

fn map_name(input: &str) -> IResult<&str, (Property, Property)> {
    terminated(
        separated_pair(property, tag("-to-"), property),
        tag(" map:"),
    )(input)
}

#[derive(Debug)]
struct PropertyMap {
    ranges: Vec<MapRange>,
}

impl PropertyMap {
    fn apply(&self, x: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|range| range.apply(x))
            .unwrap_or(x)
    }

    fn apply_range(&self, seeds: &SeedRange) -> Vec<SeedRange> {
        let mut todo: Vec<SeedRange> = vec![seeds.clone()];
        let mut out = vec![];
        for range in &self.ranges {
            let (new_out, new_todo): (Vec<Vec<SeedRange>>, Vec<Vec<SeedRange>>) =
                todo.into_iter().map(|x| range.apply_range(&x)).unzip();
            todo = new_todo.into_iter().flatten().collect();
            out.extend(new_out.into_iter().flatten());
        }
        out.extend(todo);
        out
    }
}

fn property_map(input: &str) -> IResult<&str, PropertyMap> {
    map(
        preceded(pair(map_name, newline), separated_list1(newline, map_range)),
        |ranges| PropertyMap { ranges },
    )(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<PropertyMap>)> {
    all_consuming(terminated(
        separated_pair(
            seeds,
            count(newline, 2),
            separated_list1(count(newline, 2), property_map),
        ),
        newline,
    ))(input)
}

pub fn part1(input: &str) -> String {
    let (_, (mut seeds, maps)) = parse(input).expect("parsing failure");
    for map in maps {
        seeds = seeds.into_iter().map(|x| map.apply(x)).collect();
    }
    seeds.iter().min().expect("no seeds").to_string()
}

fn seed_ranges(input: &str) -> IResult<&str, Vec<SeedRange>> {
    preceded(
        tag("seeds: "),
        separated_list1(
            space1,
            map(separated_pair(number, space1, number), |(start, length)| {
                SeedRange { start, length }
            }),
        ),
    )(input)
}

#[derive(Debug, Default, Clone)]
struct SeedRange {
    start: u64,
    length: u64,
}

impl SeedRange {
    const fn min(&self) -> u64 {
        self.start
    }

    const fn end(&self) -> u64 {
        self.start + self.length - 1
    }
}

fn parse2(input: &str) -> IResult<&str, (Vec<SeedRange>, Vec<PropertyMap>)> {
    all_consuming(terminated(
        separated_pair(
            seed_ranges,
            count(newline, 2),
            separated_list1(count(newline, 2), property_map),
        ),
        newline,
    ))(input)
}

pub fn part2(input: &str) -> String {
    let (_, (mut seeds, maps)) = parse2(input).expect("parsing failure");
    for map in maps {
        seeds = seeds
            .into_iter()
            .flat_map(|x| map.apply_range(&x))
            .collect();
    }
    seeds
        .iter()
        .map(SeedRange::min)
        .filter(|x| x != &0)
        .min()
        .expect("no seeds")
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 35.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 46.to_string());
    }
}
