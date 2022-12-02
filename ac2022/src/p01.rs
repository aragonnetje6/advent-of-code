use std::iter::Map;
use std::str::Split;

pub fn part_1(input: &str) -> u32 {
    process_input(input).max().unwrap()
}

fn process_input(input: &str) -> Map<Map<Split<&str>, fn(_) -> _>, fn(Vec<u32>)> {
    input
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|x| x.parse().unwrap_or_default())
                .collect()
        })
        .map(|sublist: Vec<u32>| sublist.iter().sum())
}

pub fn part_2(input: &str) -> u32 {
    let mut sums: Vec<u32> = process_input(input).collect();
    sums.sort();
    sums.iter().rev().take(3).sum()
}
