use nom::{
    character::complete, combinator::map, multi::separated_list1, sequence::separated_pair, IResult,
};

fn parse_line(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(complete::u32, complete::multispace1, complete::u32)(input)
}

fn parse_file(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    map(separated_list1(complete::newline, parse_line), |v| {
        v.into_iter().unzip()
    })(input)
}

pub fn part1(input: &str) -> String {
    let (_, (mut list1, mut list2)) = parse_file(input).expect("parsing failed");
    list1.sort_unstable();
    list2.sort_unstable();
    list1
        .into_iter()
        .zip(list2)
        .map(|(x, y)| x.abs_diff(y))
        .sum::<u32>()
        .to_string()
}

fn count<T: Eq>(x: T, list: &[T]) -> usize {
    list.iter().filter(|item| **item == x).count()
}

pub fn part2(input: &str) -> String {
    let (_, (list1, list2)) = parse_file(input).expect("parsing failed");
    list1
        .into_iter()
        .map(|x| count(x, &list2) * x as usize)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 11.to_string());
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(DATA1), 32.to_string());
    }
}
