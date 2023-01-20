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

fn main() {
    wrapper(2022, 1, 1, p01::part1);
    wrapper(2022, 1, 2, p01::part2);
    wrapper(2022, 2, 1, p02::part1);
    wrapper(2022, 2, 2, p02::part2);
    wrapper(2022, 3, 1, p03::part1);
    wrapper(2022, 3, 2, p03::part2);
    wrapper(2022, 4, 1, p04::part1);
    wrapper(2022, 4, 2, p04::part2);
    wrapper(2022, 5, 1, p05::part1);
    wrapper(2022, 5, 2, p05::part2);
    wrapper(2022, 6, 1, p06::part1);
    wrapper(2022, 6, 2, p06::part2);
    wrapper(2022, 7, 1, p07::part1);
    wrapper(2022, 7, 2, p07::part2);
    wrapper(2022, 8, 1, p08::part1);
    wrapper(2022, 8, 2, p08::part2);
    wrapper(2022, 9, 1, p09::part1);
    wrapper(2022, 9, 2, p09::part2);
    wrapper(2022, 10, 1, p10::part1);
    wrapper(2022, 10, 2, p10::part2);
    wrapper(2022, 11, 1, p11::part1);
    wrapper(2022, 11, 2, p11::part2);
    wrapper(2022, 12, 1, p12::part1);
    wrapper(2022, 12, 2, p12::part2);
    wrapper(2022, 13, 1, p13::part1);
    wrapper(2022, 13, 2, p13::part2);
    wrapper(2022, 14, 1, p14::part1);
    wrapper(2022, 14, 2, p14::part2);
    wrapper(2022, 15, 1, p15::part1);
    // wrapper(2022, 15, 2, p15::part2);
}
