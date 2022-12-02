pub fn part_1(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|x| x.parse().unwrap_or_default())
                .collect()
        })
        .map(|sublist: Vec<u32>| sublist.iter().sum())
        .max()
        .unwrap()
}

pub fn part_2(input: &str) -> u32 {
    let mut sums: Vec<u32> = input
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|x| x.parse().unwrap_or_default())
                .collect()
        })
        .map(|sublist: Vec<u32>| sublist.iter().sum())
        .collect();
    sums.sort();
    sums.iter().rev().take(3).sum()
}
