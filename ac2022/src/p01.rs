pub fn part1(input: &str) -> String {
    process_input(input)
        .iter()
        .max()
        .expect("empty input")
        .to_string()
}

fn process_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.parse::<u32>().unwrap_or_default())
                .collect()
        })
        .map(|sublist: Vec<u32>| sublist.iter().sum())
        .collect()
}

pub fn part2(input: &str) -> String {
    let mut sums: Vec<u32> = process_input(input);
    sums.sort_unstable();
    sums.iter().rev().take(3).sum::<u32>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "24000");
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), "45000");
    }
}
