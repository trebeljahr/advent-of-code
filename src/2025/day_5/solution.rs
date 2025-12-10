struct Range {
    min: i64,
    max: i64,
}

pub fn solution() {
    let lines = crate::utils::file_reader::lines_from_file("./src/2025/day_5/ingredients.txt");
    // println!("Lines: {:?}", lines);

    let mut iter = lines.split(|a| a == "");
    let ranges = iter.next().unwrap().iter().map(|line| {
        let parts: Vec<&str> = line.split("-").collect();
        return Range {
            min: parts[0].parse::<i64>().unwrap(),
            max: parts[1].parse::<i64>().unwrap(),
        };
    });

    let ids = iter.next().unwrap().iter().map(|line| line.parse::<i64>());

    let mut num_valid = 0;

    for id in ids {
        let id = id.unwrap();
        let mut valid = false;
        for range in ranges.clone() {
            if id >= range.min && id <= range.max {
                valid = true;
                num_valid += 1;
                break;
            }
        }

        if valid {
            println!("ID {} is valid", id);
        } else {
            println!("ID {} is NOT valid", id);
        }
    }

    println!("Number of valid IDs: {}", num_valid);
}

// pub fn parse_limits
