pub fn solution() {
    let lines = crate::utils::file_reader::lines_from_file("./src/2025/day_7/teleporter.txt");
    println!("Lines: {:?}", lines);

    let beam_idx = lines[0].find("S").unwrap();
    let mut beam_positions: Vec<usize> = vec![beam_idx];

    let mut count = 0;

    for i in 1..lines.len() {
        let line = &lines[i];

        for j in 0..line.len() {
            let char = line.chars().nth(j).unwrap();
            if (char == '.' && beam_positions.contains(&j)) {
                continue;
            }

            if (char == '^' && beam_positions.contains(&j)) {
                count += 1;
                if (j != 0) {
                    beam_positions.push(j - 1);
                }

                if (j + 1 < line.len()) {
                    beam_positions.push(j + 1)
                };

                beam_positions.retain(|pos| *pos != j)
            }
        }
    }

    println!("Answer: {:?}", count);
}
