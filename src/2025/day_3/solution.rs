pub fn solution() {
    let lines = crate::utils::file_reader::lines_from_file("./src/2025/day_3/joltage-input.txt");

    let mut total = 0;
    for line in lines {
        println!("\n{:?}", line);
        let max_nums = find_max_joltage_nums(line);
        let num = max_nums[0].value * 10 + max_nums[1].value;
        total += num
    }

    println!("\nTotal joltage number: {}", total);
}

struct MaxJoltageNum {
    i: usize,
    value: i32,
}

fn find_max_joltage_nums(str: String) -> Vec<MaxJoltageNum> {
    let mut max_num_1 = MaxJoltageNum { i: 0, value: -1 };
    let mut max_num_2 = MaxJoltageNum { i: 0, value: -1 };

    for i in 0..str.len() {
        let c = str.chars().nth(i).unwrap();

        let num: i32 = c.to_string().parse::<i32>().unwrap();

        if (num > max_num_1.value && i != str.len() - 1) {
            max_num_1.i = i;
            max_num_1.value = num;
        }
    }

    for i in 0..str.len() {
        let c = str.chars().nth(i).unwrap();
        let num: i32 = c.to_string().parse::<i32>().unwrap();

        if (num > max_num_2.value && i != max_num_1.i && i > max_num_1.i) {
            max_num_2.i = i;
            max_num_2.value = num;
        }
    }

    let mut max_nums = vec![max_num_1, max_num_2];
    max_nums.sort_by(|a, b| a.i.cmp(&b.i));

    println!(
        "Max joltage numbers: {:?}",
        max_nums.iter().map(|m| m.value).collect::<Vec<i32>>()
    );

    return max_nums;
}
