use std::fmt::Display;
use std::fs;

pub fn wrapper<F: FnOnce(&str) -> T, T: Display>(year: u16, index: u8, part: u8, func: F) {
    println!(
        "Day {index} part {part}: {}",
        func(&fs::read_to_string(format!("./ac{year}/input/p{index:0>2}")).unwrap())
    )
}
