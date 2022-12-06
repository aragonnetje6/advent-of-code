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

fn main() {
    wrapper(2021, 1, 1, p01::part1);
    wrapper(2021, 1, 2, p01::part2);
    wrapper(2021, 2, 1, p02::part1);
    wrapper(2021, 2, 2, p02::part2);
    wrapper(2021, 3, 1, p03::part1);
    wrapper(2021, 3, 2, p03::part2);
    wrapper(2021, 4, 1, p04::part1);
    wrapper(2021, 4, 2, p04::part2);
    wrapper(2021, 5, 1, p05::part1);
    wrapper(2021, 5, 2, p05::part2);
    wrapper(2021, 6, 1, p06::part1);
    wrapper(2021, 6, 2, p06::part2);
    wrapper(2021, 7, 1, p07::part1);
    wrapper(2021, 7, 2, p07::part2);
    wrapper(2021, 8, 1, p08::part1);
    wrapper(2021, 8, 2, p08::part2);
    wrapper(2021, 9, 1, p09::part1);
    wrapper(2021, 9, 2, p09::part2);
    wrapper(2021, 10, 1, p10::part1);
    wrapper(2021, 10, 2, p10::part2);
    wrapper(2021, 11, 1, p11::part1);
    wrapper(2021, 11, 2, p11::part2);
    wrapper(2021, 12, 1, p12::part1);
    wrapper(2021, 12, 2, p12::part2);
}
