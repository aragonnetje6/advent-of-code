pub fn part1(input: &str) -> u32 {
    *process_input(input).iter().max().unwrap()
}

fn process_input(input: &str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|x| x.parse::<u32>().unwrap_or_default())
                .collect()
        })
        .map(|sublist: Vec<u32>| sublist.iter().sum())
        .collect()
}

pub fn part2(input: &str) -> u32 {
    let mut sums: Vec<u32> = process_input(input);
    sums.sort();
    sums.iter().rev().take(3).sum()
}
