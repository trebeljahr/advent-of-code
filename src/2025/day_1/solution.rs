use std::{
    fs::File,
    io::{BufReader, prelude::*},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn solution() {
    let lines = lines_from_file("./src/2025/day_1/password-rotation.txt");

    let mut number: i32 = 50;
    let mut count = 0;

    for line in lines {
        print!("\n{:?}:", line);

        let first_char = &line[0..1];
        let rest = &line[1..];
        let num_rotations = rest.parse::<i32>().unwrap();

        if (first_char == "L") {
            for _ in 0..num_rotations {
                number -= 1;

                if (number < 0) {
                    let difference = 0 - number;
                    number = 99 - difference + 1;
                }
            }
            print!("{} ", number);
        } else if (first_char == "R") {
            for _ in 0..num_rotations {
                number += 1;

                if (number > 99) {
                    let difference = number - 99;
                    number = 0 + difference - 1;
                }
            }
            print!("{} ", number);
        }

        if (number == 0) {
            count += 1;
        }
    }

    print!("\nCount of 0s: {}", count);
}
