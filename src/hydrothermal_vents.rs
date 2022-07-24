// Day 5: https://adventofcode.com/2021/day/5

use std::fs;
use std::cmp::{ min, max };

pub fn test() {
    println!("Day 5:");
    println!("Line overlaps: {}", count_line_overlaps());
    println!("Line overlaps with diagonals: {}", count_line_overlaps_with_diagonals());
}

fn get_vent_data() -> Vec<((i32, i32), (i32, i32))>  {
    let contents = fs::read_to_string("./data/vents.txt")
        .expect("Something went wrong with reading the file :(");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut vent_data: Vec<((i32, i32), (i32, i32))> = Vec::with_capacity(lines.len());
    for line in lines {
        let split_arrow: Vec<&str> = line.split("->").collect();
        let split_commas: Vec<Vec<&str>> = split_arrow.iter().map(|s| s.split(",").collect()).collect();
        // Parse numbers
        let n: Vec<Vec<i32>> = split_commas.iter().map(|v| v.iter().map(|s| s.trim().parse::<i32>().unwrap()).collect()).collect();
        vent_data.push(((n[0][0], n[0][1]), (n[1][0], n[1][1])));
    }
    return vent_data;
}

fn count_line_overlaps() -> i32 {
    let vent_data = get_vent_data();
    // Determine largest x and y values
    let mut largest_x = 0;
    let mut largest_y = 0;
    for ((x1, y1), (x2, y2)) in &vent_data {
        if *x1 > largest_x { largest_x = *x1; }
        if *x2 > largest_x { largest_x = *x2; }
        if *y1 > largest_y { largest_y = *y1; }
        if *y2 > largest_y { largest_y = *y2; }
    }
    let mut ocean_floor: Vec<Vec<i32>> = Vec::with_capacity((largest_y + 1) as usize);
    // Create ocean floor matrix
    for r in 0..((largest_y + 1) as usize) {
        ocean_floor.push(Vec::with_capacity((largest_x + 1) as usize));
        for _ in 0..(largest_x + 1) {
            ocean_floor[r].push(0);
        }
    }
    // Add vent lines
    for ((x1, y1), (x2, y2)) in &vent_data {
        // Vertical lines
        if *x1 == *x2 {
            for y in (min(*y1, *y2) as usize)..=(max(*y1, *y2) as usize) {
                ocean_floor[y][*x1 as usize] += 1;
            }
        }
        // Horizontal lines
        if *y1 == *y2 {
            for x in (min(*x1, *x2) as usize)..=(max(*x1, *x2) as usize) {
                ocean_floor[*y1 as usize][x] += 1;
            }
        }
    }
    // print_ocean_floor(&ocean_floor);
    // Count line overlaps (where value >= 2)
    let mut overlaps = 0;
    for r in 0..ocean_floor.len() {
        for c in 0..ocean_floor[r].len() {
            if ocean_floor[r][c] >= 2 {
                overlaps += 1;
            }
        }
    }
    return overlaps;
}

fn _print_ocean_floor(ocean_floor: &Vec<Vec<i32>>) {
    for r in 0..ocean_floor.len() {
        for c in 0..ocean_floor[r].len() {
            let value = ocean_floor[r][c];
            if value == 0 {
                print!(". ");
            } else {
                print!("{} ", value);
            }
        }
        println!();
    }
}

fn count_line_overlaps_with_diagonals() -> i32 {
    let vent_data = get_vent_data();
    // Determine largest x and y values
    let mut largest_x = 0;
    let mut largest_y = 0;
    for ((x1, y1), (x2, y2)) in &vent_data {
        if *x1 > largest_x { largest_x = *x1; }
        if *x2 > largest_x { largest_x = *x2; }
        if *y1 > largest_y { largest_y = *y1; }
        if *y2 > largest_y { largest_y = *y2; }
    }
    let mut ocean_floor: Vec<Vec<i32>> = Vec::with_capacity((largest_y + 1) as usize);
    // Create ocean floor matrix
    for r in 0..((largest_y + 1) as usize) {
        ocean_floor.push(Vec::with_capacity((largest_x + 1) as usize));
        for _ in 0..(largest_x + 1) {
            ocean_floor[r].push(0);
        }
    }
    // Add vent lines
    for ((x1, y1), (x2, y2)) in &vent_data {
        // Vertical lines
        if *x1 == *x2 {
            for y in (min(*y1, *y2) as usize)..=(max(*y1, *y2) as usize) {
                ocean_floor[y][*x1 as usize] += 1;
            }
        // Horizontal lines
        } else if *y1 == *y2 {
            for x in (min(*x1, *x2) as usize)..=(max(*x1, *x2) as usize) {
                ocean_floor[*y1 as usize][x] += 1;
            }
        // Diagonal lines
        } else {
            let x_start = min(*x1, *x2);
            let x_end = max(*x1, *x2);
            let y_start = if x_start == *x1 { *y1 } else { *y2 };
            let y_end = if x_end == *x1 { *y1 } else { *y2 };
            let y_direction = if (y_end - y_start) > 0 { 1 } else { -1 };
            for i in 0..=(x_end - x_start) {
                ocean_floor[(y_start + i * y_direction) as usize][(x_start + i) as usize] += 1;
            }
        }
        
    }
    // print_ocean_floor(&ocean_floor);
    // Count line overlaps (where value >= 2)
    let mut overlaps = 0;
    for r in 0..ocean_floor.len() {
        for c in 0..ocean_floor[r].len() {
            if ocean_floor[r][c] >= 2 {
                overlaps += 1;
            }
        }
    }
    return overlaps;
}