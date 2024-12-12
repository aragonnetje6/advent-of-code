use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::tag, character::complete, combinator::map, multi::separated_list1,
    sequence::separated_pair, IResult,
};

#[derive(Debug, Clone)]
struct Equation {
    target: u128,
    params: Vec<u128>,
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    map(
        separated_pair(
            complete::u128,
            tag(": "),
            separated_list1(complete::space1, complete::u128),
        ),
        |(target, params)| Equation { target, params },
    )(input)
}

fn parse_file(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(complete::newline, parse_equation)(input)
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn eval(self, x: u128, y: u128) -> u128 {
        match self {
            Self::Add => x + y,
            Self::Mul => x * y,
            Self::Concat => (x.to_string() + &y.to_string())
                .parse()
                .expect("number failed to parse?"),
        }
    }
}

fn validate_one(equation: &Equation, operators: &[Operator]) -> bool {
    equation
        .params
        .iter()
        .zip(std::iter::once(Operator::Add).chain(operators.iter().copied()))
        .fold(0, |acc, (param, op)| op.eval(acc, *param))
        == equation.target
}

fn validate(equation: &Equation, operator_options: &[Operator]) -> bool {
    repeat_n(operator_options.iter().copied(), equation.params.len() - 1)
        .multi_cartesian_product()
        .any(|operators| validate_one(equation, &operators))
}

pub fn part1(input: &str) -> String {
    let (_, equations) = parse_file(input).expect("parsing error");
    equations
        .into_iter()
        .filter(|eq| validate(eq, &[Operator::Add, Operator::Mul]))
        .map(|eq| eq.target)
        .sum::<u128>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (_, equations) = parse_file(input).expect("parsing error");
    equations
        .into_iter()
        .filter(|eq| validate(eq, &[Operator::Add, Operator::Mul, Operator::Concat]))
        .map(|eq| eq.target)
        .sum::<u128>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 3749.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 11387.to_string());
    }
}
