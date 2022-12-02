use std::fmt::Display;
use std::fs;

mod p01;

fn main() {
    wrapper(1, 1, p01::part_1);
    wrapper(1, 2, p01::part_2);
}

fn wrapper<F: FnOnce(&str) -> T, T: Display>(index: u32, part: u32, func: F) {
    println!("Day {index} part {part}: {}", func(&fs::read_to_string(format!("./ac2022/input/p{index:0>2}")).unwrap()))
}
