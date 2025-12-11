pub fn solution() {
    let mut lines =
        crate::utils::file_reader::lines_from_file("./src/2025/day_7/teleporter-example.txt");
    println!("Lines: {:?}", lines);

    let beam_idx = lines[0].find("S").unwrap();
    let mut beam_positions: Vec<usize> = vec![beam_idx];

    let mut count = 0;

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
                    chars[j - 1] = '|';
                    beam_positions.push(j - 1);
                }

                if (j + 1 < lines[i].len()) {
                    chars[j + 1] = '|';
                    beam_positions.push(j + 1)
                };

                beam_positions.retain(|pos| *pos != j)
            }

            lines[i] = chars.into_iter().collect();
        }
    }

    println!("Answer: {:?}", count);

    // println!("Lines: \n{}", lines.join("\n").replace("\\n", "\n"));
}
