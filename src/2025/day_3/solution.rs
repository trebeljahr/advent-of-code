pub fn solution() {
    let lines =
        crate::utils::file_reader::lines_from_file("./src/2025/day_3/joltage-input-example.txt");
    for line in lines {
        println!("\n{:?}", line);
        find_max_joltage_num(line);
    }
}

fn find_max_joltage_num(str: String) {
    let mut max_nums = [0, 0];

    for c in str.chars() {
        let num: i32 = c.to_string().parse::<i32>().unwrap();

        for i in 0..max_nums.len() {
            if num > max_nums[i] {
                max_nums[i] = num;
                break;
            }
        }
    }

    max_nums.sort();
    max_nums.reverse();

    println!("Max joltage number: {:?}", max_nums);
}
