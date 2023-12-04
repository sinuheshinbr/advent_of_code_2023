use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let mut total = 0;

    for line in lines {
        let split_line: Vec<&str> = line.split(":").collect();
        let values: Vec<&str> = split_line[1].split("|").collect();
        let winning_numbers_str: Vec<&str> = values[0].trim().split_whitespace().collect();
        let owned_numbers_str: Vec<&str> = values[1].trim().split_whitespace().collect();
        let mut winning_numbers = HashMap::new();
        let mut matches: u32 = 0;

        for winning in winning_numbers_str {
            winning_numbers.insert(winning.parse::<i32>().unwrap(), ());
        }

        for owned_str in owned_numbers_str {
            if winning_numbers.contains_key(&owned_str.parse::<i32>().unwrap()) {
                matches += 1
            }
        }

        let row_points = if matches == 0 {
            0
        } else {
            (2_i32).pow(matches - 1)
        };
        total += row_points;
    }
    println!("total: {}", total)
}
