// Day 7: https://adventofcode.com/2021/day/7

use std::fs;

pub fn test() {
    println!("Day 7:");
    println!("Least fuel cost: {}", calculate_least_fuel_cost());
    println!("Least fuel cost with triangular rate: {}", calculate_least_fuel_cost_with_triangular_rate());
}

fn get_crab_positions() -> Vec<i32> {
    let contents = fs::read_to_string("./data/crab-positions.txt")
        .expect("Something went wrong while reading file 😔");
    let strings: Vec<&str> = contents.split(",").collect();
    let numbers: Vec<i32> = strings.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    return numbers;
}

fn calculate_least_fuel_cost() -> i32 {
    let crab_positions = get_crab_positions();
    let mut max_position = crab_positions[0];
    for p in &crab_positions {
        if *p > max_position { max_position = *p; }
    }
    let mut least_fuel_cost = i32::MAX;
    for final_position in 0..max_position {
        let mut fuel_cost = 0;
        for p in &crab_positions {
            fuel_cost += i32::abs(*p - final_position);
        }
        if fuel_cost < least_fuel_cost {
            least_fuel_cost = fuel_cost;
        }
        // println!("{fuel_cost} {least_fuel_cost}");
    }
    return least_fuel_cost;    
}

fn calculate_least_fuel_cost_with_triangular_rate() -> i32 {
    let crab_positions = get_crab_positions();
    let mut max_position = crab_positions[0];
    for p in &crab_positions {
        if *p > max_position { max_position = *p; }
    }
    let mut least_fuel_cost = i32::MAX;
    let triangular_numbers = get_triangular_numbers(max_position);
    for final_position in 0..max_position {
        let mut fuel_cost = 0;
        for p in &crab_positions {
            fuel_cost += triangular_numbers[i32::abs(*p - final_position) as usize];
        }
        if fuel_cost < least_fuel_cost {
            least_fuel_cost = fuel_cost;
        }
        // println!("{fuel_cost} {least_fuel_cost}");
    }
    return least_fuel_cost;            
}

fn get_triangular_numbers(up_to: i32) -> Vec<i32> {
    let mut triangular_numbers: Vec<i32> = Vec::with_capacity((up_to + 1) as usize);
    let mut number = 0;
    for n in 0..=up_to {
        number += n;
        triangular_numbers.push(number);
    }
    return triangular_numbers;
}