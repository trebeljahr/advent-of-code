use crate::utils::file_reader::lines_from_file;

pub fn solution() {
    let lines = lines_from_file("./src/2025/day_2/invalid-ids.txt");

    let mut sum = 0;

    for line in lines {
        println!("{:?}", line);

        let splits = line.split(",").collect::<Vec<&str>>();
        for part in splits {
            let range_split = part.split("-").collect::<Vec<&str>>();

            let patterns = create_duplicate_pattern(range_split.clone());
            println!("Patterns: {:?}", patterns);

            patterns
                .iter()
                .map(|p| p.parse::<i64>().unwrap())
                .for_each(|n| sum += n);

            println!("Sum of patterns: {}", sum);
        }
    }
}

fn create_duplicate_pattern(range_split: Vec<&str>) -> Vec<String> {
    let max_pattern_len = range_split[1].len();

    let start = range_split[0].parse::<i64>().unwrap();
    let end = range_split[1].parse::<i64>().unwrap();

    println!("\nStart: {}, End: {}", start, end);

    let mut patterns: Vec<String> = Vec::new();
    let mut filtered_patterns: Vec<String> = Vec::new();

    for i in 1..=9
        .to_string()
        .repeat(max_pattern_len / 2)
        .parse::<i64>()
        .unwrap()
    {
        let pattern = format!("{}", i.to_string().repeat(2));
        patterns.push(pattern);
    }

    for pattern in patterns {
        let pattern_num = pattern.parse::<i64>().unwrap();
        if (pattern_num < start || pattern_num > end) {
            continue;
        }

        filtered_patterns.push(pattern);
    }

    return filtered_patterns;
}
