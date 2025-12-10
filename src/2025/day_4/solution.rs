pub fn solution() {
    let lines = crate::utils::file_reader::lines_from_file("./src/2025/day_4/paper.txt");

    // println!("Lines: {:?}", lines);
    find_neighbours(lines);
}

fn find_neighbours(lines: Vec<String>) {
    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '@' {
                continue;
            }

            let mut neighbour_count = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }

                    let ny = y as isize + dy;
                    let nx = x as isize + dx;

                    if ny < 0 || nx < 0 {
                        continue;
                    }

                    let ny = ny as usize;
                    let nx = nx as usize;

                    if ny >= lines.len() || nx >= lines[ny].len() {
                        continue;
                    }

                    if lines[ny].chars().nth(nx).unwrap() == '@' {
                        neighbour_count += 1;
                    }
                }
            }

            if (neighbour_count < 4) {
                sum += 1;
            }
        }
    }

    println!("Sum: {}", sum);
}
