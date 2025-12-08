use std::fs;

pub fn day_6() {
    let file_path = "inputs/day_6_input.txt";
    //let file_path = "inputs/day_6_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");

    let lines: Vec<&str> = context.lines().collect();

    let number_grid: Vec<Vec<i64>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let symb_row: Vec<char> = lines[lines.len() - 1]
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let grid_height = number_grid.len();
    let grid_width = number_grid[0].len();
    let mut result_row: Vec<i64> = Vec::new();
    let mut result: i64;

    for i in 0..grid_width {
        result = 0;
        for j in 0..grid_height {
            if symb_row[i] == '+' {
                result += number_grid[j][i];
            } else {
                if result == 0 {
                    result += 1;
                }
                result *= number_grid[j][i];
            }
        }
        result_row.push(result);
    }

    println!("{}", result_row.iter().sum::<i64>());
}
