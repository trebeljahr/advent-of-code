#![allow(unused_parens)]

#[path = "2025/mod.rs"]
mod days_2025;

mod utils;

use inquire::Select;
use std::fs;

fn fetch_options_for_years_from_filesystem() -> Vec<String> {
    let paths = fs::read_dir("./src/2025").unwrap();

    let mut path_output: Vec<String> = Vec::new();

    for path in paths {
        let unwrapped_path = path.unwrap().path();

        if unwrapped_path.is_dir() {
            path_output.push(unwrapped_path.display().to_string());
        }
    }

    return path_output;
}

fn run_day_of_choice(choice: String) {
    let day_to_run = choice.replace("./src/2025/", "");
    let day_to_run = day_to_run.replace("/", "");

    if day_to_run == "day_1" {
        days_2025::day_1::solution();
    }

    if day_to_run == "day_2" {
        days_2025::day_2::solution();
    }

    if day_to_run == "day_3" {
        days_2025::day_3::solution();
    }

    if day_to_run == "day_4" {
        days_2025::day_4::solution();
    }

    if day_to_run == "day_5" {
        days_2025::day_5::solution();
    }

    if day_to_run == "day_6" {
        days_2025::day_6::solution();
    }

    if day_to_run == "day_7" {
        days_2025::day_7::solution();
    }

    if day_to_run == "day_8" {
        days_2025::day_8::solution();
    }

    if day_to_run == "day_9" {
        days_2025::day_9::solution();
    }
}

fn main() {
    let mut options = fetch_options_for_years_from_filesystem();

    options.sort();
    options.reverse();

    let ans = Select::new("Which day do you want to run?", options).prompt();

    match ans {
        Ok(choice) => run_day_of_choice(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}
