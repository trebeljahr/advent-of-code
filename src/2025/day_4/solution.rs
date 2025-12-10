pub fn solution() {
    let lines = crate::utils::file_reader::lines_from_file("./src/2025/day_4/paper.txt");

    // println!("Lines: {:?}", lines);
    let sum = find_neighbours(lines, 0);
    println!("Final Sum: {}", sum);
}

fn find_neighbours(mut lines: Vec<String>, mut sum: i32) -> i32 {
    let mut indices_to_remove: Vec<(usize, usize)> = vec![];

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

                indices_to_remove.push((y, x));
            }
        }
    }

    if (indices_to_remove.len() > 0) {
        for (y, x) in indices_to_remove {
            lines[y].replace_range(x..=x, ".");
        }

        return find_neighbours(lines, sum);
    }

    return sum;
}
