use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::{map, value, verify},
    IResult,
};

pub fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let x: Vec<char> = line.chars().filter(char::is_ascii_digit).collect();
            x.first()
                .expect("no digit")
                .to_digit(10)
                .expect("impossible")
                * 10
                + x.last()
                    .expect("no digit")
                    .to_digit(10)
                    .expect("impossible")
        })
        .sum::<u32>()
        .to_string()
}

fn number(input: &str) -> IResult<&str, Option<u32>> {
    alt((
        map(
            alt((
                value(1, tag("one")),
                value(2, tag("two")),
                value(3, tag("three")),
                value(4, tag("four")),
                value(5, tag("five")),
                value(6, tag("six")),
                value(7, tag("seven")),
                value(8, tag("eight")),
                value(9, tag("nine")),
            )),
            Some,
        ),
        map(
            verify(anychar, |s: &char| s.is_alphanumeric()),
            |s: char| s.to_string().parse::<u32>().ok(),
        ),
    ))(input)
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            (0..line.len())
                .filter_map(|i| {
                    number(line.chars().skip(i).collect::<String>().as_str())
                        .ok()
                        .and_then(|(_, res)| res)
                })
                .collect::<Vec<u32>>()
        })
        .map(|x| x.first().unwrap() * 10 + x.last().unwrap())
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const DATA2: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 142.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA2), 281.to_string());
    }
}
