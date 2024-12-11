use nom::{character::complete, multi::separated_list1, IResult};

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(
        complete::newline,
        separated_list1(complete::space1, complete::u32),
    )(input)
}

fn is_safe(report: &[u32]) -> bool {
    report.windows(2).all(|window| {
        window[0] > window[1]
            && window[0].abs_diff(window[1]) > 0
            && window[0].abs_diff(window[1]) < 4
    }) || report.windows(2).all(|window| {
        window[0] < window[1]
            && window[0].abs_diff(window[1]) > 0
            && window[0].abs_diff(window[1]) < 4
    })
}

pub fn part1(input: &str) -> String {
    let (_, reports) = parse(input).expect("parsing failure");
    reports
        .into_iter()
        .filter(|x| is_safe(x))
        .count()
        .to_string()
}

fn is_safe_dampened(report: &[u32]) -> bool {
    is_safe(report)
        || (0..report.len()).any(|i| {
            let mut shortened = Vec::from(report);
            shortened.remove(i);
            is_safe(&shortened)
        })
}

pub fn part2(input: &str) -> String {
    let (_, reports) = parse(input).expect("parsing failure");
    reports
        .into_iter()
        .filter(|x| is_safe_dampened(x))
        .count()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 2.to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 4.to_string());
    }
}
