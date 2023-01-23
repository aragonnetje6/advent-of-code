extern crate core;

use lib::wrapper;

mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;
mod p10;
mod p11;
mod p12;
mod p13;
mod p14;
mod p15;
// mod p16;

type Solution = fn(&str) -> String;

fn main() {
    let year = 2022;
    let functions: Vec<(Solution, Solution)> = vec![
        (p01::part1, p01::part2),
        (p02::part1, p02::part2),
        (p03::part1, p03::part2),
        (p04::part1, p04::part2),
        (p05::part1, p05::part2),
        (p06::part1, p06::part2),
        (p07::part1, p07::part2),
        (p08::part1, p08::part2),
        (p09::part1, p09::part2),
        (p10::part1, p10::part2),
        (p11::part1, p11::part2),
        (p12::part1, p12::part2),
        (p13::part1, p13::part2),
        (p14::part1, p14::part2),
        (p15::part1, p15::part2),
    ];
    for (index, (part1, part2)) in functions.iter().enumerate() {
        wrapper(year, index + 1, 1, part1);
        wrapper(year, index + 1, 2, part2);
    }
}
