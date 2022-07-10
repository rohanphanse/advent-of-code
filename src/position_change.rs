// Day 2 of Advent of Code: https://adventofcode.com/2021/day/2

use std::fs;

pub fn test() {
    println!("Day 2:");
    let (horizontal_position, depth) = calculate_position_change();
    println!("Position change: ({}, {}) -> {}", horizontal_position, depth, horizontal_position * depth);
    let (aim_horizontal_position, aim_depth) = calculate_aim_position_change();
    println!("Aim position change: ({}, {}) -> {}", aim_horizontal_position, aim_depth, aim_horizontal_position * aim_depth);
    
}

fn get_movements() -> Vec<(String, i32)> {
    let contents = fs::read_to_string("./data/movements.txt")
        .expect("Something went wrong with reading the file");
    let movement_strings: Vec<&str> = contents.split("\n").collect();
    let movements_split: Vec<Vec<&str>> = movement_strings.iter().map(|s| s.split(" ").collect()).collect();
    let movements: Vec<(String, i32)> = movements_split.iter().map(|v| (v[0].to_string(), v[1].parse::<i32>().unwrap())).collect();
    return movements;
}

fn calculate_position_change() -> (i32, i32) {
    let movements = get_movements();
    let mut horizontal_position = 0;
    let mut depth = 0;
    for (command, distance) in &movements {
        if command == "forward" {
            horizontal_position += distance;
        }
        if command == "up" {
            depth -= distance;
        }
        if command == "down" {
            depth += distance;
        }
    }
    return (horizontal_position, depth);
}

fn calculate_aim_position_change() -> (i32, i32) {
    let movements = get_movements();
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (command, distance) in &movements {
        if command == "forward" {
            horizontal_position += distance;
            depth += aim * distance;
        }
        if command == "up" {
            aim -= distance;
        }
        if command == "down" {
            aim += distance;
        }
    }
    return (horizontal_position, depth);
}