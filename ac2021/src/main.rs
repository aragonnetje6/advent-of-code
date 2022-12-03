use std::fmt::Display;
use std::fs;

mod p01;
mod p02;
mod p03;
mod p04;

fn main() {
    wrapper(1, 1, p01::part1);
    wrapper(1, 2, p01::part2);
    wrapper(2, 1, p02::part1);
    wrapper(2, 2, p02::part2);
    wrapper(3, 1, p03::part1);
    wrapper(3, 2, p03::part2);
    wrapper(4, 1, p04::part1);
    wrapper(4, 2, p04::part2);
}

fn wrapper<F: FnOnce(&str) -> T, T: Display>(index: u32, part: u32, func: F) {
    println!("Day {index} part {part}: {}", func(&fs::read_to_string(format!("./ac2021/input/p{index:0>2}")).unwrap()))
}