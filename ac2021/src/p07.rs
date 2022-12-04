fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn total_alignment_cost<F: Fn(u32, u32) -> u32>(
    positions: &[u32],
    target: u32,
    cost_function: &F,
) -> u32 {
    positions
        .iter()
        .map(|current| cost_function(*current, target))
        .sum()
}

fn min_alignment_cost<F: Fn(u32, u32) -> u32>(positions: &[u32], cost_function: &F) -> u32 {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    (min..=max)
        .map(|target| total_alignment_cost(positions, target, cost_function))
        .min()
        .unwrap()
}

fn linear_cost(current: u32, target: u32) -> u32 {
    current.abs_diff(target)
}

pub fn part1(input: &str) -> u32 {
    let data = parse_input(input);
    min_alignment_cost(&data, &linear_cost)
}

fn incrementing_cost(current: u32, target: u32) -> u32 {
    let dist = current.abs_diff(target);
    dist * (dist+1) / 2
}

pub fn part2(input: &str) -> u32 {
    let data = parse_input(input);
    min_alignment_cost(&data, &incrementing_cost)
}

#[cfg(test)]
mod test {
    use super::*;

    const DATA: &str = r"16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 168);
    }
}
