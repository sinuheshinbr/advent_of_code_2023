use std::{error::Error, fs};

fn main() {
    // let _ = part_one();
    let _ = part_two();
}

fn part_one() -> Result<(), Box<dyn Error>> {
    let file: String = fs::read_to_string("input.txt")?.parse()?;
    let lines: Vec<&str> = file.lines().collect();
    let mut total = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut left_char: Option<char> = None;
        let mut right_char: Option<char> = None;

        for char in chars {
            if char.is_numeric() {
                if let None = left_char {
                    left_char = Some(char)
                }

                right_char = Some(char)
            }
        }

        let number_string = format!("{}{}", left_char.unwrap(), right_char.unwrap());

        let line_value: i32 = number_string.parse().unwrap();

        total += line_value;
    }
    println!("part one result: {}", total);
    Ok(())
}

fn part_two() -> Result<(), Box<dyn Error>> {
    let file: String = fs::read_to_string("input.txt")?.parse()?;
    let lines: Vec<&str> = file.lines().collect();
    let mut total = 0;

    for line in lines {
        let mut left_val: Option<u32> = None;
        let mut right_val: Option<u32> = None;
        for (index, char) in line.chars().enumerate() {
            if char.is_numeric() {
                if let None = left_val {
                    left_val = Some(char.to_digit(10).unwrap())
                }

                right_val = Some(char.to_digit(10).unwrap())
            } else {
                let substring = &line[index..];
                let value: Option<u8> = match substring {
                    s if s.starts_with("one") => Some(1),
                    s if s.starts_with("two") => Some(2),
                    s if s.starts_with("three") => Some(3),
                    s if s.starts_with("four") => Some(4),
                    s if s.starts_with("five") => Some(5),
                    s if s.starts_with("six") => Some(6),
                    s if s.starts_with("seven") => Some(7),
                    s if s.starts_with("eight") => Some(8),
                    s if s.starts_with("nine") => Some(9),
                    s if s.starts_with("ten") => Some(10),
                    _ => None,
                };

                if let Some(val) = value {
                    if let None = left_val {
                        left_val = Some(val as u32)
                    }

                    right_val = Some(val as u32)
                }
            }
        }
        let number_string = format!("{}{}", left_val.unwrap(), right_val.unwrap());

        let line_value: i32 = number_string.parse().unwrap();

        total += line_value;
    }
    println!("total: {}", total);

    Ok(())
}
