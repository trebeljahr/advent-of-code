#[derive(Debug, Clone)]
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

    // println!("Ranges: {:?}", ranges.clone().collect::<Vec<Range>>());

    let mut num_valid = 0;

    for id in ids {
        let id = id.unwrap();
        for range in ranges.clone() {
            if id >= range.min && id <= range.max {
                num_valid += 1;
                break;
            }
        }
    }

    println!("Number of valid IDs: {}", num_valid);

    let merged_ranges = merge_overlapping_ranges(ranges.collect());

    let sum: i64 = merged_ranges
        .into_iter()
        .map(|range| range.max - range.min + 1)
        .sum();

    println!("Sum: {}", sum)
}

fn merge_overlapping_ranges(ranges: Vec<Range>) -> Vec<Range> {
    let mut merged_ranges: Vec<Range> = ranges.clone();

    loop {
        let mut did_merge = false;

        let clone = merged_ranges.clone();

        for i in 0..merged_ranges.len() {
            for j in 0..merged_ranges.len() {
                let range = &clone[i];
                let other_range = &clone[j];

                if i == j {
                    continue;
                }

                if (range.min <= other_range.max && range.max >= other_range.min) {
                    let new_min = std::cmp::min(range.min, other_range.min);
                    let new_max = std::cmp::max(range.max, other_range.max);

                    merged_ranges.retain(|r| !(r.min == range.min && r.max == range.max));
                    merged_ranges
                        .retain(|r| !(r.min == other_range.min && r.max == other_range.max));

                    merged_ranges.push(Range {
                        min: new_min,
                        max: new_max,
                    });

                    did_merge = true;
                    break;
                }
            }
        }

        if !did_merge {
            break;
        }
    }

    // println!("Merged Ranges: {:?}", merged_ranges);

    return merged_ranges;
}
