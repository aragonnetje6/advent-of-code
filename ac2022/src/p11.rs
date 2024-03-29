use std::collections::HashMap;
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

fn starting_items(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("  Starting items: ")(input)?;
    separated_list1(tag(", "), number)(input)
}

#[derive(Debug, Clone)]
enum Operation {
    Multiply(u32),
    Add(u32),
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

fn test(input: &str) -> IResult<&str, u32> {
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

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: u32,
    if_true: usize,
    if_false: usize,
    inspections: u128,
}

fn run_monkeys(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            monkeys[i].inspections += 1;
            let item = monkeys[i].items.drain(..1).next().unwrap();
            let operated_item = match monkeys[i].operation {
                Operation::Multiply(x) => item * x,
                Operation::Add(x) => item + x,
                Operation::Square => item.pow(2),
            } / 3;
            let destination = if operated_item % monkeys[i].test == u32::from(0u8) {
                monkeys[i].if_true
            } else {
                monkeys[i].if_false
            };
            monkeys[destination].items.push(operated_item);
        }
    }
}

#[derive(Debug)]
struct Item {
    modulo_primes: HashMap<u32, u32>,
}

impl Item {
    fn new(primes: &[u32], value: u32) -> Self {
        Self {
            modulo_primes: primes.iter().map(|prime| (*prime, value % prime)).collect(),
        }
    }

    fn add(&mut self, x: u32) {
        for (prime, modulo) in &mut self.modulo_primes {
            *modulo = (*modulo + x) % prime;
        }
    }

    fn multiply(&mut self, x: u32) {
        for (prime, modulo) in &mut self.modulo_primes {
            *modulo = (*modulo * x) % prime;
        }
    }

    fn square(&mut self) {
        for (prime, modulo) in &mut self.modulo_primes {
            *modulo = modulo.pow(2) % prime;
        }
    }

    fn is_divisible_by(&self, x: u32) -> bool {
        *self.modulo_primes.get(&x).unwrap() == 0
    }
}

#[derive(Debug)]
struct SmartMonkey {
    items: Vec<Item>,
    operation: Operation,
    test: u32,
    if_true: usize,
    if_false: usize,
    inspections: u128,
}

impl SmartMonkey {
    fn from_monkeys(monkeys: &[Monkey]) -> Vec<Self> {
        let primes: Vec<u32> = monkeys
            .iter()
            .map(|x| x.test)
            .chain(monkeys.iter().filter_map(|monkey| match monkey.operation {
                Operation::Multiply(x) => Some(x),
                _ => None,
            }))
            .collect();
        monkeys
            .iter()
            .cloned()
            .map(
                |Monkey {
                     items,
                     operation,
                     test,
                     if_true,
                     if_false,
                     inspections,
                 }| Self {
                    operation,
                    test,
                    if_true,
                    if_false,
                    inspections,
                    items: items.iter().map(|x| Item::new(&primes, *x)).collect(),
                },
            )
            .collect()
    }
}

fn run_monkeys2(monkeys: &mut [SmartMonkey]) {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            monkeys[i].inspections += 1;
            let mut item = monkeys[i].items.drain(..1).next().unwrap();
            match monkeys[i].operation {
                Operation::Multiply(x) => item.multiply(x),
                Operation::Add(x) => item.add(x),
                Operation::Square => item.square(),
            };
            let destination = if item.is_divisible_by(monkeys[i].test) {
                monkeys[i].if_true
            } else {
                monkeys[i].if_false
            };
            monkeys[destination].items.push(item);
        }
    }
}

pub fn part1(input: &str) -> String {
    let (_, mut data) = monkeys(input).unwrap();
    for _ in 1..=20 {
        run_monkeys(&mut data);
    }
    data.sort_unstable_by_key(|x| x.inspections);
    data.iter()
        .rev()
        .take(2)
        .map(|x| x.inspections)
        .reduce(|acc, new| acc * new)
        .unwrap()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let (_, data) = monkeys(input).unwrap();
    let mut smart_monkeys = SmartMonkey::from_monkeys(&data);
    for _ in 1..=10000 {
        run_monkeys2(&mut smart_monkeys);
    }
    smart_monkeys.sort_unstable_by_key(|x| x.inspections);
    smart_monkeys
        .iter()
        .rev()
        .take(2)
        .map(|x| x.inspections)
        .reduce(|acc, new| acc * new)
        .unwrap()
        .to_string()
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
        assert_eq!(part1(DATA1), "10605");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), "2713310158");
    }
}
