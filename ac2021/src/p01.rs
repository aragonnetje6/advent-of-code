pub fn part1(input: &str) -> String {
    let data: Vec<_> = input
        .split('\n')
        .filter_map(|x| str::parse::<u32>(x).ok())
        .collect();
    data
        .iter()
        .skip(1)
        .zip(data.iter())
        .filter(|(later, earlier)| later > earlier)
        .count().to_string()
}

pub fn part2(input: &str) -> String {
    let data: Vec<_> = input
        .split('\n')
        .filter_map(|x| str::parse::<u32>(x).ok())
        .collect();
    data
        .iter()
        .skip(3)
        .zip(data.iter())
        .filter(|(later, earlier)| later > earlier)
        .count().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n"),
            "7"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n"),
            "5"
        );
    }
}
