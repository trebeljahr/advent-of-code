pub fn solution() {
    let mut lines = crate::utils::file_reader::lines_from_file("./src/2025/day_7/teleporter.txt");

    let beam_idx = lines[0].find("S").unwrap();
    let mut beam_positions: Vec<usize> = vec![beam_idx];

    let mut count = 0;

    let mut counts: Vec<i64> = vec![0; lines[0].len()];
    counts[beam_idx] = 1;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let mut chars: Vec<char> = lines[i].chars().collect();

            let char = lines[i].chars().nth(j).unwrap();
            if (char == '.' && beam_positions.contains(&j)) {
                chars[j] = '|';
            }

            if (char == '^' && beam_positions.contains(&j)) {
                count += 1;
                if (j != 0) {
                    counts[j - 1] += counts[j];

                    chars[j - 1] = '|';
                    beam_positions.push(j - 1);
                }

                if (j + 1 < lines[i].len()) {
                    counts[j + 1] += counts[j];

                    chars[j + 1] = '|';
                    beam_positions.push(j + 1)
                };

                counts[j] = 0;

                beam_positions.retain(|pos| *pos != j)
            }

            lines[i] = chars.into_iter().collect();
        }
    }

    println!("Answer: {:?}", count);
    println!("Answer Part 2: {:?}", counts.iter().sum::<i64>());
}
