use lib::wrapper;

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
