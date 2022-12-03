use lib::wrapper;

mod p01;
mod p02;

fn main() {
    wrapper(1, 1, p01::part1);
    wrapper(1, 2, p01::part2);
    wrapper(2, 1, p02::part1);
    wrapper(2, 2, p02::part2);
}
