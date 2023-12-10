use itertools::Itertools;
use nom::{
    character::complete::{self, newline, space1},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    all_consuming(terminated(
        separated_list1(newline, separated_list1(space1, complete::i64)),
        newline,
    ))(input)
}

fn calc_differences(data: &[i64]) -> Vec<i64> {
    data.iter().tuple_windows().map(|(a, b)| b - a).collect()
}

fn predict_next(data: &[i64]) -> i64 {
    if data.iter().all_equal() {
        *data.first().expect("empty data")
    } else {
        data.last().expect("empty data") + predict_next(&calc_differences(data))
    }
}

pub fn part1(input: &str) -> String {
    let (_, patterns) = parse(input).expect("parsing failed");
    patterns
        .iter()
        .map(|x| predict_next(x))
        .sum::<i64>()
        .to_string()
}

fn predict_prev(data: &[i64]) -> i64 {
    if data.iter().all_equal() {
        *data.first().expect("empty data")
    } else {
        data.first().expect("empty data") - predict_prev(&calc_differences(data))
    }
}

pub fn part2(input: &str) -> String {
    let (_, patterns) = parse(input).expect("parsing failed");
    patterns
        .iter()
        .map(|x| predict_prev(x))
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 114.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 2.to_string());
    }
}
