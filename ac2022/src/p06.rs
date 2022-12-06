use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    input
        .chars()
        .enumerate()
        .find(|(i, _)| {
            input
                .chars()
                .skip(*i)
                .take(4)
                .collect::<HashSet<char>>()
                .len()
                == 4
        })
        .map(|(i, _)| i)
        .unwrap()
        + 4
}

pub fn part2(input: &str) -> usize {
    input
        .chars()
        .enumerate()
        .find(|(i, _)| {
            input
                .chars()
                .skip(*i)
                .take(14)
                .collect::<HashSet<char>>()
                .len()
                == 14
        })
        .map(|(i, _)| i)
        .unwrap()
        + 14
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA1: &str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const DATA2: &str = r"bvwbjplbgvbhsrlpgdmjqwftvncz";
    const DATA3: &str = r"nppdvjthqldpwncqszvftbrmjlhg";
    const DATA4: &str = r"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const DATA5: &str = r"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 7);
        assert_eq!(part1(DATA2), 5);
        assert_eq!(part1(DATA3), 6);
        assert_eq!(part1(DATA4), 10);
        assert_eq!(part1(DATA5), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 19);
        assert_eq!(part2(DATA2), 23);
        assert_eq!(part2(DATA3), 23);
        assert_eq!(part2(DATA4), 29);
        assert_eq!(part2(DATA5), 26);
    }
}
