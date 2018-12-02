extern crate itertools;

mod helpers;
mod day1;
mod day2;

use std::io::{stdin,stdout,Write};

fn main() {
    let mut buffer = String::new();
    print!("Please enter the day: ");
    let _= stdout().flush();
    stdin().read_line(&mut buffer).expect("Failed to read input");

    match buffer.trim().parse::<i32>() {
        Ok(1) => {
            let strings = helpers::read_lines("resources/day1.txt").expect("Failed to load file: day1");

            println!("Resulting frequency is: {}", day1::part_1(&strings));
            println!("First repeat is: {}", day1::part_2(&strings));
        }
        Ok(2) => {
            let strings = helpers::read_lines("resources/day2.txt").expect("Failed to load file: day2");
            println!("Checksum is: {}", day2::check_sum(&strings));
            println!("Checksum is: {}", day2::find_matching(&strings).unwrap());
        }
        _ => println!("Input was not a valid day '{}'", buffer)
    }
}
