extern crate core;

use lib::solution;

mod p01;
mod p02;
mod p03;
mod p04;

#[allow(clippy::zero_prefixed_literal)]
fn main() {
    solution!(01, 1);
    solution!(01, 2);
    solution!(02, 1);
    solution!(02, 2);
    solution!(03, 1);
    solution!(03, 2);
    solution!(04, 1);
    solution!(04, 2);
}
