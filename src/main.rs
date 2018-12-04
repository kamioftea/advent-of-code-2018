extern crate itertools;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate chrono;

mod helpers;
mod day1;
mod day2;
mod day3;
mod day4;

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
        Ok(3) => {
            let strings = helpers::read_lines("resources/day3.txt").expect("Failed to load file: day3");
            let claims = day3::parse_claims(&strings);
            println!("Conflicted area is: {}", day3::get_conflicted_area(&claims));
            println!("Unique claim id is: {:?}", day3::get_unique_claim_id(&claims));
        }
        Ok(4) => {
            let strings = helpers::read_lines("resources/day4.txt").expect("Failed to load file: day4");
            let log_entries = day4::parse_logs(&strings);

            let (id1, minute1) = day4::part_1(&log_entries);
            println!("Guard ID x Sleepiest Minute is: {} x {} = {}", id1, minute1, id1 * (minute1 as i32));

            let (id2, minute2) = day4::part_2(&log_entries);
            println!("Guard ID x Sleepiest Minute is: {} x {} = {}", id2, minute1, id2 * (minute2 as i32));
        }
        _ => println!("Input was not a valid day '{}'", buffer)
    }
}
