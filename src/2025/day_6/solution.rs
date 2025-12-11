pub fn solution() {
    let lines = crate::utils::file_reader::lines_from_file("./src/2025/day_6/math.txt");
    println!("Lines: {:?}", lines);

    let split_idxs = find_white_space_columns_in_input(lines.clone());
    println!("White space columns: {:?}", split_idxs);

    let mut nums_by_col: Vec<Vec<i64>> = vec![vec![]; split_idxs.len()];
    let mut nums_by_col_part_2: Vec<Vec<String>> = vec![vec![]; split_idxs.len()];

    let mut operators_by_col: Vec<Vec<String>> = vec![vec![]; split_idxs.len()];

    for j in 0..lines.len() {
        for i in 0..split_idxs.len() {
            let start = if i > 0 { split_idxs[i - 1] } else { 0 };
            let end = split_idxs[i];

            let sub_slice = &lines[j][start..end];

            if sub_slice.contains(&['+', '*']) {
                let parsed_result = sub_slice.trim().to_string();
                operators_by_col[i].push(parsed_result.clone());
            } else {
                let parsed_result = sub_slice.trim().parse::<i64>().unwrap();
                nums_by_col[i].push(parsed_result);

                nums_by_col_part_2[i].push(sub_slice.to_string());
            }
        }
    }

    let mut sum_of_results = 0;
    let mut sum_of_results_2 = 0;

    for i in 0..nums_by_col_part_2.len() {
        let col = &nums_by_col_part_2[i];

        let mut out_nums: Vec<String> = vec!["".to_string(); col[0].len()];
        for num in col {
            for digit_i in (0..num.len()).rev() {
                let digit = &num.chars().nth(digit_i).unwrap().to_string();
                out_nums[digit_i] += digit;
            }
        }

        let out: Vec<i64> = out_nums
            .iter()
            .filter(|s| s.contains(|c: char| c.is_alphanumeric()))
            .map(|n| n.trim().parse::<i64>().unwrap())
            .collect();

        let operator = &operators_by_col[i][0];
        let mut result = if operator == "+" { 0 } else { 1 };

        out.iter().for_each(|n| {
            if (operator == "+") {
                result += n;
            } else if (operator == "*") {
                result *= n;
            }
        });

        sum_of_results_2 += result;
    }

    for i in 0..operators_by_col.len() {
        let operator = &operators_by_col[i][0];
        let nums = &nums_by_col[i];

        let mut result = if operator == "+" { 0 } else { 1 };

        nums.iter().for_each(|n| {
            if (operator == "+") {
                result += n;
            } else if (operator == "*") {
                result *= n;
            }
        });

        sum_of_results += result;
    }

    println!("Sum of Results for 1st Star: {:?}", sum_of_results);
    println!("Sum of Results for 2nd Star: {:?}", sum_of_results_2);
}

pub fn find_white_space_columns_in_input(lines: Vec<String>) -> Vec<usize> {
    let mut white_space_columns: Vec<usize> = vec![];

    let num_columns = lines[0].len();

    for col in 0..num_columns {
        let mut is_white_space_column = true;

        for line in &lines {
            if line.chars().nth(col).unwrap() != ' ' {
                is_white_space_column = false;
                break;
            }
        }

        if is_white_space_column {
            white_space_columns.push(col);
        }
    }

    white_space_columns.push(num_columns);

    return white_space_columns;
}
