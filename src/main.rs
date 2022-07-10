mod depth_increases;
mod position_change;
mod power_consumption;

fn main() {
    depth_increases::test();
    position_change::test();
    power_consumption::test();
    println!("Hello, world!");
}