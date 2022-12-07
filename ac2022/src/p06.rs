use std::collections::HashSet;

fn find_unique_sequence(input: &str, window_size: usize) -> usize {
    input
        .as_bytes()
        .windows(window_size)
        .enumerate()
        .find(|(_, items)| items.iter().collect::<HashSet<&u8>>().len() == window_size)
        .map(|(i, _)| i)
        .unwrap()
        + window_size
}

pub fn part1(input: &str) -> usize {
    find_unique_sequence(input, 4)
}

pub fn part2(input: &str) -> usize {
    find_unique_sequence(input, 14)
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
