use std::fmt::Display;
use std::fs;

/// Wrapper of Advent of Code challenges
///
/// # Arguments
///
/// * `year`:
/// * `index`:
/// * `part`:
/// * `func`:
///
/// returns: ()
///
/// # Panics
/// If no input file is present
pub fn wrapper<F: FnOnce(&str) -> T, T: Display>(year: u16, index: usize, part: u8, func: F) {
    println!(
        "Day {index} part {part}: {}",
        func(&fs::read_to_string(format!("./ac{year}/input/p{index:0>2}")).unwrap())
    );
}
