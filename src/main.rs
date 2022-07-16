mod depth_increases;
mod position_change;
mod binary_diagnostics;
mod hydrothermal_vents;

fn main() {
    depth_increases::test();
    position_change::test();
    binary_diagnostics::test();
    hydrothermal_vents::test();
    println!("Hello, world!");
}