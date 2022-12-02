use std::fs;

pub fn part_1() {
    let max: u32 = fs::read_to_string("./ac2022/input/p01")
        .unwrap()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|x| x.parse().unwrap_or_default())
                .collect()
        })
        .map(|sublist: Vec<u32>| sublist.iter().sum())
        .max()
        .unwrap();
    println!("Maximum: {}", max);
}

pub fn part_2() {
    let mut sums: Vec<u32> = fs::read_to_string("./ac2022/input/p01")
        .unwrap()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .map(|x| x.parse().unwrap_or_default())
                .collect()
        })
        .map(|sublist: Vec<u32>| sublist.iter().sum())
        .collect();
    sums.sort();
    let total: u32 = sums.iter().rev().take(3).sum();
    println!("Top three total: {}", total)
}
