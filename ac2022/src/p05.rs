use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while_m_n};
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Copy, Clone, Debug)]
enum Element {
Present(char),
Empty,
}

#[derive(Copy, Clone, Debug)]
struct Move {
    amt: u32,
    from: usize,
    to: usize,
}

impl Move {
    fn execute<T>(&self, stack: &mut [Vec<T>]) {
        for _ in 0..self.amt {
            let elem = stack[self.from - 1].pop().unwrap();
            stack[self.to - 1].push(elem);
        }
    }

    fn execute_9001<T>(&self, stack: &mut [Vec<T>]) {
        let mut intermediate = vec![];
        for _ in 0..self.amt {
            intermediate.push(stack[self.from - 1].pop().unwrap());
        }
        for _ in 0..self.amt {
            stack[self.to - 1].push(intermediate.pop().unwrap());
        }
    }
}

fn present_element(input: &str) -> IResult<&str, Element> {
    let (input, _) = tag("[")(input)?;
    let (input, c) = map_res(take(1usize), char::from_str)(input)?;
    let (input, _) = tag("]")(input)?;
    Ok((input, Element::Present(c)))
}

fn empty_element(input: &str) -> IResult<&str, Element> {
    map(take_while_m_n(3, 3, |x| x == ' '), |_| Element::Empty)(input)
}

fn element(input: &str) -> IResult<&str, Element> {
    alt((present_element, empty_element))(input)
}

fn stack_row(input: &str) -> IResult<&str, Vec<Element>> {
    separated_list1(tag(" "), element)(input)
}

fn stack_index(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag(" ")(input)?;
    let (input, i) = map_res(take(1usize), u8::from_str)(input)?;
    let (input, _) = tag(" ")(input)?;
    Ok((input, i))
}

fn index_row(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list1(tag(" "), stack_index)(input)
}

fn transpose<T: Copy>(original: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..original[0].len())
    .map(|index2| {
            (0..original.len())
            .map(|index1| original[index1][index2])
            .collect()
        })
    .collect()
}

fn stack(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, mut stack) = separated_list1(tag("\n"), stack_row)(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, _) = index_row(input)?;
    stack.reverse();
    let stack = transpose(&stack);
    let stack = stack
    .iter()
    .map(|line| {
            line.iter()
            .filter_map(|elem| match *elem {
                        Element::Present(x) => Some(x),
                        Element::Empty => None,
                })
            .collect()
        })
    .collect();
    Ok((input, stack))
}

fn instruction(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, amt) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = map_res(digit1, str::parse)(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = map_res(digit1, str::parse)(input)?;
    Ok((input, Move { amt, from, to }))
}

fn instructions(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(tag("\n"), instruction)(input)
}

fn parse_all(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Move>)> {
    let (input, stack) = stack(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, orders) = instructions(input)?;
    Ok((input, (stack, orders)))
}

pub fn part1(input: &str) -> String {
    let (_, (mut stack, orders)) = parse_all(input).unwrap();
    orders.iter().for_each(|order| order.execute(&mut stack));
    stack.iter().map(|line| line.last().unwrap()).collect()
}

pub fn part2(input: &str) -> String {
    let (_, (mut stack, orders)) = parse_all(input).unwrap();
    orders
        .iter()
        .for_each(|order| order.execute_9001(&mut stack));
    stack.iter().map(|line| line.last().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = "    [D]\x20\x20\x20\x20
[N] [C]\x20\x20\x20\x20
[Z] [M] [P]
 1   2   3\x20

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "CMZ");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), "MCD");
    }
}
