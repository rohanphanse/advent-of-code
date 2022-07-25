// Day 6: https://adventofcode.com/2021/day/6

use std::fs;

pub fn test() {
    println!("Day 6:");
    println!("Lanternfish population after 80 days: {}", simulate(80));
    println!("# of lanternfish after 256 days: {}", simulate_efficiently(256));
}

fn get_initial_population() -> Vec<i32> {
    let contents = fs::read_to_string("./data/lantern-fish.txt")
        .expect("Something went wrong while reading file 😔");
    let strings: Vec<&str> = contents.split(",").collect();
    let numbers: Vec<i32> = strings.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    return numbers;
}

fn simulate(days: i32) -> i32 {
    let mut lantern_fish = get_initial_population();
    for _ in 0..days {
        for i in 0..lantern_fish.len() {
            if lantern_fish[i] == 0 {
                lantern_fish[i] = 6;
                lantern_fish.push(8);
            } else {
                lantern_fish[i] -= 1;
            }
        }
    }
    return lantern_fish.len() as i32;
}

fn simulate_efficiently(days: i128) -> i32 {
    let mut lantern_fish = get_initial_population();
    return 0;
}