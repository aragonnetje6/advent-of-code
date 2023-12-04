pub fn part1(input: &str) -> String {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let mut line_iter = line.split(',').map(|elf| {
                let mut iter = elf.split('-');
                (
                    iter.next()
                        .and_then(|x| x.parse::<u32>().ok())
                        .expect("parsing failure"),
                    iter.next()
                        .and_then(|x| x.parse::<u32>().ok())
                        .expect("parsing failure"),
                )
            });
            (
                line_iter.next().expect("parsing failure"),
                line_iter.next().expect("parsing failure"),
            )
        })
        .filter(|((elf1_low, elf1_high), (elf2_low, elf2_high))| {
            (elf1_low <= elf2_low && elf1_high >= elf2_high)
                || (elf2_low <= elf1_low && elf2_high >= elf1_high)
        })
        .count()
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let mut line_iter = line.split(',').map(|elf| {
                let mut iter = elf.split('-');
                (
                    iter.next()
                        .and_then(|x| x.parse::<u32>().ok())
                        .expect("parsing failure"),
                    iter.next()
                        .and_then(|x| x.parse::<u32>().ok())
                        .expect("parsing failure"),
                )
            });
            (
                line_iter.next().expect("parsing failure"),
                line_iter.next().expect("parsing failure"),
            )
        })
        .filter(|((elf1_low, elf1_high), (elf2_low, elf2_high))| {
            (elf1_low <= elf2_low && elf1_high >= elf2_high)
                || (elf2_low <= elf1_low && elf2_high >= elf1_high)
                || (elf1_low <= elf2_low && elf1_high >= elf2_low)
                || (elf1_low <= elf2_high && elf1_high >= elf2_high)
        })
        .count()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "2");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), "4");
    }
}
