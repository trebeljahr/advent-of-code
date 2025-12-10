pub fn solution() {
    let lines = crate::utils::file_reader::lines_from_file("./src/2025/day_3/joltage-input.txt");

    let mut total = 0;
    for line in lines {
        println!("\n{:?}", line);
        let num = find_max_joltage_num(line);
        total += num
    }

    println!("\nTotal joltage number: {}", total);
}

#[derive(Clone, Debug)]
struct MaxJoltageNum {
    i: usize,
    value: i32,
}

fn find_max_joltage_num(str: String) -> i64 {
    let mut max_nums = vec![MaxJoltageNum { i: 0, value: -1 }; 12];

    for i in 0..max_nums.len() {
        let last_i = if i == 0 { 0 } else { max_nums[i - 1].i + 1 };
        for j in last_i..str.len() {
            let c = str.chars().nth(j).unwrap();

            let num: i32 = c.to_string().parse::<i32>().unwrap();
            let max_i = max_nums.len() - i - 1;

            if (num > max_nums[i].value && j < str.len() - max_i) {
                max_nums[i].i = j;
                max_nums[i].value = num;
            }
        }
    }

    let num = max_nums
        .iter()
        .map(|m| m.value.to_string())
        .collect::<Vec<String>>()
        .join("")
        .as_str()
        .parse::<i64>()
        .unwrap();

    println!("Max joltage numbers: {:?}", num);

    return num;
}
