use std::fs;

pub fn test() {
    println!("Day 1:");
    println!("Depth increases: {}", count_depth_increases());
    println!("Depth sum increases: {}", count_depth_sum_increases());
}

fn get_depths() -> Vec<i32> {
    let contents = fs::read_to_string("./data/depths.txt")
        .expect("Something went wrong with reading the file");
    let depth_strings: Vec<&str> = contents.split("\n").collect();
    let depths: Vec<i32> = depth_strings.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    return depths;
}

fn count_depth_increases() -> i32 {
    let depths = get_depths();
    let mut count = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            count += 1;
        }
    }
    return count;
}

fn count_depth_sum_increases() -> i32 {
    let depths = get_depths();
    let mut count = 0;
    for i in 3..depths.len() {
        let previous_sum = depths[i - 3] + depths[i - 2] + depths[i - 1];
        let current_sum = depths[i - 2] + depths[i - 1] + depths[i];
        if current_sum > previous_sum {
            count += 1;
        }
    }
    return count;
}