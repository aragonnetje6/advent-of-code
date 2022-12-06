use lib::wrapper;

mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;

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
}
