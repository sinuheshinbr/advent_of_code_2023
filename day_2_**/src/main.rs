use std::{error::Error, fs};

fn main() {
    let _ = part_one();
    let _ = part_two();
}

fn part_two() -> Result<(), Box<dyn Error>> {
    let file: String = fs::read_to_string("input.txt")?.parse()?;
    let lines: Vec<&str> = file.lines().collect();
    let mut sum = 0;

    for line in lines {
        let mut iter = line.split(":");
        let _ = iter.next().unwrap().split(" ").nth(1).unwrap();
        let game_plays: Vec<&str> = iter.next().unwrap().split(";").collect();
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for play in game_plays {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let balls: Vec<&str> = play.split(",").collect();
            for ball in balls {
                let mut values = ball.trim().split(" ");
                let qty = values.next().unwrap();
                let color = values.next().unwrap();

                match color {
                    "red" => red += qty.parse::<i32>().unwrap(),
                    "green" => green += qty.parse::<i32>().unwrap(),
                    "blue" => blue += qty.parse::<i32>().unwrap(),
                    _ => unreachable!(),
                }
            }

            if red > min_red {
                min_red = red
            }
            if green > min_green {
                min_green = green
            }
            if blue > min_blue {
                min_blue = blue
            }
        }

        let pow = min_red * min_green * min_blue;

        sum += pow;
    }
    println!("sum: {}", sum);
    Ok(())
}

fn part_one() -> Result<(), Box<dyn Error>> {
    let file: String = fs::read_to_string("input.txt")?.parse()?;
    let lines: Vec<&str> = file.lines().collect();
    let mut sum = 0;

    for line in lines {
        let mut iter = line.split(":");
        let game_id = iter.next().unwrap().split(" ").nth(1).unwrap();
        let game_plays: Vec<&str> = iter.next().unwrap().split(";").collect();
        let mut possible = true;

        for play in game_plays {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let balls: Vec<&str> = play.split(",").collect();
            for ball in balls {
                let mut values = ball.trim().split(" ");
                let qty = values.next().unwrap();
                let color = values.next().unwrap();

                match color {
                    "red" => red += qty.parse::<i32>().unwrap(),
                    "green" => green += qty.parse::<i32>().unwrap(),
                    "blue" => blue += qty.parse::<i32>().unwrap(),
                    _ => unreachable!(),
                }
            }

            if red > 12 || green > 13 || blue > 14 {
                possible = false
            }
        }

        if possible {
            sum += game_id.parse::<i32>().unwrap()
        }
    }
    println!("sum: {}", sum);
    Ok(())
}
