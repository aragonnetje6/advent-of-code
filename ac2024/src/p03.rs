use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete,
    combinator::{map, value},
    multi::many1,
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Mul(u64, u64),
    Do,
    Dont,
}

fn op_mul(input: &str) -> IResult<&str, Operation> {
    map(
        delimited(
            tag("mul("),
            separated_pair(complete::u64, complete::char(','), complete::u64),
            complete::char(')'),
        ),
        |(x, y)| Operation::Mul(x, y),
    )(input)
}

fn op_do(input: &str) -> IResult<&str, Operation> {
    value(Operation::Do, tag("do()"))(input)
}

fn op_dont(input: &str) -> IResult<&str, Operation> {
    value(Operation::Dont, tag("don't()"))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Operation>> {
    map(
        many1(alt((
            map(op_mul, Some),
            map(op_do, Some),
            map(op_dont, Some),
            value(None, take(1usize)),
        ))),
        |x| x.into_iter().flatten().collect(),
    )(input)
}

pub fn part1(input: &str) -> String {
    let (_, ops) = parse(input).expect("parsing error");
    ops.into_iter()
        .filter_map(|operation| match operation {
            Operation::Mul(x, y) => Some(x * y),
            Operation::Do | Operation::Dont => None,
        })
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (_, ops) = parse(input).expect("parsing error");
    ops.into_iter()
        .fold((0, true), |(acc, state), val| match val {
            Operation::Mul(x, y) if state => (acc + x * y, true),
            Operation::Mul(_, _) | Operation::Dont => (acc, false),
            Operation::Do => (acc, true),
        })
        .0
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
    const DATA2: &str = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 161.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA2), 48.to_string());
    }
}
