fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn part1(input: &str) -> String {
    let mut data = parse_input(input);
    for _ in 0..80 {
        let zeroes = bytecount::count(&data, 0);
        data.resize(data.len() + zeroes, 9);
        for val in &mut data {
            *val = match *val {
                0 => 6,
                x => x - 1,
            }
        }
    }
    data.len().to_string()
}

pub fn part2(input: &str) -> String {
    let data = parse_input(input);
    let mut brackets: Vec<u128> = (0..9).map(|x| bytecount::count(&data, x) as u128).collect();
    for _ in 0..256 {
        let zeroes = brackets[0];
        for i in 0..brackets.len() - 1 {
            brackets[i] = brackets[i + 1];
        }
        brackets[8] = zeroes;
        brackets[6] += zeroes;
    }
    brackets.iter().sum::<u128>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "5934");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), "26984457539");
    }
}
