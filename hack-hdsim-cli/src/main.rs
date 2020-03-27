extern crate hack_hdsim_lib;

use hack_hdsim_lib::And2;

fn main() {
    let and2 = And2::new(true, false);
    println!(
        "Created And2 gate, a: {}, b: {}, out: {}",
        and2.in_a(),
        and2.in_b(),
        and2.out()
    )
}
