use std::fs;

pub fn part_1_and_2() {
    let mut sums: Vec<u32> = fs::read_to_string("./input/p01")
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
    sums.reverse();
    println!("Maximum: {}", sums[0]);
    println!("Top three total: {}", sums[0..3].iter().sum::<u32>())
}
