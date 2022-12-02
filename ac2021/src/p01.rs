pub fn part1(input: &str) -> usize {
    let data: Vec<_> = input
        .split('\n')
        .filter_map(|x| str::parse::<u32>(x).ok())
        .collect();
    let increases = data
        .iter()
        .skip(1)
        .zip(data.iter())
        .filter(|(later, earlier)| later > earlier)
        .count();
    increases
}

pub fn part2(input: &str) -> usize {
    let data: Vec<_> = input
        .split('\n')
        .filter_map(|x| str::parse::<u32>(x).ok())
        .collect();
    let increases = data
        .iter()
        .skip(3)
        .zip(data.iter())
        .filter(|(later, earlier)| later > earlier)
        .count();
    increases
}
