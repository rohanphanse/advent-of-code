use std::fs;

pub fn test() {
    println!("Day 3:");
    println!("Power consumption: {}", calculate_power_consumption());
    println!("Life support rating: {}", calculate_life_support_rating());
}

fn get_binary_data() -> Vec<Vec<char>> {
    let contents = fs::read_to_string("./data/binary.txt")
        .expect("Something went wrong with reading the file");
    let binary_strings: Vec<&str> = contents.split("\n").collect();
    let binary_data: Vec<Vec<char>> = binary_strings.iter().map(|s| s.chars().collect()).collect();
    return binary_data;
}

fn calculate_power_consumption() -> i32 {
    let binary_data = get_binary_data();
    let rows = binary_data.len();
    let columns = binary_data[0].len();
    let mut gamma_binary: Vec<char> = Vec::with_capacity(columns);
    let mut epsilon_binary: Vec<char> = Vec::with_capacity(columns);
    for c in 0..columns {
        let mut count_ones = 0;
        for r in 0..rows {
            if binary_data[r][c] == '1' { count_ones += 1; }
        }
        if count_ones >= (rows as f64 / 2.0).ceil() as i32 {
            gamma_binary.push('1');
            epsilon_binary.push('0');
        } else {
            gamma_binary.push('0');
            epsilon_binary.push('1');
        }
    }
    let gamma_rate = i32::from_str_radix(&String::from_iter(&gamma_binary), 2).unwrap();
    // println!("{:?} {}", &gamma_binary, gamma_rate);
    let epsilon_rate = i32::from_str_radix(&String::from_iter(&epsilon_binary), 2).unwrap();
    // println!("{:?} {}", &epsilon_binary, epsilon_rate);
    return gamma_rate * epsilon_rate;
}

fn calculate_life_support_rating() -> i32 {
    let binary_data = get_binary_data();
    // Calculate oxygen rating
    let mut oxygen_candidates = binary_data.to_vec();
    let mut oxygen_rating = -1;
    for c in 0..oxygen_candidates[0].len() {
        let mut count_ones = 0;
        let rows = oxygen_candidates.len();
        for r in 0..rows {
            if oxygen_candidates[r][c] == '1' { count_ones += 1; }
        }
        let most_common_bit = if count_ones >= (rows as f64 / 2.0).ceil() as i32 { '1' } else { '0' };
        let mut new_candidates: Vec<Vec<char>> = Vec::new();
        for r in 0..rows {
            if oxygen_candidates[r][c] == most_common_bit {
                new_candidates.push(oxygen_candidates[r].to_vec());
            }
        }
        oxygen_candidates = new_candidates;
        if oxygen_candidates.len() == 1 {
            oxygen_rating = i32::from_str_radix(&String::from_iter(&oxygen_candidates[0]), 2).unwrap();
            break;
        }
    }
    assert!(oxygen_rating != -1, "Oxygen rating error");
    // Calculate CO2 rating
    let mut co2_candidates = binary_data.to_vec();
    let mut co2_rating = -1;
    for c in 0..co2_candidates[0].len() {
        let mut count_ones = 0;
        let rows = co2_candidates.len();
        for r in 0..rows {
            if co2_candidates[r][c] == '1' { count_ones += 1; }
        }
        let least_common_bit = if count_ones < (rows as f64 / 2.0).ceil() as i32 { '1' } else { '0' };
        let mut new_candidates: Vec<Vec<char>> = Vec::new();
        for r in 0..rows {
            if co2_candidates[r][c] == least_common_bit {
                new_candidates.push(co2_candidates[r].to_vec());
            }
        }
        co2_candidates = new_candidates;
        if co2_candidates.len() == 1 {
            co2_rating = i32::from_str_radix(&String::from_iter(&co2_candidates[0]), 2).unwrap();
            break;
        }
    }
    assert!(co2_rating != -1, "CO2 rating error");
    return oxygen_rating * co2_rating;
}