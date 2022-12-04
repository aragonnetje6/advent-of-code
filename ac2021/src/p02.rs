use crate::p02::Command::{Down, Forward, Up};

enum Command {
    Forward(u128),
    Down(u128),
    Up(u128),
}

pub fn part1(input: &str) -> u128 {
    let commands: Vec<Command> = parse_commands(input);
    let mut depth = 0u128;
    let mut pos = 0u128;
    for command in commands {
        match command {
            Forward(x) => pos += x,
            Down(x) => depth += x,
            Up(x) => depth -= x,
        }
    }
    depth * pos
}

pub fn part2(input: &str) -> u128 {
    let commands: Vec<Command> = parse_commands(input);
    let mut depth = 0u128;
    let mut pos = 0u128;
    let mut aim = 0u128;
    for command in commands {
        match command {
            Forward(x) => {
                pos += x;
                depth += aim * x;
            }
            Down(x) => aim += x,
            Up(x) => aim -= x,
        }
    }
    depth * pos
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .split('\n')
        .filter_map(|comm: &str| {
            let mut iter = comm.split(' ');
            let command = iter.next()?;
            let val_str = iter.next()?;
            let val: u128 = val_str.parse().ok()?;
            Some(match command {
                "forward" => Forward(val),
                "up" => Up(val),
                "down" => Down(val),
                &_ => unreachable!(),
            })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n"),
            150
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n"),
            900
        );
    }
}
