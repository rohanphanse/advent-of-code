use std::fs;

pub fn test() {
    println!("Day 3:");
}

fn get_binary_data() -> Vec<Vec<char>> {
    let contents = fs::read_to_string("./data/binary.txt")
        .expect("Something went wrong with reading the file");
    let binary_strings: Vec<&str> = contents.split("\n").collect();
    let binary_data: Vec<Vec<char>> = binary_strings.iter().map(|s| s.chars().collect()).collect();
    return binary_data;
}

fn calculate_power_consumption() {
    let binary_data = get_binary_data();
    let rows = binary_data.len();
    let columns = binary_data[0].len();
    let mut count_ones = 0;
    let mut gamma_binary: Vec<char> = Vec::with_capacity(columns);
    let mut epsilon_binary: Vec<char> = Vec::with_capacity(columns);
    for c in 0..columns {
        for r in 0..rows {
            if binary_data[r][c] == '1' { count_ones += 1};
        }
        if count_ones > columns / 2 {
            gamma_binary.push('1');
            epsilon_binary.push('0');
        } else {
            gamma_binary.push('0');
            epsilon_binary.push('1');
        }
    }
}