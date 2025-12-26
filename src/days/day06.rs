use std::fs;

pub fn day_6() {
    let file_path = "inputs/day_6_input.txt";
    //let file_path = "inputs/day_6_input_test.txt";
    let context = fs::read_to_string(file_path).expect("Error reading file");

    let lines: Vec<&str> = context.lines().collect();

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    
    let number_grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            while chars.len() < max_width {
                chars.push(' ');
            }
            chars
        })
        .collect();

    let grid_height = number_grid.len();
    let grid_width = max_width;
    
    let mut total: i128 = 0;

    let mut current_block_numbers: Vec<i128> = Vec::new();
    let mut current_operator = ' ';

    for col in (0..grid_width).rev() {
        let is_separator = (0..grid_height).all(|row| number_grid[row][col] == ' ');

        if is_separator {
            if !current_block_numbers.is_empty() {
                total += solve_block(&current_block_numbers, current_operator);
                current_block_numbers.clear();
                current_operator = ' ';
            }
        } else {
            let bottom_char = number_grid[grid_height - 1][col];
            if bottom_char == '+' || bottom_char == '*' {
                current_operator = bottom_char;
            }

            let mut num_str = String::new();
            for row in 0..grid_height - 1 {
                let c = number_grid[row][col];
                if c.is_digit(10) {
                    num_str.push(c);
                }
            }

            if !num_str.is_empty() {
                let number: i128 = num_str.parse().unwrap();
                current_block_numbers.push(number);
            }
        }

    }
    if !current_block_numbers.is_empty() {
            total += solve_block(&current_block_numbers, current_operator);
        }
    println!("Total: {}", total);
}

fn solve_block(numbers: &[i128], operator: char) -> i128 {
    if numbers.is_empty() {
        return 0;
    }
    
    match operator {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => 0, // Should not happen based on problem description
    }
}

    

