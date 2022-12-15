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
}
