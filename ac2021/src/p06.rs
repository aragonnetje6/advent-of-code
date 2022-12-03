fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}



pub fn part1(input: &str) -> usize {
    let mut data = parse_input(input);
    for _ in 0..80 {
        let zeroes = data.iter().filter(|val| val == &&0).count();
        data.resize(data.len() + zeroes, 9);
        data.iter_mut().for_each(|val| *val = match *val {0=>6, x=> { x - 1 }});
    }
    data.len()
}

pub fn part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 5934)
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(DATA), 1589590444365)
    }
}
