use std::fs;

pub fn part_1_and_2() {
    let input = fs::read_to_string("./input/p01").unwrap();
    let data: Vec<Vec<u32>> = input
        .split("\n\n")
        .map(|x| x.split("\n").map(|x| x.parse().unwrap_or_default()).collect())
        .collect();
    let mut sums: Vec<u32> = data
        .iter()
        .map(|sublist| sublist.iter().sum::<u32>())
        .collect();
    sums.sort();
    sums.reverse();
    println!("Maximum: {}", sums[0]);
    println!("Top three total: {}", sums[0..3].iter().sum::<u32>())
}