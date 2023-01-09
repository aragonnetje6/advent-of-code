use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

fn number<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, str::parse)(input)
}

fn starting_items(input: &str) -> IResult<&str, Vec<u128>> {
    let (input, _) = tag("  Starting items: ")(input)?;
    separated_list1(tag(", "), number)(input)
}

#[derive(Debug)]
enum Operation {
    Multiply(u128),
    Add(u128),
    Square,
}

fn multiply_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("* ")(input)?;
    let (input, operand) = number(input)?;
    Ok((input, Operation::Multiply(operand)))
}

fn add_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("+ ")(input)?;
    let (input, operand) = number(input)?;
    Ok((input, Operation::Add(operand)))
}

fn square_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("* old")(input)?;
    Ok((input, Operation::Square))
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("  Operation: new = old ")(input)?;
    alt((multiply_operation, add_operation, square_operation))(input)
}

fn test(input: &str) -> IResult<&str, u128> {
    let (input, _) = tag("  Test: divisible by ")(input)?;
    number(input)
}

fn if_true(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("    If true: throw to monkey ")(input)?;
    number(input)
}

fn if_false(input: &str) -> IResult<&str, usize> {
    let (input, _) = tag("    If false: throw to monkey ")(input)?;
    number(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = tag("Monkey ")(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(":\n")(input)?;
    let (input, items) = starting_items(input)?;
    let (input, _) = newline(input)?;
    let (input, operation) = operation(input)?;
    let (input, _) = newline(input)?;
    let (input, test) = test(input)?;
    let (input, _) = newline(input)?;
    let (input, if_true) = if_true(input)?;
    let (input, _) = newline(input)?;
    let (input, if_false) = if_false(input)?;
    let (input, _) = newline(input)?;
    Ok((
        input,
        Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            inspections: 0,
        },
    ))
}

fn monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, monkey)(input)
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test: u128,
    if_true: usize,
    if_false: usize,
    inspections: u128,
}

fn run_monkeys(monkeys: &mut Vec<Monkey>, div_by_three: bool) {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            monkeys[i].inspections += 1;
            let item = monkeys[i].items.drain(..1).next().unwrap();
            let operated_item = match monkeys[i].operation {
                Operation::Multiply(x) => item * x,
                Operation::Add(x) => item + x,
                Operation::Square => item.pow(2),
            } / (if div_by_three {3} else {1});
            let destination = if operated_item % monkeys[i].test == u128::from(0u8) {
                monkeys[i].if_true
            } else {
                monkeys[i].if_false
            };
            monkeys[destination].items.push(operated_item);
        }
    }
}

pub fn part1(input: &str) -> u128 {
    let (_, mut data) = monkeys(input).unwrap();
    for _ in 0..20 {
        run_monkeys(&mut data, true);
    }
    data.sort_unstable_by_key(|x| x.inspections);
    data.iter()
        .rev()
        .take(2)
        .map(|x| x.inspections)
        .reduce(|acc, new| acc * new)
        .unwrap()
}

pub fn part2(input: &str) -> u128 {
    let (_, mut data) = monkeys(input).unwrap();
    for _ in 0..650 {
        run_monkeys(&mut data, false);
    }
    data.sort_unstable_by_key(|x| x.inspections);
    data.iter()
        .rev()
        .take(2)
        .map(|x| x.inspections)
        .reduce(|acc, new| acc * new)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 2_713_310_158);
    }
}
