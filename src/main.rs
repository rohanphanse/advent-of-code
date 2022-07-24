mod depth_increases;
mod position_change;
mod binary_diagnostics;
mod hydrothermal_vents;

fn main() {
    println!("Advent of Code 2021!");
    depth_increases::test();
    position_change::test();
    binary_diagnostics::test();
    hydrothermal_vents::test();
}