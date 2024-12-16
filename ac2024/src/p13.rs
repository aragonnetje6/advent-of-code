use nom::{
    bytes::complete::tag,
    character::complete,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Button {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Copy)]
struct Prize {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

fn parse_button(input: &str) -> IResult<&str, Button> {
    terminated(
        map(
            preceded(
                separated_pair(tag("Button "), complete::one_of("AB"), tag(": X+")),
                separated_pair(complete::u64, tag(", Y+"), complete::u64),
            ),
            |(x, y)| Button { x, y },
        ),
        complete::newline,
    )(input)
}

fn parse_prize(input: &str) -> IResult<&str, Prize> {
    terminated(
        map(
            preceded(
                tag("Prize: X="),
                separated_pair(complete::u64, tag(", Y="), complete::u64),
            ),
            |(x, y)| Prize { x, y },
        ),
        complete::newline,
    )(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    map(
        tuple((parse_button, parse_button, parse_prize)),
        |(button_a, button_b, prize)| Machine {
            button_a,
            button_b,
            prize,
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Machine>> {
    separated_list1(complete::newline, parse_machine)(input)
}

fn get_optimal_path(machine: Machine) -> Option<(u64, u64)> {
    let Machine {
        button_a,
        button_b,
        prize,
    } = machine;
    let mut out = None;
    for a_count in 0.. {
        if a_count * button_a.x > prize.x || a_count * button_a.y > prize.y {
            break;
        }
        let remaining_x = prize.x - a_count * button_a.x;
        let remaining_y = prize.y - a_count * button_a.y;
        if !(remaining_x % button_b.x == 0
            && remaining_y % button_b.y == 0
            && remaining_x / button_b.x == remaining_y / button_b.y)
        {
            continue;
        }
        dbg!(a_count);
        let b_count = remaining_x / button_b.x;
        if let Some((a_old, b_old)) = out {
            if a_count * 3 + b_count < a_old * 3 + b_old {
                out = Some((a_count, b_count));
            }
        } else {
            out = Some((a_count, b_count));
        }
    }
    out
}

pub fn part1(input: &str) -> String {
    parse_input(input)
        .expect("parsing error")
        .1
        .into_iter()
        .filter_map(get_optimal_path)
        .map(|(r#as, bs)| r#as * 3 + bs)
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    parse_input(input)
        .expect("parsing error")
        .1
        .into_iter()
        .map(
            |Machine {
                 button_a,
                 button_b,
                 prize: Prize { x, y },
             }| Machine {
                button_a,
                button_b,
                prize: Prize {
                    x: x + 10_000_000_000_000,
                    y: y + 10_000_000_000_000,
                },
            },
        )
        .filter_map(get_optimal_path)
        .inspect(|x| {
            dbg!(x);
        })
        .map(|(r#as, bs)| r#as * 3 + bs)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(DATA1), 480.to_string());
    }
}
