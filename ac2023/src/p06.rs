use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse::<u64>)(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, number)(input)
}

fn times(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(pair(tag("Time: "), space1), numbers)(input)
}

fn distances(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(pair(tag("Distance: "), space1), numbers)(input)
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    map(separated_pair(times, newline, distances), |(t, d)| {
        t.into_iter().zip(d).collect()
    })(input)
}

pub fn part1(input: &str) -> String {
    let (_, races) = parse(input).expect("parsing failure");
    races
        .iter()
        .map(|(time, distance)| {
            (1..*time)
                .map(|v| v * (*time - v))
                .filter(|d| d > distance)
                .count()
        })
        .product::<usize>()
        .to_string()
}

fn number2(input: &str) -> IResult<&str, u64> {
    map_res(separated_list1(space1, digit1), |x| {
        x.join("").parse::<u64>()
    })(input)
}

fn times2(input: &str) -> IResult<&str, u64> {
    preceded(pair(tag("Time: "), space1), number2)(input)
}

fn distances2(input: &str) -> IResult<&str, u64> {
    preceded(pair(tag("Distance: "), space1), number2)(input)
}

fn parse2(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(times2, newline, distances2)(input)
}

pub fn part2(input: &str) -> String {
    let (_, (time, distance)) = parse2(input).expect("parsing failed");
    (1..time)
        .map(|v| v * (time - v))
        .filter(|d| d > &distance)
        .count()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 288.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 71503.to_string());
    }
}
