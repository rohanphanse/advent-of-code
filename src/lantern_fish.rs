// Day 6: https://adventofcode.com/2021/day/6

use std::fs;

pub fn test() {
    println!("Day 6:");
    println!("Lanternfish population after 80 days: {}", simulate_efficiently(80));
    println!("# of lanternfish after 256 days: {}", simulate_efficiently(256));
}

fn get_initial_population() -> Vec<i32> {
    let contents = fs::read_to_string("./data/lantern-fish.txt")
        .expect("Something went wrong while reading file 😔");
    let strings: Vec<&str> = contents.split(",").collect();
    let numbers: Vec<i32> = strings.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    return numbers;
}

fn _simulate(days: i32) -> i32 {
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

fn simulate_efficiently(days: i32) -> i128 {
    let initial_population = get_initial_population();
    let mut lantern_fish: Vec<i128> = vec![0 ; 9];
    for p in initial_population {
        lantern_fish[p as usize] += 1;
    }
    for _ in 0..days {
        // Shifts elements to left, cycles first element to end
        lantern_fish.rotate_left(1);
        lantern_fish[6] += lantern_fish[8];
    }
    return lantern_fish.iter().sum();
}